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

//! # Bitcoin Extensions
//!
//! Extensions to the Simplicity language to allow use on the Bitcoin
//! blockchain
//!

use bitcoin_hashes::{sha256, Hash, HashEngine};
use byteorder::{LittleEndian, WriteBytesExt};
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
    tx: bitcoin::Transaction,
}

impl TxEnv {
    /// Constructor from a transaction
    pub fn from_tx(tx: bitcoin::Transaction) -> TxEnv {
        TxEnv { tx: tx }
    }
}

/// Set of new Simplicity nodes enabled by the Bitcoin extension
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BtcNode {
    Version,
    LockTime,
    InputsHash,
    OutputsHash,
    NumInputs,
    TotalInputValue,
    CurrentPrevOutpoint,
    CurrentValue,
    CurrentSequence,
    CurrentIndex,
    InputPrevOutpoint,
    InputValue,
    InputSequence,
    NumOutputs,
    TotalOutputValue,
    OutputValue,
    OutputScriptHash,
    ScriptCMR,
}

impl fmt::Display for BtcNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            BtcNode::Version => "version",
            BtcNode::LockTime => "locktime",
            BtcNode::InputsHash => "inputshash",
            BtcNode::OutputsHash => "outputshash",
            BtcNode::NumInputs => "numinputs",
            BtcNode::TotalInputValue => "totalinputvalue",
            BtcNode::CurrentPrevOutpoint => "currentprevoutpoint",
            BtcNode::CurrentValue => "currentvalue",
            BtcNode::CurrentSequence => "currentsequence",
            BtcNode::CurrentIndex => "currentindex",
            BtcNode::InputPrevOutpoint => "inputprevoutpoint",
            BtcNode::InputValue => "inputvalue",
            BtcNode::InputSequence => "inputsequence",
            BtcNode::NumOutputs => "numoutputs",
            BtcNode::TotalOutputValue => "totaloutputvalue",
            BtcNode::OutputValue => "outputvalue",
            BtcNode::OutputScriptHash => "outputscripthash",
            BtcNode::ScriptCMR => "scriptcmr",
        })
    }
}

impl extension::Jet for BtcNode {
    type TxEnv = TxEnv;

    fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<BtcNode, Error> {
        let code = match iter.read_bits_be(4) {
            Some(code) => code,
            None => return Err(Error::EndOfStream),
        };
        match code {
            0 => match iter.next() {
                Some(false) => Ok(BtcNode::Version),
                Some(true) => Ok(BtcNode::LockTime),
                None => Err(Error::EndOfStream),
            },
            1 => Ok(BtcNode::InputsHash),
            2 => Ok(BtcNode::OutputsHash),
            3 => Ok(BtcNode::NumInputs),
            4 => Ok(BtcNode::TotalInputValue),
            5 => Ok(BtcNode::CurrentPrevOutpoint),
            6 => Ok(BtcNode::CurrentValue),
            7 => Ok(BtcNode::CurrentSequence),
            8 => match iter.next() {
                Some(false) => Ok(BtcNode::CurrentIndex),
                Some(true) => Ok(BtcNode::InputPrevOutpoint),
                None => Err(Error::EndOfStream),
            },
            9 => Ok(BtcNode::InputValue),
            10 => Ok(BtcNode::InputSequence),
            11 => Ok(BtcNode::NumOutputs),
            12 => Ok(BtcNode::TotalOutputValue),
            13 => Ok(BtcNode::OutputValue),
            14 => Ok(BtcNode::OutputScriptHash),
            15 => Ok(BtcNode::ScriptCMR),
            _ => unreachable!(),
        }
    }

    fn source_type(&self) -> TypeName {
        match *self {
            BtcNode::Version
            | BtcNode::LockTime
            | BtcNode::InputsHash
            | BtcNode::OutputsHash
            | BtcNode::NumInputs
            | BtcNode::TotalInputValue
            | BtcNode::CurrentPrevOutpoint
            | BtcNode::CurrentValue
            | BtcNode::CurrentSequence
            | BtcNode::CurrentIndex => TypeName(b"1"),
            BtcNode::InputPrevOutpoint | BtcNode::InputValue | BtcNode::InputSequence => {
                TypeName(b"i")
            }
            BtcNode::NumOutputs | BtcNode::TotalOutputValue => TypeName(b"1"),
            BtcNode::OutputValue | BtcNode::OutputScriptHash => TypeName(b"i"),
            BtcNode::ScriptCMR => TypeName(b"1"),
        }
    }

    /// Name of the target type for this node
    fn target_type(&self) -> TypeName {
        match *self {
            BtcNode::Version => TypeName(b"i"),
            BtcNode::LockTime => TypeName(b"i"),
            BtcNode::InputsHash => TypeName(b"h"),
            BtcNode::OutputsHash => TypeName(b"h"),
            BtcNode::NumInputs => TypeName(b"i"),
            BtcNode::TotalInputValue => TypeName(b"l"),
            BtcNode::CurrentPrevOutpoint => TypeName(b"*hi"),
            BtcNode::CurrentValue => TypeName(b"l"),
            BtcNode::CurrentSequence => TypeName(b"i"),
            BtcNode::CurrentIndex => TypeName(b"i"),
            BtcNode::InputPrevOutpoint => TypeName(b"+1*hi"),
            BtcNode::InputValue => TypeName(b"+1l"),
            BtcNode::InputSequence => TypeName(b"+1i"),
            BtcNode::NumOutputs => TypeName(b"i"),
            BtcNode::TotalOutputValue => TypeName(b"l"),
            BtcNode::OutputValue => TypeName(b"+1l"),
            BtcNode::OutputScriptHash => TypeName(b"+1h"),
            BtcNode::ScriptCMR => TypeName(b"h"),
        }
    }

    fn cmr(&self) -> Cmr {
        match *self {
            BtcNode::Version => Cmr::new(b"SimplicityPrimitiveBitcoin\x1fversion"),
            BtcNode::LockTime => Cmr::new(b"SimplicityPrimitiveBitcoin\x1flockTime"),
            BtcNode::InputsHash => Cmr::new(b"SimplicityPrimitiveBitcoin\x1finputsHash"),
            BtcNode::OutputsHash => Cmr::new(b"SimplicityPrimitiveBitcoinx1foutputsHash"),
            BtcNode::NumInputs => Cmr::new(b"SimplicityPrimitiveBitcoinx1fnumInputs"),
            BtcNode::TotalInputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1ftotalInputValue"),
            BtcNode::CurrentPrevOutpoint => {
                Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentPrevOutpoint")
            }
            BtcNode::CurrentValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentValue"),
            BtcNode::CurrentSequence => Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentSequence"),
            BtcNode::CurrentIndex => Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentIndex"),
            BtcNode::InputPrevOutpoint => {
                Cmr::new(b"SimplicityPrimitiveBitcoinx1finputPrevOutpoint")
            }
            BtcNode::InputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1finputValue"),
            BtcNode::InputSequence => Cmr::new(b"SimplicityPrimitiveBitcoinx1finputSequence"),
            BtcNode::NumOutputs => Cmr::new(b"SimplicityPrimitiveBitcoinx1fnumOutputs"),
            BtcNode::TotalOutputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1ftotalOutputValue"),
            BtcNode::OutputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1foutputValue"),
            BtcNode::OutputScriptHash => Cmr::new(b"SimplicityPrimitiveBitcoinx1foutputScriptHash"),
            BtcNode::ScriptCMR => Cmr::new(b"SimplicityPrimitiveBitcoinx1fscriptCMR"),
        }
    }

    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize> {
        match *self {
            BtcNode::Version => w.write_u8(64 + 0, 7),
            BtcNode::LockTime => w.write_u8(64 + 1, 7),
            BtcNode::InputsHash => w.write_u8(32 + 1, 6),
            BtcNode::OutputsHash => w.write_u8(32 + 2, 6),
            BtcNode::NumInputs => w.write_u8(32 + 3, 6),
            BtcNode::TotalInputValue => w.write_u8(32 + 4, 6),
            BtcNode::CurrentPrevOutpoint => w.write_u8(32 + 5, 6),
            BtcNode::CurrentValue => w.write_u8(32 + 6, 6),
            BtcNode::CurrentSequence => w.write_u8(32 + 7, 6),
            BtcNode::CurrentIndex => w.write_u8(64 + 16, 7),
            BtcNode::InputPrevOutpoint => w.write_u8(64 + 17, 7),
            BtcNode::InputValue => w.write_u8(32 + 9, 6),
            BtcNode::InputSequence => w.write_u8(32 + 10, 6),
            BtcNode::NumOutputs => w.write_u8(32 + 11, 6),
            BtcNode::TotalOutputValue => w.write_u8(32 + 12, 6),
            BtcNode::OutputValue => w.write_u8(32 + 13, 6),
            BtcNode::OutputScriptHash => w.write_u8(32 + 14, 6),
            BtcNode::ScriptCMR => w.write_u8(32 + 15, 6),
        }
    }

    fn exec(&self, mac: &mut exec::BitMachine, txenv: &Self::TxEnv) {
        // FIXME finish this
        match *self {
            BtcNode::InputsHash => {
                let mut eng = sha256::Hash::engine();
                for input in &txenv.tx.input {
                    eng.input(&input.previous_output.txid[..]);
                    eng.write_u32::<LittleEndian>(input.previous_output.vout)
                        .unwrap();
                    eng.write_u64::<LittleEndian>(99_998_000).unwrap(); // value FIXME
                    eng.write_u32::<LittleEndian>(input.sequence).unwrap();
                }
                mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
            }
            BtcNode::OutputsHash => {
                let mut eng = sha256::Hash::engine();
                for output in &txenv.tx.output {
                    eng.write_u64::<LittleEndian>(output.value).unwrap();
                    eng.input(&sha256::Hash::hash(&output.script_pubkey[..]));
                }
                mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
            }
            // FIXME don't hardcode this
            BtcNode::CurrentValue => {
                mac.write_u64(99_998_000);
            }
            BtcNode::CurrentIndex => {
                mac.write_u32(0);
            }
            BtcNode::LockTime => {
                mac.write_u32(txenv.tx.lock_time);
            }
            BtcNode::Version => {
                mac.write_u32(txenv.tx.version);
            }
            ref b => unimplemented!("bitcoin {}", b),
        }
    }
}
