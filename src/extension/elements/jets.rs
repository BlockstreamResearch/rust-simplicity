// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Elements Extensions: Jets
//!
//! Jets to the Simplicity language to allow use on the Elements
//! blockchain.
//!

use std::error;
use std::{fmt, io};

use super::data_structures::{is_asset_new_issue, is_asset_reissue, SimplicityEncodable, TxEnv};
use crate::bititer::BitIter;
use crate::cmr::Cmr;
use crate::encode;
use crate::exec;
use crate::extension::TypeName;
use crate::extension::{self, ExtError};
use crate::Error;
use bitcoin_hashes::{sha256, Hash};
use elements::confidential::Value;

/// Set of new Simplicity nodes enabled by the Bitcoin extension
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum ElementsNode {
    Version,
    LockTime,
    InputIsPegin,
    InputPrevOutpoint,
    InputAsset,
    InputAmount,
    InputScriptHash,
    InputSequence,
    InputIssuanceBlinding,
    InputIssuanceContract,
    InputIssuanceEntropy,
    InputIssuanceAssetAmount,
    InputIssuanceTokenAmount,
    OutputAsset,
    OutputAmount,
    OutputNonce,
    OutputScriptHash,
    OutputNullDatum,
    ScriptCmr,
    CurrentIndex,
    CurrentIsPegin,
    CurrentPrevOutpoint,
    CurrentAsset,
    CurrentAmount,
    CurrentScriptHash,
    CurrentSequence,
    CurrentIssuanceBlinding,
    CurrentIssuanceContract,
    CurrentIssuanceEntropy,
    CurrentIssuanceAssetAmount,
    CurrentIssuanceTokenAmount,
    InputsHash,
    OutputsHash,
    NumInputs,
    NumOutputs,
    Fee,
}

impl fmt::Display for ElementsNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            ElementsNode::Version => "version",
            ElementsNode::LockTime => "locktime",
            ElementsNode::InputIsPegin => "inputispegin",
            ElementsNode::InputPrevOutpoint => "inputprevoutpoint",
            ElementsNode::InputAsset => "inputasset",
            ElementsNode::InputAmount => "inputamount",
            ElementsNode::InputScriptHash => "inputscripthash",
            ElementsNode::InputSequence => "inputsequence",
            ElementsNode::InputIssuanceBlinding => "inputissuanceblinding",
            ElementsNode::InputIssuanceContract => "inputissuancecontract",
            ElementsNode::InputIssuanceEntropy => "inputissuanceentropy",
            ElementsNode::InputIssuanceAssetAmount => "inputissuanceassetamount",
            ElementsNode::InputIssuanceTokenAmount => "inputissuancetokenamount",
            ElementsNode::OutputAsset => "outputasset",
            ElementsNode::OutputAmount => "outputamount",
            ElementsNode::OutputNonce => "outputnonce",
            ElementsNode::OutputScriptHash => "outputscripthash",
            ElementsNode::OutputNullDatum => "outputnulldatum",
            ElementsNode::ScriptCmr => "scriptcmr",
            ElementsNode::CurrentIndex => "currentIndex",
            ElementsNode::CurrentIsPegin => "currentIspegin",
            ElementsNode::CurrentPrevOutpoint => "currentprevoutpoint",
            ElementsNode::CurrentAsset => "currentasset",
            ElementsNode::CurrentAmount => "currentamount",
            ElementsNode::CurrentScriptHash => "currentscripthash",
            ElementsNode::CurrentSequence => "currentsequence",
            ElementsNode::CurrentIssuanceBlinding => "currentissuanceblinding",
            ElementsNode::CurrentIssuanceContract => "currentissuancecontract",
            ElementsNode::CurrentIssuanceEntropy => "currentissuanceentropy",
            ElementsNode::CurrentIssuanceAssetAmount => "currentissuanceassetAmount",
            ElementsNode::CurrentIssuanceTokenAmount => "currentissuancetokenAmount",
            ElementsNode::InputsHash => "inputshash",
            ElementsNode::OutputsHash => "outputshash",
            ElementsNode::NumInputs => "numinputs",
            ElementsNode::NumOutputs => "numoutputs",
            ElementsNode::Fee => "fee",
        })
    }
}

impl extension::Jet for ElementsNode {
    type TxEnv = TxEnv;
    type JetErr = ElementsJetErr;

    fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<ElementsNode, Error> {
        let code = match iter.read_bits_be(5) {
            Some(code) => code,
            None => return Err(Error::EndOfStream),
        };
        match code {
            0 => match iter.next() {
                Some(false) => Ok(ElementsNode::Version),
                Some(true) => Ok(ElementsNode::LockTime),
                None => Err(Error::EndOfStream),
            },
            1 => Ok(ElementsNode::InputIsPegin),
            2 => Ok(ElementsNode::InputPrevOutpoint),
            3 => Ok(ElementsNode::InputAsset),
            4 => match iter.next() {
                Some(false) => Ok(ElementsNode::InputAmount),
                Some(true) => Ok(ElementsNode::InputScriptHash),
                None => Err(Error::EndOfStream),
            },
            5 => Ok(ElementsNode::InputSequence),
            6 => Ok(ElementsNode::InputIssuanceBlinding),
            7 => Ok(ElementsNode::InputIssuanceContract),
            8 => match iter.next() {
                Some(false) => Ok(ElementsNode::InputIssuanceEntropy),
                Some(true) => Ok(ElementsNode::InputIssuanceAssetAmount),
                None => Err(Error::EndOfStream),
            },
            9 => Ok(ElementsNode::InputIssuanceTokenAmount),
            10 => Ok(ElementsNode::OutputAsset),
            11 => Ok(ElementsNode::OutputAmount),
            12 => match iter.next() {
                Some(false) => Ok(ElementsNode::OutputNonce),
                Some(true) => Ok(ElementsNode::OutputScriptHash),
                None => Err(Error::EndOfStream),
            },
            13 => Ok(ElementsNode::OutputNullDatum),
            14 => Ok(ElementsNode::ScriptCmr),
            15 => Ok(ElementsNode::CurrentIndex),
            16 => Ok(ElementsNode::CurrentIsPegin),
            17 => Ok(ElementsNode::CurrentPrevOutpoint),
            18 => Ok(ElementsNode::CurrentAsset),
            19 => Ok(ElementsNode::CurrentAmount),
            20 => Ok(ElementsNode::CurrentScriptHash),
            21 => Ok(ElementsNode::CurrentSequence),
            22 => Ok(ElementsNode::CurrentIssuanceBlinding),
            23 => Ok(ElementsNode::CurrentIssuanceContract),
            24 => Ok(ElementsNode::CurrentIssuanceEntropy),
            25 => Ok(ElementsNode::CurrentIssuanceAssetAmount),
            26 => Ok(ElementsNode::CurrentIssuanceTokenAmount),
            27 => Ok(ElementsNode::InputsHash),
            28 => Ok(ElementsNode::OutputsHash),
            29 => Ok(ElementsNode::NumInputs),
            30 => Ok(ElementsNode::NumOutputs),
            31 => Ok(ElementsNode::Fee),
            _ => unreachable!(),
        }
    }

    fn source_type(&self) -> TypeName {
        match *self {
            ElementsNode::Version => TypeName(b"1"),
            ElementsNode::LockTime => TypeName(b"1"),
            ElementsNode::InputIsPegin
            | ElementsNode::InputPrevOutpoint
            | ElementsNode::InputAsset
            | ElementsNode::InputAmount
            | ElementsNode::InputScriptHash
            | ElementsNode::InputSequence
            | ElementsNode::InputIssuanceBlinding
            | ElementsNode::InputIssuanceContract
            | ElementsNode::InputIssuanceEntropy
            | ElementsNode::InputIssuanceAssetAmount
            | ElementsNode::InputIssuanceTokenAmount
            | ElementsNode::OutputAsset
            | ElementsNode::OutputAmount
            | ElementsNode::OutputNonce
            | ElementsNode::OutputScriptHash => TypeName(b"i"),
            ElementsNode::OutputNullDatum => TypeName(b"*ii"),
            ElementsNode::ScriptCmr
            | ElementsNode::CurrentIndex
            | ElementsNode::CurrentIsPegin
            | ElementsNode::CurrentPrevOutpoint
            | ElementsNode::CurrentAsset
            | ElementsNode::CurrentAmount
            | ElementsNode::CurrentScriptHash
            | ElementsNode::CurrentSequence
            | ElementsNode::CurrentIssuanceBlinding
            | ElementsNode::CurrentIssuanceContract
            | ElementsNode::CurrentIssuanceEntropy
            | ElementsNode::CurrentIssuanceAssetAmount
            | ElementsNode::CurrentIssuanceTokenAmount
            | ElementsNode::InputsHash
            | ElementsNode::OutputsHash
            | ElementsNode::NumInputs
            | ElementsNode::NumOutputs => TypeName(b"1"),
            ElementsNode::Fee => TypeName(b"h"),
        }
    }

    /// Name of the target type for this node
    fn target_type(&self) -> TypeName {
        match *self {
            ElementsNode::Version => TypeName(b"i"),
            ElementsNode::LockTime => TypeName(b"i"),
            ElementsNode::InputIsPegin => TypeName(b"+12"),
            ElementsNode::InputPrevOutpoint => TypeName(b"+1*hi"),
            ElementsNode::InputAsset => TypeName(b"+1+*2hh"),
            ElementsNode::InputAmount => TypeName(b"+1+*2hl"),
            ElementsNode::InputScriptHash => TypeName(b"+1h"),
            ElementsNode::InputSequence => TypeName(b"+1i"),
            ElementsNode::InputIssuanceBlinding => TypeName(b"+1+1h"),
            ElementsNode::InputIssuanceContract => TypeName(b"+1+1h"),
            ElementsNode::InputIssuanceEntropy => TypeName(b"+1+1h"),
            ElementsNode::InputIssuanceAssetAmount => TypeName(b"+1+1+*2hl"),
            ElementsNode::InputIssuanceTokenAmount => TypeName(b"+1+1+*2hl"),
            ElementsNode::OutputAsset => TypeName(b"+1+*2hh"),
            ElementsNode::OutputAmount => TypeName(b"+1+*2hl"),
            ElementsNode::OutputNonce => TypeName(b"+1+*2hh"),
            ElementsNode::OutputScriptHash => TypeName(b"+1h"),
            ElementsNode::OutputNullDatum => TypeName(b"+1+1+**22h+2*22"),
            ElementsNode::ScriptCmr => TypeName(b"h"),
            ElementsNode::CurrentIndex => TypeName(b"i"),
            ElementsNode::CurrentIsPegin => TypeName(b"2"),
            ElementsNode::CurrentPrevOutpoint => TypeName(b"*hi"),
            ElementsNode::CurrentAsset => TypeName(b"+*2hh"),
            ElementsNode::CurrentAmount => TypeName(b"+*2hl"),
            ElementsNode::CurrentScriptHash => TypeName(b"h"),
            ElementsNode::CurrentSequence => TypeName(b"i"),
            ElementsNode::CurrentIssuanceBlinding => TypeName(b"+1h"),
            ElementsNode::CurrentIssuanceContract => TypeName(b"+1h"),
            ElementsNode::CurrentIssuanceEntropy => TypeName(b"+1h"),
            ElementsNode::CurrentIssuanceAssetAmount => TypeName(b"+1+*2hl"),
            ElementsNode::CurrentIssuanceTokenAmount => TypeName(b"+1+*2hl"),
            ElementsNode::InputsHash => TypeName(b"h"),
            ElementsNode::OutputsHash => TypeName(b"h"),
            ElementsNode::NumInputs => TypeName(b"i"),
            ElementsNode::NumOutputs => TypeName(b"i"),
            ElementsNode::Fee => TypeName(b"l"),
        }
    }

    fn cmr(&self) -> Cmr {
        match *self {
            ElementsNode::Version => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fversion")
            }
            ElementsNode::LockTime => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1flockTime")
            }
            ElementsNode::InputIsPegin => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputIsPegin")
            }
            ElementsNode::InputPrevOutpoint => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputPrevOutpoint")
            }
            ElementsNode::InputAsset => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputAsset")
            }
            ElementsNode::InputAmount => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputAmount")
            }
            ElementsNode::InputScriptHash => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputScriptHash")
            }
            ElementsNode::InputSequence => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputSequence")
            }
            ElementsNode::InputIssuanceBlinding => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputIssuanceBlinding")
            }
            ElementsNode::InputIssuanceContract => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputIssuanceContract")
            }
            ElementsNode::InputIssuanceEntropy => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputIssuanceEntropy")
            }
            ElementsNode::InputIssuanceAssetAmount => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputIssuanceAssetAmt")
            }
            ElementsNode::InputIssuanceTokenAmount => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputIssuanceTokenAmt")
            }
            ElementsNode::OutputAsset => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1foutputAsset")
            }
            ElementsNode::OutputAmount => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1foutputAmount")
            }
            ElementsNode::OutputNonce => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1foutputNonce")
            }
            ElementsNode::OutputScriptHash => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1foutputScriptHash")
            }
            ElementsNode::OutputNullDatum => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1foutputNullDatum")
            }
            ElementsNode::ScriptCmr => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fscriptCMR")
            }
            ElementsNode::CurrentIndex => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentIndex")
            }
            ElementsNode::CurrentIsPegin => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentIsPegin")
            }
            ElementsNode::CurrentPrevOutpoint => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentPrevOutpoint")
            }
            ElementsNode::CurrentAsset => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentAsset")
            }
            ElementsNode::CurrentAmount => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentAmount")
            }
            ElementsNode::CurrentScriptHash => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentScriptHash")
            }
            ElementsNode::CurrentSequence => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentSequence")
            }
            ElementsNode::CurrentIssuanceBlinding => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentIssuanceBlinding")
            }
            ElementsNode::CurrentIssuanceContract => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentIssuanceContract")
            }
            ElementsNode::CurrentIssuanceEntropy => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentIssuanceEntropy")
            }
            ElementsNode::CurrentIssuanceAssetAmount => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentIssuanceAssetAmt")
            }
            ElementsNode::CurrentIssuanceTokenAmount => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fcurrentIssuanceTokenAmt")
            }
            ElementsNode::InputsHash => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1finputsHash")
            }
            ElementsNode::OutputsHash => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1foutputsHash")
            }
            ElementsNode::NumInputs => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fnumInputs")
            }
            ElementsNode::NumOutputs => {
                Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1fnumOutputs")
            }
            ElementsNode::Fee => Cmr::new(b"Simplicity-Draft\x1fPrimitive\x1fElements\x1ffee"),
        }
    }

    fn wmr(&self) -> Cmr {
        self.cmr()
    }

    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize> {
        match *self {
            ElementsNode::Version => w.write_u8(128 + 0, 8),
            ElementsNode::LockTime => w.write_u8(128 + 1, 8),
            ElementsNode::InputIsPegin => w.write_u8(64 + 1, 7),
            ElementsNode::InputPrevOutpoint => w.write_u8(64 + 2, 7),
            ElementsNode::InputAsset => w.write_u8(64 + 3, 7),
            ElementsNode::InputAmount => w.write_u8(128 + 2, 8),
            ElementsNode::InputScriptHash => w.write_u8(128 + 3, 8),
            ElementsNode::InputSequence => w.write_u8(64 + 5, 7),
            ElementsNode::InputIssuanceBlinding => w.write_u8(64 + 6, 7),
            ElementsNode::InputIssuanceContract => w.write_u8(64 + 7, 7),
            ElementsNode::InputIssuanceEntropy => w.write_u8(128 + 4, 8),
            ElementsNode::InputIssuanceAssetAmount => w.write_u8(128 + 5, 8),
            ElementsNode::InputIssuanceTokenAmount => w.write_u8(64 + 9, 7),
            ElementsNode::OutputAsset => w.write_u8(64 + 10, 7),
            ElementsNode::OutputAmount => w.write_u8(64 + 11, 7),
            ElementsNode::OutputNonce => w.write_u8(128 + 6, 8),
            ElementsNode::OutputScriptHash => w.write_u8(128 + 7, 8),
            ElementsNode::OutputNullDatum => w.write_u8(64 + 13, 7),
            ElementsNode::ScriptCmr => w.write_u8(64 + 14, 7),
            ElementsNode::CurrentIndex => w.write_u8(64 + 15, 7),
            ElementsNode::CurrentIsPegin => w.write_u8(64 + 16, 7),
            ElementsNode::CurrentPrevOutpoint => w.write_u8(64 + 17, 7),
            ElementsNode::CurrentAsset => w.write_u8(64 + 18, 7),
            ElementsNode::CurrentAmount => w.write_u8(64 + 19, 7),
            ElementsNode::CurrentScriptHash => w.write_u8(64 + 20, 7),
            ElementsNode::CurrentSequence => w.write_u8(64 + 21, 7),
            ElementsNode::CurrentIssuanceBlinding => w.write_u8(64 + 22, 7),
            ElementsNode::CurrentIssuanceContract => w.write_u8(64 + 23, 7),
            ElementsNode::CurrentIssuanceEntropy => w.write_u8(64 + 24, 7),
            ElementsNode::CurrentIssuanceAssetAmount => w.write_u8(64 + 25, 7),
            ElementsNode::CurrentIssuanceTokenAmount => w.write_u8(64 + 26, 7),
            ElementsNode::InputsHash => w.write_u8(64 + 27, 7),
            ElementsNode::OutputsHash => w.write_u8(64 + 28, 7),
            ElementsNode::NumInputs => w.write_u8(64 + 29, 7),
            ElementsNode::NumOutputs => w.write_u8(64 + 30, 7),
            ElementsNode::Fee => w.write_u8(64 + 31, 7),
        }
    }

    fn exec(&self, mac: &mut exec::BitMachine, txenv: &Self::TxEnv) -> Result<(), ElementsJetErr> {
        assert!(txenv.tx.input.len() == txenv.utxos.len());
        // env must always be valid.
        let curr_idx = txenv.ix as usize;
        let curr_inp = &txenv.tx.input[curr_idx];
        let curr_utxo = &txenv.utxos[curr_idx];
        match *self {
            ElementsNode::Version => mac.write_u32(txenv.tx.version),
            ElementsNode::LockTime => mac.write_u32(txenv.tx.lock_time),
            ElementsNode::InputIsPegin => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    mac.write_bit(txenv.tx.input[idx].is_pegin);
                } else {
                    mac.skip(1);
                }
            }
            ElementsNode::InputPrevOutpoint => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    mac.write_bytes(&txenv.tx.input[idx].previous_output.txid);
                    mac.write_u32(txenv.tx.input[idx].previous_output.vout);
                } else {
                    mac.skip(256 + 32);
                }
            }
            ElementsNode::InputAsset => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let asset = txenv.utxos[idx].asset;
                    asset.simplicity_encode(mac)?
                } else {
                    // 2 bits for prefix and 256 bits for hash.
                    mac.skip(2 + 256);
                }
            }
            ElementsNode::InputAmount => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let amt = txenv.utxos[idx].value;
                    amt.simplicity_encode(mac)?
                } else {
                    // 2 bits for prefix and 256 bits for hash.
                    mac.skip(2 + 256);
                }
            }
            ElementsNode::InputScriptHash => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let script_pubkey = txenv.utxos[idx].script_pubkey;
                    mac.write_bytes(&script_pubkey);
                } else {
                    // 256 bits for hash.
                    mac.skip(256);
                }
            }
            ElementsNode::InputSequence => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let seq = txenv.tx.input[idx].sequence;
                    mac.write_u32(seq);
                } else {
                    // 32 bits for sequence.
                    mac.skip(32);
                }
            }
            ElementsNode::InputIssuanceBlinding => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    assert!(txenv.tx.input[idx].has_issuance());
                    blinding_issuance(mac, &txenv.tx.input[idx].asset_issuance);
                } else {
                    // issuance_type + 256 hash bits.
                    mac.skip(1 + 256);
                }
            }
            ElementsNode::InputIssuanceContract => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    assert!(txenv.tx.input[idx].has_issuance());
                    contract_issuance(mac, &txenv.tx.input[idx].asset_issuance);
                } else {
                    // issuance type + 256 bits for hash.
                    mac.skip(1 + 256);
                }
            }
            ElementsNode::InputIssuanceEntropy => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    assert!(txenv.tx.input[idx].has_issuance());
                    let asset = txenv.tx.input[idx].asset_issuance;
                    entropy_issuance(mac, &asset);
                } else {
                    // 1 + 256 bits for hash.
                    mac.skip(1 + 256);
                }
            }
            ElementsNode::InputIssuanceAssetAmount => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let asset = txenv.tx.input[idx].asset_issuance;
                    asset_amt_issuance(mac, &asset, txenv.tx.input[idx].has_issuance())?
                } else {
                    // 1 + 258 bits for conf value.
                    mac.skip(1 + 258);
                }
            }
            ElementsNode::InputIssuanceTokenAmount => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.input.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let asset = txenv.tx.input[idx].asset_issuance;
                    inflation_amt_issuance(mac, &asset)?
                } else {
                    // 1 + 258 bits for conf value.
                    mac.skip(1 + 258);
                }
            }
            ElementsNode::OutputAsset => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.output.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let asset = txenv.tx.output[idx].asset;
                    asset.simplicity_encode(mac)?
                } else {
                    // 258 bits for conf value.
                    mac.skip(258);
                }
            }
            ElementsNode::OutputAmount => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.output.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let value = txenv.tx.output[idx].value;
                    value.simplicity_encode(mac)?
                } else {
                    // 258 bits for conf value.
                    mac.skip(258);
                }
            }
            ElementsNode::OutputNonce => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.output.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let nonce = txenv.tx.output[idx].nonce;
                    nonce.simplicity_encode(mac)?
                } else {
                    // 259 bits for conf nonce.
                    mac.skip(259);
                }
            }
            ElementsNode::OutputScriptHash => {
                let idx = mac.read_u32() as usize;
                let is_valid_idx = idx < txenv.tx.output.len();
                mac.write_bit(is_valid_idx);
                if is_valid_idx {
                    let output_script_pubkey = &txenv.tx.output[idx].script_pubkey;
                    mac.write_bytes(&sha256::Hash::hash(&output_script_pubkey.to_bytes()));
                } else {
                    // 256 bits of hash.
                    mac.skip(256);
                }
            }
            ElementsNode::OutputNullDatum => unimplemented!(),
            ElementsNode::ScriptCmr => {
                mac.write_bytes(&txenv.script_cmr);
            }
            ElementsNode::CurrentIndex => {
                mac.write_u32(txenv.ix);
            }
            ElementsNode::CurrentIsPegin => {
                mac.write_bit(curr_inp.is_pegin());
            }
            ElementsNode::CurrentPrevOutpoint => {
                mac.write_bytes(&curr_inp.previous_output.txid);
                mac.write_u32(curr_inp.previous_output.vout);
            }
            ElementsNode::CurrentAsset => curr_utxo.asset.simplicity_encode(mac)?,
            ElementsNode::CurrentAmount => curr_utxo.value.simplicity_encode(mac)?,
            ElementsNode::CurrentScriptHash => {
                // TODO: cache these while creating utxo
                mac.write_bytes(&curr_utxo.script_pubkey);
            }
            ElementsNode::CurrentSequence => {
                mac.write_u32(curr_inp.sequence);
            }
            ElementsNode::CurrentIssuanceBlinding => {
                assert!(curr_inp.has_issuance());
                blinding_issuance(mac, &curr_inp.asset_issuance)
            }
            ElementsNode::CurrentIssuanceContract => {
                assert!(curr_inp.has_issuance());
                contract_issuance(mac, &curr_inp.asset_issuance);
            }
            ElementsNode::CurrentIssuanceEntropy => {
                assert!(curr_inp.has_issuance());
                entropy_issuance(mac, &curr_inp.asset_issuance);
            }
            ElementsNode::CurrentIssuanceAssetAmount => {
                asset_amt_issuance(mac, &curr_inp.asset_issuance, curr_inp.has_issuance())?
            }
            ElementsNode::CurrentIssuanceTokenAmount => {
                assert!(curr_inp.has_issuance());
                inflation_amt_issuance(mac, &curr_inp.asset_issuance)?
            }
            /*
            inputHash(l) :=
            BE256(LE[prevOutpoint.txid]),LE32(prevOutpoint.vout),LE32(sequence),encIssuance(l[issuance])
            */
            ElementsNode::InputsHash => {
                mac.write_bytes(&txenv.inputs_hash);
            }
            ElementsNode::OutputsHash => {
                mac.write_bytes(&txenv.outputs_hash);
            }
            ElementsNode::NumInputs => {
                mac.write_u32(txenv.tx.input.len() as u32);
            }
            ElementsNode::NumOutputs => {
                mac.write_u32(txenv.tx.output.len() as u32);
            }
            ElementsNode::Fee => unimplemented!(),
        };
        Ok(())
    }
}

// Write an optional 'blindingNonce' from an 'assetIssuance' when reissuing an asset
// writes 257 bits
fn blinding_issuance(mac: &mut exec::BitMachine, issuance: &elements::AssetIssuance) {
    let is_reissue = is_asset_reissue(issuance);
    mac.write_bit(is_reissue);
    if is_reissue {
        mac.write_bytes(&issuance.asset_blinding_nonce);
    } else {
        mac.skip(256);
    }
}

// Write an optional 'contractHash' from an 'assetIssuance' when issuing new asset
fn contract_issuance(mac: &mut exec::BitMachine, issuance: &elements::AssetIssuance) {
    let is_new_issue = is_asset_new_issue(issuance);
    mac.write_bit(is_new_issue);
    if is_new_issue {
        mac.write_bytes(&issuance.asset_entropy);
    } else {
        mac.skip(256);
    }
}

// Write an optional 'entropy' from an 'assetIssuance' when reissuing an asset
fn entropy_issuance(mac: &mut exec::BitMachine, issuance: &elements::AssetIssuance) {
    let is_reissue = is_asset_reissue(issuance);
    mac.write_bit(is_reissue);
    if is_reissue {
        mac.write_bytes(&issuance.asset_entropy);
    } else {
        mac.skip(256);
    }
}

// Write an optional confidential asset amount 'amount' from an 'assetIssuance'
fn asset_amt_issuance(
    mac: &mut exec::BitMachine,
    issuance: &elements::AssetIssuance,
    has_issuance: bool,
) -> Result<(), ElementsJetErr> {
    let is_null_amt = matches!(issuance.amount, Value::Null);
    mac.write_bit(has_issuance && !is_null_amt);
    if has_issuance {
        issuance.amount.simplicity_encode(mac)?;
    } else {
        // confidential value 258 bits
        mac.skip(2 + 256);
    }
    Ok(())
}

// Write an optional confidential new token amount 'amount' from an 'assetIssuance'
fn inflation_amt_issuance(
    mac: &mut exec::BitMachine,
    issuance: &elements::AssetIssuance,
) -> Result<(), ElementsJetErr> {
    let is_null_amt = matches!(issuance.amount, Value::Null);
    let is_new_issue = is_asset_new_issue(issuance);
    mac.write_bit(is_new_issue && !is_null_amt);
    if is_new_issue {
        issuance.inflation_keys.simplicity_encode(mac)?;
    } else {
        // confidential value 258 bits
        mac.skip(2 + 256);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ElementsJetErr {
    /// Tried to encode Null Asset
    NullAssetEncoding,
    /// Tried to encode Null Value
    NullValueEncoding,
}

impl fmt::Display for ElementsJetErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElementsJetErr::NullAssetEncoding => write!(f, "tried to encode null asset"),
            ElementsJetErr::NullValueEncoding => write!(f, "tried to encode null value"),
        }
    }
}

impl error::Error for ElementsJetErr {}
impl ExtError for ElementsJetErr {}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::cmr;
    use crate::extension::elements::data_structures::{ElementsUtxo, TxEnv};
    use crate::extension::elements::test_sighashall::{
        ELEMENTS_CHECK_SIGHASH_ALL, SIGHASH_ALL_AMR, SIGHASH_ALL_CMR,
    };
    use bitcoin::Script;
    use bitcoin_hashes::sha256;
    use elements::{
        confidential, AssetIssuance, OutPoint, Transaction, TxIn, TxInWitness, TxOut, TxOutWitness,
    };

    #[test]
    fn sighash_all_cmr() {
        // Run SighashALL program
        let mut bits: crate::bititer::BitIter<_> =
            ELEMENTS_CHECK_SIGHASH_ALL.iter().cloned().into();
        let program =
            crate::program::Program::<crate::extension::elements::ElementsNode>::decode(&mut bits)
                .expect("decoding program");
        assert_eq!(program.root_node().cmr.into_inner(), SIGHASH_ALL_CMR);
        assert_eq!(program.root_node().amr.into_inner(), SIGHASH_ALL_AMR);
        // FIXME: Implement and check wmr
    }
    #[test]
    #[ignore] // too expensive to run. Run with -- --ignored to run this
    fn exec_sighash_all() {
        /*
        rawTransaction testTx1 = (rawTransaction)
          { .input = (rawInput[])
                     { { .prevTxid = (unsigned char[32]){"\xeb\x04\xb6\x8e\x9a\x26\xd1\x16\x04\x6c\x76\xe8\xff\x47\x33\x2f\xb7\x1d\xda\x90\xff\x4b\xef\x53\x70\xf2\x52\x26\xd3\xbc\x09\xfc"}
                       , .prevIx = 0
                       , .sequence = 0xfffffffe
                       , .isPegin = false
                       , .issuance = {0}
                       , .txo = { .asset = (unsigned char[33]){"\x01\x23\x0f\x4f\x5d\x4b\x7c\x6f\xa8\x45\x80\x6e\xe4\xf6\x77\x13\x45\x9e\x1b\x69\xe8\xe6\x0f\xce\xe2\xe4\x94\x0c\x7a\x0d\x5d\xe1\xb2"}
                                , .value = (unsigned char[9]){"\x01\x00\x00\x00\x02\x54\x0b\xe4\x00"}
                                , .scriptPubKey = {0}
                     } }        }
          , .output = (rawOutput[])
                      { { .asset = (unsigned char[33]){"\x01\x23\x0f\x4f\x5d\x4b\x7c\x6f\xa8\x45\x80\x6e\xe4\xf6\x77\x13\x45\x9e\x1b\x69\xe8\xe6\x0f\xce\xe2\xe4\x94\x0c\x7a\x0d\x5d\xe1\xb2"}
                        , .value = (unsigned char[9]){"\x01\x00\x00\x00\x02\x54\x0b\xd7\x1c"}
                        , .nonce = NULL
                        , .scriptPubKey = { .code = (unsigned char [26]){"\x19\x76\xa9\x14\x48\x63\x3e\x2c\x0e\xe9\x49\x5d\xd3\xf9\xc4\x37\x32\xc4\x7f\x47\x02\xa3\x62\xc8\x88\xac"}
                                          , .len = 26
                                          }
                        }
                      , { .asset = (unsigned char[33]){"\x01\x23\x0f\x4f\x5d\x4b\x7c\x6f\xa8\x45\x80\x6e\xe4\xf6\x77\x13\x45\x9e\x1b\x69\xe8\xe6\x0f\xce\xe2\xe4\x94\x0c\x7a\x0d\x5d\xe1\xb2"}
                        , .value = (unsigned char[9]){"\x01\x00\x00\x00\x00\x00\x00\x0c\xe4"}
                        , .nonce = NULL
                        , .scriptPubKey = {0}
                      } }
          , .numInputs = 1
          , .numOutputs = 2
          , .version = 0x00000002
          , .lockTime = 0x00000000
          };
        */
        let asset: [u8; 32] = [
            0x23, 0x0f, 0x4f, 0x5d, 0x4b, 0x7c, 0x6f, 0xa8, 0x45, 0x80, 0x6e, 0xe4, 0xf6, 0x77,
            0x13, 0x45, 0x9e, 0x1b, 0x69, 0xe8, 0xe6, 0x0f, 0xce, 0xe2, 0xe4, 0x94, 0x0c, 0x7a,
            0x0d, 0x5d, 0xe1, 0xb2,
        ];
        let tx_id: [u8; 32] = [
            0xeb, 0x04, 0xb6, 0x8e, 0x9a, 0x26, 0xd1, 0x16, 0x04, 0x6c, 0x76, 0xe8, 0xff, 0x47,
            0x33, 0x2f, 0xb7, 0x1d, 0xda, 0x90, 0xff, 0x4b, 0xef, 0x53, 0x70, 0xf2, 0x52, 0x26,
            0xd3, 0xbc, 0x09, 0xfc,
        ];
        use crate::bitcoin_hashes::Hash;
        use bitcoin_hashes::sha256d;
        let asset = confidential::Asset::Explicit(sha256d::Hash::from_inner(asset));
        //create the txenv
        let elements_tx = Transaction {
            version: 2,
            lock_time: 0,
            input: vec![TxIn {
                previous_output: OutPoint {
                    txid: bitcoin::Txid::from_inner(tx_id),
                    vout: 0,
                },
                sequence: 0xfffffffe,
                is_pegin: false,
                has_issuance: false,
                // perhaps make this an option in elements upstream?
                asset_issuance: AssetIssuance {
                    asset_blinding_nonce: [0; 32],
                    asset_entropy: [0; 32],
                    amount: confidential::Value::Null,
                    inflation_keys: confidential::Value::Null,
                },
                script_sig: Script::new(),
                witness: TxInWitness {
                    amount_rangeproof: vec![],
                    inflation_keys_rangeproof: vec![],
                    script_witness: vec![],
                    pegin_witness: vec![],
                },
            }],
            output: vec![
                TxOut {
                    asset: asset.clone(),
                    value: confidential::Value::Explicit(0x00000002540bd71c),
                    nonce: confidential::Nonce::Null,
                    script_pubkey: hex_script(
                        &"1976a91448633e2c0ee9495dd3f9c43732c47f4702a362c888ac",
                    ),
                    witness: TxOutWitness {
                        surjection_proof: vec![],
                        rangeproof: vec![],
                    },
                },
                TxOut {
                    asset: asset.clone(),
                    value: confidential::Value::Explicit(0x0000000000000ce4),
                    nonce: confidential::Nonce::Null,
                    script_pubkey: Script::new(),
                    witness: TxOutWitness {
                        surjection_proof: vec![],
                        rangeproof: vec![],
                    },
                },
            ],
        };
        let utxo = ElementsUtxo {
            script_pubkey: sha256::Hash::from_inner([0; 32]),
            asset: asset,
            value: confidential::Value::Explicit(0x00000002540be400),
        };

        let cmr = cmr::Cmr::from(SIGHASH_ALL_CMR);
        let mut bits: crate::bititer::BitIter<_> =
            ELEMENTS_CHECK_SIGHASH_ALL.iter().cloned().into();
        let program =
            crate::program::Program::<crate::extension::elements::ElementsNode>::decode(&mut bits)
                .expect("decoding program");
        let txenv = TxEnv::from_txenv(Arc::new(elements_tx), vec![utxo], 0, cmr);

        //finally run the program
        let mut mac = crate::exec::BitMachine::for_program(&program);
        mac.exec(&program, &txenv).unwrap();
    }
    #[cfg(test)]
    fn hex_script(s: &str) -> bitcoin::Script {
        let v: Vec<u8> = bitcoin::hashes::hex::FromHex::from_hex(s).unwrap();
        bitcoin::Script::from(v)
    }
}
