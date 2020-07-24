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
pub enum Node {
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Node::Version => "version",
            Node::LockTime => "locktime",
            Node::InputIsPegin => "inputispegin",
            Node::InputPrevOutpoint => "inputprevoutpoint",
            Node::InputAsset => "inputasset",
            Node::InputAmount => "inputamount",
            Node::InputScriptHash => "inputscripthash",
            Node::InputSequence => "inputsequence",
            Node::InputIssuanceBlinding => "inputissuanceblinding",
            Node::InputIssuanceContract => "inputissuancecontract",
            Node::InputIssuanceEntropy => "inputissuanceentropy",
            Node::InputIssuanceAssetAmount => "inputissuanceassetamount",
            Node::InputIssuanceTokenAmount => "inputissuancetokenamount",
            Node::OutputAsset => "outputasset",
            Node::OutputAmount => "outputamount",
            Node::OutputNonce => "outputnonce",
            Node::OutputScriptHash => "outputscripthash",
            Node::OutputNullDatum => "outputnulldatum",
            Node::ScriptCmr => "scriptcmr",
            Node::CurrentIndex => "currentIndex",
            Node::CurrentIsPegin => "currentIspegin",
            Node::CurrentPrevOutpoint => "currentprevoutpoint",
            Node::CurrentAsset => "currentasset",
            Node::CurrentAmount => "currentamount",
            Node::CurrentScriptHash => "currentscripthash",
            Node::CurrentSequence => "currentsequence",
            Node::CurrentIssuanceBlinding => "currentissuanceblinding",
            Node::CurrentIssuanceContract => "currentissuancecontract",
            Node::CurrentIssuanceEntropy => "currentissuanceentropy",
            Node::CurrentIssuanceAssetAmount => "currentissuanceassetAmount",
            Node::CurrentIssuanceTokenAmount => "currentissuancetokenAmount",
            Node::InputsHash => "inputshash",
            Node::OutputsHash => "outputshash",
            Node::NumInputs => "numinputs",
            Node::NumOutputs => "numoutputs",
            Node::Fee => "fee",
        })
    }
}

impl extension::Node for Node {
    type TxEnv = TxEnv;

    fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Node, Error> {
        let code = match iter.read_bits_be(5) {
            Some(code) => code,
            None => return Err(Error::EndOfStream),
        };
        match code {
            0 => match iter.next() {
                Some(false) => Ok(Node::Version),
                Some(true) => Ok(Node::LockTime),
                None => Err(Error::EndOfStream),
            },
            1 => Ok(Node::InputIsPegin),
            2 => Ok(Node::InputPrevOutpoint),
            3 => Ok(Node::InputAsset),
            4 => match iter.next() {
                Some(false) => Ok(Node::InputAmount),
                Some(true) => Ok(Node::InputScriptHash),
                None => Err(Error::EndOfStream),
            },
            5 => Ok(Node::InputSequence),
            6 => Ok(Node::InputIssuanceBlinding),
            7 => Ok(Node::InputIssuanceContract),
            8 => match iter.next() {
                Some(false) => Ok(Node::InputIssuanceEntropy),
                Some(true) => Ok(Node::InputIssuanceAssetAmount),
                None => Err(Error::EndOfStream),
            },
            9 => Ok(Node::InputIssuanceTokenAmount),
            10 => Ok(Node::OutputAsset),
            11 => Ok(Node::OutputAmount),
            12 => match iter.next() {
                Some(false) => Ok(Node::OutputNonce),
                Some(true) => Ok(Node::OutputScriptHash),
                None => Err(Error::EndOfStream),
            },
            13 => Ok(Node::OutputNullDatum),
            14 => Ok(Node::ScriptCmr),
            15 => Ok(Node::CurrentIndex),
            16 => Ok(Node::CurrentIsPegin),
            17 => Ok(Node::CurrentPrevOutpoint),
            18 => Ok(Node::CurrentAsset),
            19 => Ok(Node::CurrentAmount),
            20 => Ok(Node::CurrentScriptHash),
            21 => Ok(Node::CurrentSequence),
            22 => Ok(Node::CurrentIssuanceBlinding),
            23 => Ok(Node::CurrentIssuanceContract),
            24 => Ok(Node::CurrentIssuanceEntropy),
            25 => Ok(Node::CurrentIssuanceAssetAmount),
            26 => Ok(Node::CurrentIssuanceTokenAmount),
            27 => Ok(Node::InputsHash),
            28 => Ok(Node::OutputsHash),
            29 => Ok(Node::NumInputs),
            30 => Ok(Node::NumOutputs),
            31 => Ok(Node::Fee),
            _ => unreachable!(),
        }
    }

    fn source_type(&self) -> TypeName {
        match *self {
            Node::Version => TypeName(b"1"),
            Node::LockTime => TypeName(b"1"),
            Node::InputIsPegin
            | Node::InputPrevOutpoint
            | Node::InputAsset
            | Node::InputAmount
            | Node::InputScriptHash
            | Node::InputSequence
            | Node::InputIssuanceBlinding
            | Node::InputIssuanceContract
            | Node::InputIssuanceEntropy
            | Node::InputIssuanceAssetAmount
            | Node::InputIssuanceTokenAmount
            | Node::OutputAsset
            | Node::OutputAmount
            | Node::OutputNonce
            | Node::OutputScriptHash => TypeName(b"i"),
            Node::OutputNullDatum => TypeName(b"*ii"),
            Node::ScriptCmr
            | Node::CurrentIndex
            | Node::CurrentIsPegin
            | Node::CurrentPrevOutpoint
            | Node::CurrentAsset
            | Node::CurrentAmount
            | Node::CurrentScriptHash
            | Node::CurrentSequence
            | Node::CurrentIssuanceBlinding
            | Node::CurrentIssuanceContract
            | Node::CurrentIssuanceEntropy
            | Node::CurrentIssuanceAssetAmount
            | Node::CurrentIssuanceTokenAmount
            | Node::InputsHash
            | Node::OutputsHash
            | Node::NumInputs
            | Node::NumOutputs => TypeName(b"1"),
            Node::Fee => TypeName(b"h"),
        }
    }

    /// Name of the target type for this node
    fn target_type(&self) -> TypeName {
        match *self {
            Node::Version => TypeName(b"i"),
            Node::LockTime => TypeName(b"i"),
            Node::InputIsPegin => TypeName(b"+12"),
            Node::InputPrevOutpoint => TypeName(b"+1*hi"),
            Node::InputAsset => TypeName(b"+1+*2hh"),
            Node::InputAmount => TypeName(b"+1+*2hl"),
            Node::InputScriptHash => TypeName(b"+1h"),
            Node::InputSequence => TypeName(b"+1i"),
            Node::InputIssuanceBlinding => TypeName(b"+1+1h"),
            Node::InputIssuanceContract => TypeName(b"+1+1h"),
            Node::InputIssuanceEntropy => TypeName(b"+1+1h"),
            Node::InputIssuanceAssetAmount => TypeName(b"+1+1+*2hl"),
            Node::InputIssuanceTokenAmount => TypeName(b"+1+1+*2hl"),
            Node::OutputAsset => TypeName(b"+1+*2hh"),
            Node::OutputAmount => TypeName(b"+1+*2hl"),
            Node::OutputNonce => TypeName(b"+1+*2hh"),
            Node::OutputScriptHash => TypeName(b"+1h"),
            Node::OutputNullDatum => TypeName(b"+1+1+**22h+2*22"),
            Node::ScriptCmr => TypeName(b"h"),
            Node::CurrentIndex => TypeName(b"i"),
            Node::CurrentIsPegin => TypeName(b"2"),
            Node::CurrentPrevOutpoint => TypeName(b"*hi"),
            Node::CurrentAsset => TypeName(b"+*2hh"),
            Node::CurrentAmount => TypeName(b"+*2hl"),
            Node::CurrentScriptHash => TypeName(b"h"),
            Node::CurrentSequence => TypeName(b"i"),
            Node::CurrentIssuanceBlinding => TypeName(b"+1h"),
            Node::CurrentIssuanceContract => TypeName(b"+1h"),
            Node::CurrentIssuanceEntropy => TypeName(b"+1h"),
            Node::CurrentIssuanceAssetAmount => TypeName(b"+1+*2hl"),
            Node::CurrentIssuanceTokenAmount => TypeName(b"+1+*2hl"),
            Node::InputsHash => TypeName(b"h"),
            Node::OutputsHash => TypeName(b"h"),
            Node::NumInputs => TypeName(b"i"),
            Node::NumOutputs => TypeName(b"i"),
            Node::Fee => TypeName(b"l"),
        }
    }

    fn cmr(&self) -> Cmr {
        match *self {
            Node::Version => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fversion"),
            Node::LockTime => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1flockTime"),
            Node::InputIsPegin => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIsPegin"),
            Node::InputPrevOutpoint => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputPrevOutpoint")
            }
            Node::InputAsset => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputAsset"),
            Node::InputAmount => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputAmount"),
            Node::InputScriptHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputScriptHash")
            }
            Node::InputSequence => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputSequence")
            }
            Node::InputIssuanceBlinding => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceBlinding")
            }
            Node::InputIssuanceContract => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceContract")
            }
            Node::InputIssuanceEntropy => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceEntropy")
            }
            Node::InputIssuanceAssetAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceAssetAmt")
            }
            Node::InputIssuanceTokenAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputIssuanceTokenAmt")
            }
            Node::OutputAsset => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputAsset"),
            Node::OutputAmount => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputAmount"),
            Node::OutputNonce => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputNonce"),
            Node::OutputScriptHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputScriptHash")
            }
            Node::OutputNullDatum => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputNullDatum")
            }
            Node::ScriptCmr => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fscriptCMR"),
            Node::CurrentIndex => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIndex"),
            Node::CurrentIsPegin => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIsPegin")
            }
            Node::CurrentPrevOutpoint => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentPrevOutpoint")
            }
            Node::CurrentAsset => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentAsset"),
            Node::CurrentAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentAmount")
            }
            Node::CurrentScriptHash => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentScriptHash")
            }
            Node::CurrentSequence => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentSequence")
            }
            Node::CurrentIssuanceBlinding => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceBlinding")
            }
            Node::CurrentIssuanceContract => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceContract")
            }
            Node::CurrentIssuanceEntropy => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceEntropy")
            }
            Node::CurrentIssuanceAssetAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceAssetAmt")
            }
            Node::CurrentIssuanceTokenAmount => {
                Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fcurrentIssuanceTokenAmt")
            }
            Node::InputsHash => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1finputsHash"),
            Node::OutputsHash => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1foutputsHash"),
            Node::NumInputs => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fnumInputs"),
            Node::NumOutputs => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1fnumOutputs"),
            Node::Fee => Cmr::new(b"Simplicity\x1fPrimitive\x1fElements\x1ffee"),
        }
    }

    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize> {
        match *self {
            Node::Version => w.write_u8(128 + 0, 8),
            Node::LockTime => w.write_u8(128 + 1, 8),
            Node::InputIsPegin => w.write_u8(64 + 1, 7),
            Node::InputPrevOutpoint => w.write_u8(64 + 2, 7),
            Node::InputAsset => w.write_u8(64 + 3, 7),
            Node::InputAmount => w.write_u8(128 + 2, 8),
            Node::InputScriptHash => w.write_u8(128 + 3, 8),
            Node::InputSequence => w.write_u8(64 + 5, 7),
            Node::InputIssuanceBlinding => w.write_u8(64 + 6, 7),
            Node::InputIssuanceContract => w.write_u8(64 + 7, 7),
            Node::InputIssuanceEntropy => w.write_u8(128 + 4, 8),
            Node::InputIssuanceAssetAmount => w.write_u8(128 + 5, 8),
            Node::InputIssuanceTokenAmount => w.write_u8(64 + 9, 7),
            Node::OutputAsset => w.write_u8(64 + 10, 7),
            Node::OutputAmount => w.write_u8(64 + 11, 7),
            Node::OutputNonce => w.write_u8(128 + 6, 8),
            Node::OutputScriptHash => w.write_u8(128 + 7, 8),
            Node::OutputNullDatum => w.write_u8(64 + 13, 7),
            Node::ScriptCmr => w.write_u8(64 + 14, 7),
            Node::CurrentIndex => w.write_u8(64 + 15, 7),
            Node::CurrentIsPegin => w.write_u8(64 + 16, 7),
            Node::CurrentPrevOutpoint => w.write_u8(64 + 17, 7),
            Node::CurrentAsset => w.write_u8(64 + 18, 7),
            Node::CurrentAmount => w.write_u8(64 + 19, 7),
            Node::CurrentScriptHash => w.write_u8(64 + 20, 7),
            Node::CurrentSequence => w.write_u8(64 + 21, 7),
            Node::CurrentIssuanceBlinding => w.write_u8(64 + 22, 7),
            Node::CurrentIssuanceContract => w.write_u8(64 + 23, 7),
            Node::CurrentIssuanceEntropy => w.write_u8(64 + 24, 7),
            Node::CurrentIssuanceAssetAmount => w.write_u8(64 + 25, 7),
            Node::CurrentIssuanceTokenAmount => w.write_u8(64 + 26, 7),
            Node::InputsHash => w.write_u8(64 + 27, 7),
            Node::OutputsHash => w.write_u8(64 + 28, 7),
            Node::NumInputs => w.write_u8(64 + 29, 7),
            Node::NumOutputs => w.write_u8(64 + 30, 7),
            Node::Fee => w.write_u8(64 + 31, 7),
        }
    }

    fn exec(&self, _mac: &mut exec::BitMachine, _txenv: &Self::TxEnv) {
        // FIXME finish this
        unimplemented!()
    }
}
