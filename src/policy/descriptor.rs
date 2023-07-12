use crate::{miniscript, Error, Policy};
use bitcoin_hashes::{hash160, ripemd160, sha256, Hash};
use elements::schnorr::{TapTweak, XOnlyPublicKey};
use elements::taproot::{
    ControlBlock, LeafVersion, TapBranchHash, TapLeafHash, TaprootBuilder, TaprootMerkleBranch,
    TaprootSpendInfo,
};
use elements::{bitcoin, secp256k1_zkp};
use miniscript::descriptor::{ConversionError, DescriptorSecretKey, KeyMap};
use miniscript::Error as MSError;
use miniscript::{
    hash256, translate_hash_clone, DefiniteDescriptorKey, DescriptorPublicKey, ForEachKey,
    MiniscriptKey, Satisfier, ToPublicKey, Translator,
};
use std::collections::HashMap;
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

impl UnspendableKey for DescriptorPublicKey {
    fn unspendable() -> Self {
        Self::Single(miniscript::descriptor::SinglePub {
            origin: None,
            key: miniscript::descriptor::SinglePubKey::XOnly(XOnlyPublicKey::unspendable()),
        })
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

    fn translate_pk<T, Q, E>(&self, translator: &mut T) -> Result<Descriptor<Q>, E>
    where
        T: Translator<Pk, Q, E>,
        Q: MiniscriptKey,
    {
        let internal_key = translator.pk(self.internal_key())?;
        let policy = self.policy.translate(translator)?;
        Ok(Descriptor::new(internal_key, policy))
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
    pub fn get_satisfaction<S: Satisfier<Pk>>(
        &self,
        satisfier: S,
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

impl<Pk: MiniscriptKey> ForEachKey<Pk> for Descriptor<Pk> {
    fn for_each_key<'a, F: FnMut(&'a Pk) -> bool>(&'a self, mut pred: F) -> bool
    where
        Pk: 'a,
    {
        pred(&self.internal_key) && self.policy.for_each_key(pred)
    }
}

impl Descriptor<DescriptorPublicKey> {
    /// Whether or not the descriptor has any wildcards i.e. `/*`.
    pub fn has_wildcard(&self) -> bool {
        self.for_any_key(|key| key.has_wildcard())
    }

    /// Replaces all wildcards (i.e. `/*`) in the descriptor with a particular derivation index,
    /// turning it into a *definite* descriptor.
    ///
    /// # Errors
    /// - If index ≥ 2^31
    pub fn at_derivation_index(
        &self,
        index: u32,
    ) -> Result<Descriptor<DefiniteDescriptorKey>, ConversionError> {
        struct Derivator(u32);

        impl Translator<DescriptorPublicKey, DefiniteDescriptorKey, ConversionError> for Derivator {
            fn pk(
                &mut self,
                pk: &DescriptorPublicKey,
            ) -> Result<DefiniteDescriptorKey, ConversionError> {
                pk.clone().at_derivation_index(self.0)
            }

            translate_hash_clone!(DescriptorPublicKey, DescriptorPublicKey, ConversionError);
        }

        self.translate_pk(&mut Derivator(index))
    }

    /// Parse a descriptor that may contain secret keys
    ///
    /// Internally turns every secret key found into the corresponding public key and then returns a
    /// a descriptor that only contains public keys and a map to lookup the secret key given a public key.
    pub fn parse_descriptor<C: secp256k1_zkp::Signing>(
        secp: &secp256k1_zkp::Secp256k1<C>,
        s: &str,
    ) -> Result<(Descriptor<DescriptorPublicKey>, KeyMap), MSError> {
        fn parse_key<C: secp256k1_zkp::Signing>(
            s: &str,
            key_map: &mut KeyMap,
            secp: &secp256k1_zkp::Secp256k1<C>,
        ) -> Result<DescriptorPublicKey, MSError> {
            let (public_key, secret_key) = match DescriptorSecretKey::from_str(s) {
                Ok(sk) => (
                    sk.to_public(secp)
                        .map_err(|e| MSError::Unexpected(e.to_string()))?,
                    Some(sk),
                ),
                Err(_) => (
                    DescriptorPublicKey::from_str(s)
                        .map_err(|e| MSError::Unexpected(e.to_string()))?,
                    None,
                ),
            };

            if let Some(secret_key) = secret_key {
                key_map.insert(public_key.clone(), secret_key);
            }

            Ok(public_key)
        }

        let mut keymap_pk = KeyMapWrapper(HashMap::new(), secp);

        struct KeyMapWrapper<'a, C: secp256k1_zkp::Signing>(
            KeyMap,
            &'a secp256k1_zkp::Secp256k1<C>,
        );

        impl<'a, C: secp256k1_zkp::Signing> Translator<String, DescriptorPublicKey, MSError>
            for KeyMapWrapper<'a, C>
        {
            fn pk(&mut self, pk: &String) -> Result<DescriptorPublicKey, MSError> {
                parse_key(pk, &mut self.0, self.1)
            }

            fn sha256(&mut self, sha256: &String) -> Result<sha256::Hash, MSError> {
                let hash = sha256::Hash::from_str(sha256)
                    .map_err(|e| MSError::Unexpected(e.to_string()))?;
                Ok(hash)
            }

            fn hash256(&mut self, hash256: &String) -> Result<hash256::Hash, MSError> {
                let hash = hash256::Hash::from_str(hash256)
                    .map_err(|e| MSError::Unexpected(e.to_string()))?;
                Ok(hash)
            }

            fn ripemd160(&mut self, ripemd160: &String) -> Result<ripemd160::Hash, MSError> {
                let hash = ripemd160::Hash::from_str(ripemd160)
                    .map_err(|e| MSError::Unexpected(e.to_string()))?;
                Ok(hash)
            }

            fn hash160(&mut self, hash160: &String) -> Result<hash160::Hash, MSError> {
                let hash = hash160::Hash::from_str(hash160)
                    .map_err(|e| MSError::Unexpected(e.to_string()))?;
                Ok(hash)
            }
        }

        let descriptor = Descriptor::<String>::from_str(s)?;
        let descriptor = descriptor
            .translate_pk(&mut keymap_pk)
            .map_err(|e| MSError::Unexpected(e.to_string()))?;

        Ok((descriptor, keymap_pk.0))
    }

    /// Serialize a descriptor to string with its secret keys
    pub fn to_string_with_secret(&self, key_map: &KeyMap) -> String {
        struct KeyMapLookUp<'a>(&'a KeyMap);

        impl<'a> Translator<DescriptorPublicKey, String, ()> for KeyMapLookUp<'a> {
            fn pk(&mut self, pk: &DescriptorPublicKey) -> Result<String, ()> {
                key_to_string(pk, self.0)
            }

            fn sha256(&mut self, sha256: &sha256::Hash) -> Result<String, ()> {
                Ok(sha256.to_string())
            }

            fn hash256(&mut self, hash256: &hash256::Hash) -> Result<String, ()> {
                Ok(hash256.to_string())
            }

            fn ripemd160(&mut self, ripemd160: &ripemd160::Hash) -> Result<String, ()> {
                Ok(ripemd160.to_string())
            }

            fn hash160(&mut self, hash160: &hash160::Hash) -> Result<String, ()> {
                Ok(hash160.to_string())
            }
        }

        fn key_to_string(pk: &DescriptorPublicKey, key_map: &KeyMap) -> Result<String, ()> {
            Ok(match key_map.get(pk) {
                Some(secret) => secret.to_string(),
                None => pk.to_string(),
            })
        }

        let descriptor = self
            .translate_pk(&mut KeyMapLookUp(key_map))
            .expect("Translation to string cannot fail");

        descriptor.to_string()
    }
}

impl Descriptor<DefiniteDescriptorKey> {
    /// Convert all the public keys in the descriptor to [`bitcoin::PublicKey`] by deriving them or
    /// otherwise converting them. All [`secp256k1_zkp::XOnlyPublicKey`]s are converted to by adding a
    /// default(0x02) y-coordinate.
    ///
    /// # Errors
    ///
    /// This function will return an error if hardened derivation is attempted.
    pub fn derived_descriptor<C: secp256k1_zkp::Verification>(
        &self,
        secp: &secp256k1_zkp::Secp256k1<C>,
    ) -> Result<Descriptor<bitcoin::PublicKey>, ConversionError> {
        struct Derivator<'a, C: secp256k1_zkp::Verification>(&'a secp256k1_zkp::Secp256k1<C>);

        impl<'a, C: secp256k1_zkp::Verification>
            Translator<DefiniteDescriptorKey, bitcoin::PublicKey, ConversionError>
            for Derivator<'a, C>
        {
            fn pk(
                &mut self,
                pk: &DefiniteDescriptorKey,
            ) -> Result<bitcoin::PublicKey, ConversionError> {
                pk.derive_public_key(self.0)
            }

            translate_hash_clone!(DefiniteDescriptorKey, bitcoin::PublicKey, ConversionError);
        }

        let derived = self.translate_pk(&mut Derivator(secp))?;
        Ok(derived)
    }
}

impl<Pk: MiniscriptKey> fmt::Display for Descriptor<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sim({},{})", self.internal_key, self.policy)
    }
}

impl<Pk> FromStr for Descriptor<Pk>
where
    Pk: MiniscriptKey + FromStr,
    Pk::Sha256: FromStr,
    <Pk as FromStr>::Err: ToString,
    <Pk::Sha256 as FromStr>::Err: ToString,
{
    type Err = MSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (internal_key, policy) = s
            .strip_prefix("sim(")
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| {
                let mut split = s.splitn(2, ',');
                let x = split.next()?;
                let y = split.next()?;
                Some((x, y))
            })
            .ok_or(MSError::BadDescriptor("bad taproot descriptor".to_string()))?;
        let internal_key = <Pk as FromStr>::from_str(internal_key)
            .map_err(|e| MSError::BadDescriptor(e.to_string()))?;
        let policy = Policy::from_str(policy)?;

        Ok(Self::new(internal_key, policy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string_to_string() {
        let original = "sim(d6889cb081036e0faefa3a35157ad71086b123b2b144b649798b494c300a961d,pk(e0dfe2300b0dd746a3f8674dfd4525623639042569d829c7f0eed9602d263e6f))";
        let descriptor = Descriptor::<XOnlyPublicKey>::from_str(original).expect("from_str");
        let display = descriptor.to_string();
        assert_eq!(original, display);
    }

    #[test]
    fn derived_descriptor() {
        let key = "[78412e3a/44'/0'/0']xpub6ERApfZwUNrhLCkDtcHTcxd75RbzS1ed54G1LkBUHQVHQKqhMkhgbmJbZRkrgZw4koxb5JaHWkY4ALHY2grBGRjaDMzQLcgJvLJuZZvRcEL/1/*";
        let parent_key = DescriptorPublicKey::from_str(key).expect("constant key");
        let policy = Policy::Key(parent_key.clone());
        // Use the same xpub for internal key and for policy
        // Unrealistic, but sufficient for testing
        let parent_descriptor = Descriptor::new(parent_key.clone(), policy);

        for index in 0..10 {
            let child_descriptor = parent_descriptor
                .at_derivation_index(index)
                .expect("derive descriptor");
            let child_key = parent_key
                .clone()
                .at_derivation_index(index)
                .expect("derive key");

            assert_eq!(child_key, child_descriptor.internal_key);

            if let Policy::Key(key) = child_descriptor.policy {
                assert_eq!(child_key, key);
            }
        }
    }
}
