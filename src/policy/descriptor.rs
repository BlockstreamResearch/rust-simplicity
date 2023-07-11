use crate::policy::satisfy::PolicySatisfier;
use crate::{miniscript, Error, Policy};
use bitcoin_hashes::Hash;
use elements::schnorr::{TapTweak, XOnlyPublicKey};
use elements::secp256k1_zkp;
use elements::taproot::{
    ControlBlock, LeafVersion, TapBranchHash, TapLeafHash, TaprootBuilder, TaprootMerkleBranch,
    TaprootSpendInfo,
};
use elements_miniscript::{MiniscriptKey, ToPublicKey};
use std::fmt;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

pub trait UnspendableKey: MiniscriptKey {
    fn unspendable() -> Self;
}

impl UnspendableKey for XOnlyPublicKey {
    fn unspendable() -> Self {
        XOnlyPublicKey::from_slice(&UNSPENDABLE_PUBLIC_KEY).expect("unspendable pubkey is valid")
    }
}

/// Bytes of x-only public key whose discrete logarithm (secret key) is unknown
///
/// Taken from BIP 341
const UNSPENDABLE_PUBLIC_KEY: [u8; 32] = [
    0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a, 0x5e,
    0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80, 0x3a, 0xc0,
];
/// Version of Simplicity tap leaves
pub const TAPROOT_LEAF_SIMPLICITY: u8 = 0xbe;

/// Return the version of Simplicity tap leaves
pub fn leaf_version() -> LeafVersion {
    LeafVersion::from_u8(TAPROOT_LEAF_SIMPLICITY).expect("constant leaf version")
}

// TODO: Support multiple tap leaves
/// Descriptor of Simplicity outputs.
///
/// Simplicity is embedded in Taproot outputs as tap leaves with version 0xbe.
///
/// The tap tree can include many leaves, be it version 0xc0 (standard Taproot) or 0xbe (Simplicity).
/// Currently only trees with a single Simplicity leaf are supported.
///
/// On "top" of the tap tree sits the internal key which can be used as an alternative spending path
/// if all parties that own the output agree to sign.
///
/// The internal key can be a normal public key (p2pk), a MuSig aggregate public key (multisig)
/// or an unspendable public key in case this feature is undesirable.
#[derive(Debug)]
pub struct Descriptor<Pk: MiniscriptKey> {
    internal_key: Pk,
    policy: Policy<Pk>,
    spend_info: Mutex<Option<Arc<TaprootSpendInfo>>>,
}

impl<Pk: MiniscriptKey> Clone for Descriptor<Pk> {
    fn clone(&self) -> Self {
        Self {
            internal_key: self.internal_key.clone(),
            policy: self.policy.clone(),
            // New mutex so clones don't block each other
            // Cloning the contained spending info is cheap
            spend_info: Mutex::new(
                self.spend_info
                    .lock()
                    .expect("Lock poisoned")
                    .as_ref()
                    .map(Arc::clone),
            ),
        }
    }
}

impl<Pk: MiniscriptKey> PartialEq for Descriptor<Pk> {
    fn eq(&self, other: &Self) -> bool {
        self.internal_key == other.internal_key && self.policy == other.policy
    }
}

impl<Pk: MiniscriptKey> Eq for Descriptor<Pk> {}

impl<Pk: MiniscriptKey> Descriptor<Pk> {
    /// Create a new descriptor from the given internal key and
    /// policy which will become a single tap leaf
    pub fn new(internal_key: Pk, policy: Policy<Pk>) -> Self {
        Self {
            internal_key,
            policy,
            spend_info: Mutex::new(None),
        }
    }

    /// Create a new descriptor from the given policy which will become a single tap leaf
    ///
    /// The internal key is set to a constant that is provably not spendable
    pub fn single_leaf(policy: Policy<Pk>) -> Self
    where
        Pk: UnspendableKey,
    {
        let internal_key = Pk::unspendable();
        Self::new(internal_key, policy)
    }

    /// Return the internal key
    pub fn internal_key(&self) -> &Pk {
        &self.internal_key
    }
}

impl<Pk: ToPublicKey> Descriptor<Pk> {
    /// Return the spend data
    pub fn spend_info(&self) -> Arc<TaprootSpendInfo> {
        // Return the spending info if it is already cached
        // Panic if lock is poisoned (another thread with the lock panicked)
        let read_lock = self.spend_info.lock().expect("Lock poisoned");
        if let Some(ref spend_info) = *read_lock {
            return Arc::clone(spend_info);
        }
        drop(read_lock);

        let (script, version) = self.leaf();
        let builder = TaprootBuilder::new()
            .add_leaf_with_ver(0, script, version)
            .expect("constant leaf");
        let secp = secp256k1_zkp::Secp256k1::verification_only();
        let data = builder
            .finalize(&secp, self.internal_key.to_x_only_pubkey())
            .expect("constant tree");

        // Cache spending info
        let spend_info = Arc::new(data);
        *self.spend_info.lock().expect("Lock poisoned") = Some(Arc::clone(&spend_info));
        spend_info
    }

    /// Return the script pubkey
    pub fn script_pubkey(&self) -> elements::Script {
        let output_key = self.spend_info().output_key();
        let builder = elements::script::Builder::new();
        builder
            .push_opcode(elements::opcodes::all::OP_PUSHNUM_1)
            .push_slice(&output_key.as_inner().serialize())
            .into_script()
    }

    /// Return the Elements address on the given network
    pub fn address(&self, params: &'static elements::AddressParams) -> elements::Address {
        let output_key = self.spend_info().output_key();
        elements::Address::p2tr_tweaked(output_key, None, params)
    }

    /// Return the single tap leaf
    pub fn leaf(&self) -> (elements::Script, LeafVersion) {
        let commit = self.policy.serialize_no_witness();
        let script = elements::Script::from(commit.cmr().as_ref().to_vec());
        let version = leaf_version();

        (script, version)
    }

    /// Return a satisfying non-malleable witness and script sig with minimum weight
    /// to spend an output controlled by the given descriptor if it is possible to satisfy
    /// given the `satisfier`
    pub fn get_satisfaction(
        &self,
        satisfier: &PolicySatisfier<Pk>,
    ) -> Result<(Vec<Vec<u8>>, elements::Script), Error> {
        let program = self.policy.satisfy(satisfier)?;

        // Uncomment code below for sanity check
        // that program successfully runs on Bit Machine
        // let mut mac = crate::exec::BitMachine::for_program(&program);
        // let _output = mac
        //     .exec(&program, &satisfier.env)
        //     .expect("sanity check");

        let program_and_witness_bytes = program.encode_to_vec();
        let cmr_bytes = Vec::from(program.cmr().as_ref());

        // Single leaf: leaf hash = merkle root
        let (script, leaf_version) = self.leaf();
        let leaf_hash = TapLeafHash::from_script(&script, leaf_version);
        let merkle_root = TapBranchHash::from_inner(leaf_hash.into_inner());
        // Single leaf: empty merkle inclusion proof
        let merkle_branch = TaprootMerkleBranch::from_inner(Vec::new()).expect("constant branch");

        let internal_key = self.internal_key().to_x_only_pubkey();
        let output_key_parity = internal_key
            .tap_tweak(secp256k1_zkp::SECP256K1, Some(merkle_root))
            .1;

        let control_block = ControlBlock {
            leaf_version,
            output_key_parity,
            internal_key,
            merkle_branch,
        };
        let witness = vec![
            program_and_witness_bytes,
            cmr_bytes,
            control_block.serialize(),
        ];
        let script_sig = elements::Script::new();

        Ok((witness, script_sig))
    }
}

impl<Pk: MiniscriptKey> fmt::Display for Descriptor<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.policy, f)
    }
}

impl<Pk> FromStr for Descriptor<Pk>
where
    Pk: UnspendableKey + ToPublicKey + FromStr,
    Pk::Sha256: FromStr,
    <Pk as FromStr>::Err: ToString,
    <Pk::Sha256 as FromStr>::Err: ToString,
{
    type Err = miniscript::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let policy = Policy::from_str(s)?;
        Ok(Self::single_leaf(policy))
    }
}
