// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket kanjalkar <sanket1729@gmail.com>
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

use crate::exec::BitMachine;
use crate::jet::JetFailed;
use crate::merkle::cmr::Cmr;
use bitcoin_hashes::{sha256, Hash, HashEngine};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use elements::confidential::{Asset, Nonce, Value};
use elements::taproot::ControlBlock;
use elements::{confidential, AssetIssuance, BlockHash};
use simplicity_sys::c_jets::c_env::CTxEnv;
use std::sync::Arc;

use super::c_env;

/// An Elements UTXO
// This is not a complete TxOut as it does not contain the nonce that
// is sent to the recipient.
pub struct ElementsUtxo {
    /// The 'scriptpubkey' (hash of Simplicity program)
    pub script_pubkey: elements::Script,
    /// The explicit or confidential asset
    pub asset: confidential::Asset,
    /// The explict or confidential value
    pub value: confidential::Value,
}

/// Environment for Elements Simplicity
///
/// # Note
/// The order of `utxos` must be same as of the order of inputs in the
/// transaction.
// FIXME:
// Ideally the `Arc<elements::Transaction>` would be a generic
// `AsRef<elements::Transaction>` or something, but this is an associated type
// for the `Extension` trait, and Rust will not let us have generic parameters
// on associated types. (We could possibly add a parameter to the Extension
// trait itself, but that would be messy and layer-violating.)
//
// Similar story if we tried to use a &'a elements::Transaction rather than
// an Arc: we'd have a lifetime parameter <'a> that would cause us trouble.
pub struct ElementsEnv {
    /// The CTxEnv struct
    pub(super) c_tx_env: CTxEnv,
    /// The elements transaction
    pub(super) tx: Arc<elements::Transaction>,
    /// The input utxo information corresponding to outpoint being spent.
    pub(super) utxos: Vec<ElementsUtxo>,
    /// the current index of the input
    pub(super) ix: u32,
    /// Commitment merkle root of the script being executed
    pub(super) script_cmr: Cmr,
    /// Control block used to spend this leaf script
    pub(super) control_block: ControlBlock,
    /// Optional Annex.
    pub(super) annex: Option<Vec<u8>>,
    /// Genesis block hash
    pub(super) genesis_hash: BlockHash,
}

impl ElementsEnv {
    pub fn new(
        tx: Arc<elements::Transaction>,
        utxos: Vec<ElementsUtxo>,
        ix: u32,
        script_cmr: Cmr,
        control_block: ControlBlock,
        annex: Option<Vec<u8>>,
        genesis_hash: BlockHash,
    ) -> Self {
        let c_tx = c_env::new_tx(&tx, &utxos);
        let c_tap_env = c_env::new_tap_env(&control_block, script_cmr);
        let c_tx_env = c_env::new_tx_env(c_tx, c_tap_env, genesis_hash, ix);
        ElementsEnv {
            c_tx_env,
            tx,
            utxos,
            ix,
            script_cmr,
            control_block,
            annex,
            genesis_hash,
        }
    }

    /// Obtains the FFI compatible CTxEnv from self
    pub fn c_tx_env(&self) -> &CTxEnv {
        &self.c_tx_env
    }

    /// Returns a reference to the control block of this [`ElementsEnv`].
    pub fn control_block(&self) -> &ControlBlock {
        &self.control_block
    }

    /// Returns the annex of this [`ElementsEnv`].
    pub fn annex(&self) -> Option<&Vec<u8>> {
        self.annex.as_ref()
    }

    /// Returns the genesis hash of this [`ElementsEnv`].
    pub fn genesis_hash(&self) -> BlockHash {
        self.genesis_hash
    }
}

/// Helper trait for writing various components of
/// Simplicity transactions (assets, values) into bit machine.
pub(super) trait SimplicityEncodable {
    // write the simplicity encoding of `self` on bitmachine
    // at the current write cursor.
    fn simplicity_encode(self, mac: &mut BitMachine) -> Result<(), JetFailed>;
}

// Write an confidential asset to write frame, advancing the cursor 258
// cells (or returning an error if the asset is null)
//
// The Simplicity representation of a confidential asset is:
//    ((is_explicit, is_odd), [u8; 32])
impl SimplicityEncodable for confidential::Asset {
    fn simplicity_encode(self, mac: &mut BitMachine) -> Result<(), JetFailed> {
        match self {
            Asset::Null => return Err(JetFailed),
            Asset::Explicit(asset_id) => {
                mac.write_bit(true);
                mac.skip(1);
                mac.write_bytes(asset_id.into_inner().as_ref());
            }
            // consensus rules state that asset must be 0x0a or 0x0b
            Asset::Confidential(generator) => {
                let bytes = generator.serialize();
                mac.write_bit(false); //not explicit
                mac.write_bit(bytes[0] == 0x0b);
                mac.write_bytes(&bytes[1..]);
            }
        }
        Ok(())
    }
}

// Write an confidential value to write frame, advancing the cursor 258
// cells (or returning an error if the value is null)
//
// The Simplicity representation of a confidential value is:
//     ((is_explicit, is_odd), [u8; 32])
impl SimplicityEncodable for confidential::Value {
    fn simplicity_encode(self, mac: &mut BitMachine) -> Result<(), JetFailed> {
        match self {
            Value::Null => return Err(JetFailed),
            Value::Explicit(data) => {
                mac.write_bit(true);
                mac.skip(1 + 256 - 64);
                mac.write_u64(data);
            }
            // consensus rules state that prefix value must be 0x08 or 0x09
            Value::Confidential(generator) => {
                let bytes = generator.serialize();
                mac.write_bit(false); //not explicit
                mac.write_bit(bytes[0] == 0x09);
                mac.write_bytes(&bytes[1..]);
            }
        }
        Ok(())
    }
}

// Write an confidential nonce to write frame, advancing the cursor 259 cells
//
// The Simplicity representation of a confidential nonce is:
//     ((is_not_null, is_explicit, is_odd), [u8; 32])
impl SimplicityEncodable for confidential::Nonce {
    fn simplicity_encode(self, mac: &mut BitMachine) -> Result<(), JetFailed> {
        // all paths should write 259 bits
        match self {
            Nonce::Null => {
                mac.write_bit(false);
                mac.skip(258);
            }
            Nonce::Explicit(data) => {
                mac.write_bit(true); // not null
                mac.write_bit(true); // is explicit
                mac.skip(1);
                mac.write_bytes(&data);
            }
            // consensus rules state that prefix nocne must be 0x02 or 0x03
            Nonce::Confidential(generator) => {
                let bytes = generator.serialize();
                mac.write_bit(true); // not null
                mac.write_bit(false); // not explicit
                mac.write_bit(bytes[0] == 0x03); // oddY
                mac.write_bytes(&bytes[1..]);
            }
        }
        Ok(())
    }
}

/// Simplicity has a different logic for computing the transaction input and output
/// digest. This trait defines the method for computation of such digests.
//
// TODO: Review this hashing again now that taproot sighash possibly changed these too
pub(super) trait SimplicityHash {
    /// Add the hash of current tx component
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine);
}

impl SimplicityHash for confidential::Asset {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        match *self {
            Asset::Null => {
                eng.write_u8(0).unwrap();
            }
            Asset::Explicit(asset_id) => {
                eng.write_u8(1).unwrap();
                eng.input(asset_id.into_inner().as_ref());
            }
            Asset::Confidential(generator) => {
                eng.input(&generator.serialize());
            }
        }
    }
}

impl SimplicityHash for confidential::Value {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        match *self {
            Value::Null => {
                eng.write_u8(0).unwrap();
            }
            Value::Explicit(data) => {
                eng.write_u8(1).unwrap();
                eng.write_u64::<BigEndian>(data).unwrap();
            }
            Value::Confidential(comm) => {
                eng.input(&comm.serialize());
            }
        }
    }
}

impl SimplicityHash for confidential::Nonce {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        match *self {
            Nonce::Null => {
                eng.write_u8(0).unwrap();
            }
            Nonce::Explicit(data) => {
                eng.write_u8(1).unwrap();
                eng.input(&data);
            }
            Nonce::Confidential(pubkey) => {
                eng.input(&pubkey.serialize());
            }
        }
    }
}

impl SimplicityHash for elements::Script {
    /// All scripts are first hashed to sha256 to get a scriptpubkey
    /// equivalent and then added to current sha256 context.
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        let script_hash = sha256::Hash::hash(&self.to_bytes());
        eng.input(&script_hash);
    }
}

// FIXME: I think this should belong in rust-elements
pub(super) fn is_asset_reissue(asset: &AssetIssuance) -> bool {
    asset.asset_blinding_nonce.as_ref() != &[0; 32]
}

// FIXME: I think this should belong in rust-elements
pub(super) fn is_asset_new_issue(asset: &AssetIssuance) -> bool {
    asset.asset_blinding_nonce.as_ref() == &[0; 32]
}

impl SimplicityHash for AssetIssuance {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        if is_asset_new_issue(self) {
            self.amount.simplicity_hash(eng);
            self.inflation_keys.simplicity_hash(eng);
            // asset blinding nonce here must be zero
            eng.input(self.asset_blinding_nonce.as_ref());
            eng.input(self.asset_entropy.as_ref());
        } else {
            debug_assert!(is_asset_reissue(self));
            self.amount.simplicity_hash(eng);
            // The inflation keys here must be zero
            // Review this assertion
            let null_amt = Value::Null;
            null_amt.simplicity_hash(eng);
            eng.input(self.asset_blinding_nonce.as_ref());
            eng.input(self.asset_entropy.as_ref());
        }
    }
}

impl SimplicityHash for elements::TxIn {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        eng.input(&self.previous_output.txid);
        eng.write_u32::<LittleEndian>(self.previous_output.vout)
            .unwrap();
        eng.write_u32::<LittleEndian>(self.sequence).unwrap();
        if self.has_issuance() {
            self.asset_issuance.simplicity_hash(eng);
        } else {
            let null_amt = confidential::Value::Null;
            null_amt.simplicity_hash(eng);
            null_amt.simplicity_hash(eng);
        }
    }
}

impl SimplicityHash for Vec<elements::TxIn> {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        for i in self {
            i.simplicity_hash(eng);
        }
    }
}

impl SimplicityHash for elements::TxOut {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        self.asset.simplicity_hash(eng);
        self.value.simplicity_hash(eng);
        self.nonce.simplicity_hash(eng);
        self.script_pubkey.simplicity_hash(eng);
    }
}

impl SimplicityHash for Vec<elements::TxOut> {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        for i in self {
            i.simplicity_hash(eng);
        }
    }
}
