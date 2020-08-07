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

//! # Elements Extensions
//!
//! Extensions to the Simplicity language to allow use on the Bitcoin
//! blockchain
//!

use std::{fmt, io};

use super::TypeName;
use crate::bititer::BitIter;
use crate::cmr::Cmr;
use crate::encode;
use crate::exec;
use crate::extension;
use crate::Error;

/// Transaction environment for Bitcoin Simplicity programs
pub struct TxEnv {
    _tx: elements::Transaction,
}

impl TxEnv {
    /// Constructor from a transaction
    pub fn from_tx(tx: elements::Transaction) -> TxEnv {
        TxEnv { _tx: tx }
    }
}

/// Set of new Simplicity nodes enabled by the Bitcoin extension
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
            ElementsNode::Version => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fversion"),
            ElementsNode::LockTime => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1flockTime"),
            ElementsNode::InputIsPegin => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIsPegin")
            }
            ElementsNode::InputPrevOutpoint => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputPrevOutpoint")
            }
            ElementsNode::InputAsset => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputAsset")
            }
            ElementsNode::InputAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputAmount")
            }
            ElementsNode::InputScriptHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputScriptHash")
            }
            ElementsNode::InputSequence => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputSequence")
            }
            ElementsNode::InputIssuanceBlinding => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceBlinding")
            }
            ElementsNode::InputIssuanceContract => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceContract")
            }
            ElementsNode::InputIssuanceEntropy => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceEntropy")
            }
            ElementsNode::InputIssuanceAssetAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceAssetAmt")
            }
            ElementsNode::InputIssuanceTokenAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceTokenAmt")
            }
            ElementsNode::OutputAsset => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputAsset")
            }
            ElementsNode::OutputAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputAmount")
            }
            ElementsNode::OutputNonce => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputNonce")
            }
            ElementsNode::OutputScriptHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputScriptHash")
            }
            ElementsNode::OutputNullDatum => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputNullDatum")
            }
            ElementsNode::ScriptCmr => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fscriptCMR")
            }
            ElementsNode::CurrentIndex => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIndex")
            }
            ElementsNode::CurrentIsPegin => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIsPegin")
            }
            ElementsNode::CurrentPrevOutpoint => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentPrevOutpoint")
            }
            ElementsNode::CurrentAsset => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentAsset")
            }
            ElementsNode::CurrentAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentAmount")
            }
            ElementsNode::CurrentScriptHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentScriptHash")
            }
            ElementsNode::CurrentSequence => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentSequence")
            }
            ElementsNode::CurrentIssuanceBlinding => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceBlinding")
            }
            ElementsNode::CurrentIssuanceContract => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceContract")
            }
            ElementsNode::CurrentIssuanceEntropy => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceEntropy")
            }
            ElementsNode::CurrentIssuanceAssetAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceAssetAmt")
            }
            ElementsNode::CurrentIssuanceTokenAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceTokenAmt")
            }
            ElementsNode::InputsHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputsHash")
            }
            ElementsNode::OutputsHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputsHash")
            }
            ElementsNode::NumInputs => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fnumInputs")
            }
            ElementsNode::NumOutputs => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fnumOutputs")
            }
            ElementsNode::Fee => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1ffee"),
        }
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

    fn exec(&self, _mac: &mut exec::BitMachine, _txenv: &Self::TxEnv) {
        // FIXME finish this
        unimplemented!()
    }
}
