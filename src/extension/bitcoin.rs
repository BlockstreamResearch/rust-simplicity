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
use bititer::BitIter;
use cmr::Cmr;
use Error;
use {encode, exec, extension};

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
pub enum Node {
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Node::Version => "version",
            Node::LockTime => "locktime",
            Node::InputsHash => "inputshash",
            Node::OutputsHash => "outputshash",
            Node::NumInputs => "numinputs",
            Node::TotalInputValue => "totalinputvalue",
            Node::CurrentPrevOutpoint => "currentprevoutpoint",
            Node::CurrentValue => "currentvalue",
            Node::CurrentSequence => "currentsequence",
            Node::CurrentIndex => "currentindex",
            Node::InputPrevOutpoint => "inputprevoutpoint",
            Node::InputValue => "inputvalue",
            Node::InputSequence => "inputsequence",
            Node::NumOutputs => "numoutputs",
            Node::TotalOutputValue => "totaloutputvalue",
            Node::OutputValue => "outputvalue",
            Node::OutputScriptHash => "outputscripthash",
            Node::ScriptCMR => "scriptcmr",
        })
    }
}

impl extension::Node for Node {
    type TxEnv = TxEnv;

    fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Node, Error> {
        let code = match iter.read_bits_be(4) {
            Some(code) => code,
            None => return Err(Error::EndOfStream),
        };
        match code {
            0 => match iter.next() {
                Some(false) => Ok(Node::Version),
                Some(true) => Ok(Node::LockTime),
                None => Err(Error::EndOfStream),
            },
            1 => Ok(Node::InputsHash),
            2 => Ok(Node::OutputsHash),
            3 => Ok(Node::NumInputs),
            4 => Ok(Node::TotalInputValue),
            5 => Ok(Node::CurrentPrevOutpoint),
            6 => Ok(Node::CurrentValue),
            7 => Ok(Node::CurrentSequence),
            8 => match iter.next() {
                Some(false) => Ok(Node::CurrentIndex),
                Some(true) => Ok(Node::InputPrevOutpoint),
                None => Err(Error::EndOfStream),
            },
            9 => Ok(Node::InputValue),
            10 => Ok(Node::InputSequence),
            11 => Ok(Node::NumOutputs),
            12 => Ok(Node::TotalOutputValue),
            13 => Ok(Node::OutputValue),
            14 => Ok(Node::OutputScriptHash),
            15 => Ok(Node::ScriptCMR),
            _ => unreachable!(),
        }
    }

    fn source_type(&self) -> TypeName {
        match *self {
            Node::Version
            | Node::LockTime
            | Node::InputsHash
            | Node::OutputsHash
            | Node::NumInputs
            | Node::TotalInputValue
            | Node::CurrentPrevOutpoint
            | Node::CurrentValue
            | Node::CurrentSequence
            | Node::CurrentIndex => TypeName(b"1"),
            Node::InputPrevOutpoint | Node::InputValue | Node::InputSequence => TypeName(b"i"),
            Node::NumOutputs | Node::TotalOutputValue => TypeName(b"1"),
            Node::OutputValue | Node::OutputScriptHash => TypeName(b"i"),
            Node::ScriptCMR => TypeName(b"1"),
        }
    }

    /// Name of the target type for this node
    fn target_type(&self) -> TypeName {
        match *self {
            Node::Version => TypeName(b"i"),
            Node::LockTime => TypeName(b"i"),
            Node::InputsHash => TypeName(b"h"),
            Node::OutputsHash => TypeName(b"h"),
            Node::NumInputs => TypeName(b"i"),
            Node::TotalInputValue => TypeName(b"l"),
            Node::CurrentPrevOutpoint => TypeName(b"*hi"),
            Node::CurrentValue => TypeName(b"l"),
            Node::CurrentSequence => TypeName(b"i"),
            Node::CurrentIndex => TypeName(b"i"),
            Node::InputPrevOutpoint => TypeName(b"+1*hi"),
            Node::InputValue => TypeName(b"+1l"),
            Node::InputSequence => TypeName(b"+1i"),
            Node::NumOutputs => TypeName(b"i"),
            Node::TotalOutputValue => TypeName(b"l"),
            Node::OutputValue => TypeName(b"+1l"),
            Node::OutputScriptHash => TypeName(b"+1h"),
            Node::ScriptCMR => TypeName(b"h"),
        }
    }

    fn cmr(&self) -> Cmr {
        match *self {
            Node::Version => Cmr::new(b"SimplicityPrimitiveBitcoin\x1fversion"),
            Node::LockTime => Cmr::new(b"SimplicityPrimitiveBitcoin\x1flockTime"),
            Node::InputsHash => Cmr::new(b"SimplicityPrimitiveBitcoin\x1finputsHash"),
            Node::OutputsHash => Cmr::new(b"SimplicityPrimitiveBitcoinx1foutputsHash"),
            Node::NumInputs => Cmr::new(b"SimplicityPrimitiveBitcoinx1fnumInputs"),
            Node::TotalInputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1ftotalInputValue"),
            Node::CurrentPrevOutpoint => {
                Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentPrevOutpoint")
            }
            Node::CurrentValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentValue"),
            Node::CurrentSequence => Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentSequence"),
            Node::CurrentIndex => Cmr::new(b"SimplicityPrimitiveBitcoinx1fcurrentIndex"),
            Node::InputPrevOutpoint => Cmr::new(b"SimplicityPrimitiveBitcoinx1finputPrevOutpoint"),
            Node::InputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1finputValue"),
            Node::InputSequence => Cmr::new(b"SimplicityPrimitiveBitcoinx1finputSequence"),
            Node::NumOutputs => Cmr::new(b"SimplicityPrimitiveBitcoinx1fnumOutputs"),
            Node::TotalOutputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1ftotalOutputValue"),
            Node::OutputValue => Cmr::new(b"SimplicityPrimitiveBitcoinx1foutputValue"),
            Node::OutputScriptHash => Cmr::new(b"SimplicityPrimitiveBitcoinx1foutputScriptHash"),
            Node::ScriptCMR => Cmr::new(b"SimplicityPrimitiveBitcoinx1fscriptCMR"),
        }
    }

    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize> {
        match *self {
            Node::Version => w.write_u8(64 + 0, 7),
            Node::LockTime => w.write_u8(64 + 1, 7),
            Node::InputsHash => w.write_u8(32 + 1, 6),
            Node::OutputsHash => w.write_u8(32 + 2, 6),
            Node::NumInputs => w.write_u8(32 + 3, 6),
            Node::TotalInputValue => w.write_u8(32 + 4, 6),
            Node::CurrentPrevOutpoint => w.write_u8(32 + 5, 6),
            Node::CurrentValue => w.write_u8(32 + 6, 6),
            Node::CurrentSequence => w.write_u8(32 + 7, 6),
            Node::CurrentIndex => w.write_u8(64 + 16, 7),
            Node::InputPrevOutpoint => w.write_u8(64 + 17, 7),
            Node::InputValue => w.write_u8(32 + 9, 6),
            Node::InputSequence => w.write_u8(32 + 10, 6),
            Node::NumOutputs => w.write_u8(32 + 11, 6),
            Node::TotalOutputValue => w.write_u8(32 + 12, 6),
            Node::OutputValue => w.write_u8(32 + 13, 6),
            Node::OutputScriptHash => w.write_u8(32 + 14, 6),
            Node::ScriptCMR => w.write_u8(32 + 15, 6),
        }
    }

    fn exec(&self, mac: &mut exec::BitMachine, txenv: &Self::TxEnv) {
        // FIXME finish this
        match *self {
            Node::InputsHash => {
                let mut eng = sha256::Hash::engine();
                for input in &txenv.tx.input {
                    eng.input(&input.previous_output.txid[..]);
                    eng.write_u32::<LittleEndian>(input.previous_output.vout)
                        .unwrap();
                    eng.write_u64::<LittleEndian>(99998000).unwrap(); // value FIXME
                    eng.write_u32::<LittleEndian>(input.sequence).unwrap();
                }
                mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
            }
            Node::OutputsHash => {
                let mut eng = sha256::Hash::engine();
                for output in &txenv.tx.output {
                    eng.write_u64::<LittleEndian>(output.value).unwrap();
                    eng.input(&sha256::Hash::hash(&output.script_pubkey[..]));
                }
                mac.write_bytes(&sha256::Hash::from_engine(eng)[..]);
            }
            // FIXME don't hardcode this
            Node::CurrentValue => {
                mac.write_u64(99998000);
            }
            Node::CurrentIndex => {
                mac.write_u32(0);
            }
            Node::LockTime => {
                mac.write_u32(txenv.tx.lock_time);
            }
            Node::Version => {
                mac.write_u32(txenv.tx.version);
            }
            ref b => unimplemented!("bitcoin {}", b),
        }
    }
}
