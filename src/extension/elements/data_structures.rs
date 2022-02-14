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

//! # Elements Extensions: Data Structures
//!
//! Data Structures in rust-elements cannot be directly used as-is in
//! rust-simplicity. This file has additional data-structures for
//! simplicity transactions

use crate::cmr::Cmr;
use crate::exec;
use bitcoin_hashes::{sha256, Hash, HashEngine};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use elements::confidential::{Asset, Nonce, Value};
use elements::{confidential, AssetIssuance};

use super::jets::ElementsJetErr;

/// Helper trait for writing various components of
/// Simplicity transactions (assets, values) into bit machine.
pub(in crate::extension::elements) trait SimplicityEncodable {
    // write the simplicity encoding of `self` on bitmachine
    // at the current write cursor.
    fn simplicity_encode(self, mac: &mut exec::BitMachine) -> Result<(), ElementsJetErr>;
}

/// Write an confidential asset to write frame, advancing the cursor 258
/// cells (or returning an error if the asset is null)
///
/// The Simplicity representation of a confidential asset is:
///    ((is_explicit, is_odd), [u8; 32])
impl SimplicityEncodable for confidential::Asset {
    fn simplicity_encode(self, mac: &mut exec::BitMachine) -> Result<(), ElementsJetErr> {
        match self {
            Asset::Null => return Err(ElementsJetErr::NullAssetEncoding),
            Asset::Explicit(data) => {
                mac.write_bit(true);
                mac.skip(1);
                mac.write_bytes(&data);
            }
            // consensus rules state that asset must be 0x0a or 0x0b
            Asset::Confidential(prefix, comm) => {
                if prefix != 0x0a || prefix != 0x0b {
                    unreachable!(
                        "No need to add an return error here. This will be fixed in elements 0.18"
                    )
                }
                mac.write_bit(false); //not explicit
                mac.write_bit(prefix == 0x0b);
                mac.write_bytes(&comm);
            }
        }
        Ok(())
    }
}

/// Write an confidential value to write frame, advancing the cursor 258
/// cells (or returning an error if the value is null)
///
/// The Simplicity representation of a confidential value is:
///     ((is_explicit, is_odd), [u8; 32])
impl SimplicityEncodable for confidential::Value {
    fn simplicity_encode(self, mac: &mut exec::BitMachine) -> Result<(), ElementsJetErr> {
        match self {
            Value::Null => return Err(ElementsJetErr::NullAssetEncoding),
            Value::Explicit(data) => {
                mac.write_bit(true);
                mac.skip(1 + 256 - 64);
                mac.write_u64(data);
            }
            // consensus rules state that prefix value must be 0x08 or 0x09
            Value::Confidential(prefix, comm) => {
                if prefix != 0x08 || prefix != 0x09 {
                    panic!("This is fixed upstream in rust-elements 0.18. This panic would be removed later")
                }
                mac.write_bit(false); //not explicit
                mac.write_bit(prefix == 0x09);
                debug_assert!(comm.len() == 32);
                mac.write_bytes(&comm);
            }
        }
        Ok(())
    }
}

/// Write an confidential nonce to write frame, advancing the cursor 259 cells
///
/// The Simplicity representation of a confidential nonce is:
///     ((is_not_null, is_explicit, is_odd), [u8; 32])
impl SimplicityEncodable for confidential::Nonce {
    fn simplicity_encode(self, mac: &mut exec::BitMachine) -> Result<(), ElementsJetErr> {
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
            Nonce::Confidential(prefix, comm) => {
                if prefix != 0x02 || prefix != 0x03 {
                    panic!("This is fixed upstream in rust-elements 0.18. This panic would be removed later")
                }
                mac.write_bit(true); // not null
                mac.write_bit(false); // not explicit
                mac.write_bit(prefix == 0x03); // oddY
                debug_assert!(comm.len() == 32);
                mac.write_bytes(&comm);
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
            Asset::Explicit(data) => {
                eng.write_u8(1).unwrap();
                eng.input(&data);
            }
            Asset::Confidential(prefix, data) => {
                assert!(prefix == 0x0a || prefix == 0x0b);
                eng.write_u8(prefix).unwrap();
                eng.input(&data);
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
            Value::Confidential(prefix, data) => {
                assert!(prefix == 0x08 || prefix == 0x09);
                eng.write_u8(prefix).unwrap();
                eng.input(&data);
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
            Nonce::Confidential(prefix, data) => {
                assert!(prefix == 0x02 || prefix == 0x03);
                eng.write_u8(prefix).unwrap();
                eng.input(&data);
            }
        }
    }
}

impl SimplicityHash for bitcoin::Script {
    /// All scripts are first hashed to sha256 to get a scriptpubkey
    /// equivalent and then added to current sha256 context.
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        let script_hash = sha256::Hash::hash(&self.to_bytes());
        eng.input(&script_hash);
    }
}

// I think this should belong in rust-elements
pub(super) fn is_asset_reissue(asset: &AssetIssuance) -> bool {
    asset.asset_blinding_nonce != [0; 32]
}

// I think this should belong in rust-elements
pub(super) fn is_asset_new_issue(asset: &AssetIssuance) -> bool {
    asset.asset_blinding_nonce == [0; 32]
}

impl SimplicityHash for AssetIssuance {
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine) {
        if is_asset_new_issue(self) {
            self.amount.simplicity_hash(eng);
            self.inflation_keys.simplicity_hash(eng);
            // asset blinding nonce here must be zero
            eng.input(&self.asset_blinding_nonce);
            eng.input(&self.asset_entropy);
        } else {
            debug_assert!(is_asset_reissue(self));
            self.amount.simplicity_hash(eng);
            // The inflation keys here must be zero
            // Review this assertion
            let null_amt = Value::Null;
            null_amt.simplicity_hash(eng);
            eng.input(&self.asset_blinding_nonce);
            eng.input(&self.asset_entropy);
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

/// An Elements UTXO.
// This is not a complete TxOut as it does not contain the nonce that
// is sent to the recipient.
pub struct ElementsUtxo {
    /// The ``scriptpubkey'' (hash of Simplicity program)
    pub(super) script_pubkey: sha256::Hash,
    /// The explicit or confidential asset
    pub(super) asset: confidential::Asset,
    /// The explict or confidential value
    pub(super) value: confidential::Value,
}

/// Transaction environment for Bitcoin Simplicity programs
///
/// # Note
/// The order of `utxos` must be same as of the order of inputs in the
/// transaction.
// FIXME: tx can be shared across multiple inputs in the same transaction.
// Changing tx to reference does not directly work as the trait declarations do
// not support generic assiciated types.
// Other ways of fixing this either
// 1) involve implementing triat for a reference which we cannot do
// currently because the trait also has a constructor that needs to return Self.
// 2) Changing the trait definition to have explicit lifetime parameter that complicates
// the usage of the trait because we need to pass around a generic everywhere we use the jet
// trait. This maybe slightly annoying but maybe what we have to do until have Associated generic types
pub struct TxEnv {
    /// The elements transaction
    pub(super) tx: elements::Transaction,
    /// The input utxo information corresponding to outpoint being spent.
    pub(super) utxos: Vec<ElementsUtxo>,
    /// the current index of the input
    pub(super) ix: u32,
    /// Commitment merkle root of the script being executed
    pub(super) script_cmr: Cmr,
    /// cached InputHash
    pub(super) inputs_hash: sha256::Hash,
    /// cached OutputHash
    pub(super) outputs_hash: sha256::Hash,
}

impl TxEnv {
    /// Constructor from a transaction
    pub fn from_txenv(
        tx: elements::Transaction,
        utxos: Vec<ElementsUtxo>,
        ix: u32,
        script_cmr: Cmr,
    ) -> TxEnv {
        let mut inp_eng = sha256::Hash::engine();
        let mut output_eng = sha256::Hash::engine();
        // compute the hash
        tx.input.simplicity_hash(&mut inp_eng);
        tx.output.simplicity_hash(&mut output_eng);
        let inputs_hash = sha256::Hash::from_engine(inp_eng);
        let outputs_hash = sha256::Hash::from_engine(output_eng);
        TxEnv {
            tx: tx,
            utxos: utxos,
            ix: ix,
            script_cmr: script_cmr,
            inputs_hash: inputs_hash,
            outputs_hash: outputs_hash,
        }
    }
}
