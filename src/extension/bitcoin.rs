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
use std::error;
use std::{fmt, io};

use super::ExtError;
use super::TypeName;
use crate::bititer::BitIter;
use crate::encode::BitWriter;
use crate::exec;
use crate::extension;
use crate::merkle::cmr::Cmr;
use crate::merkle::common::MerkleRoot;
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

impl Default for TxEnv {
    fn default() -> TxEnv {
        // FIXME: Review and check if the defaults make sense
        TxEnv::from_tx(bitcoin::Transaction {
            version: 2,
            lock_time: 0,
            input: vec![],
            output: vec![],
        })
    }
}

/// Set of new Simplicity nodes enabled by the Bitcoin extension
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
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
    type JetErr = BtcJetErr;

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
            BtcNode::Version => Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fversion"),
            BtcNode::LockTime => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1flockTime")
            }
            BtcNode::InputsHash => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1finputsHash")
            }
            BtcNode::OutputsHash => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1foutputsHash")
            }
            BtcNode::NumInputs => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fnumInputs")
            }
            BtcNode::TotalInputValue => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1ftotalInputValue")
            }
            BtcNode::CurrentPrevOutpoint => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fcurrentPrevOutpoint")
            }
            BtcNode::CurrentValue => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fcurrentValue")
            }
            BtcNode::CurrentSequence => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fcurrentSequence")
            }
            BtcNode::CurrentIndex => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fcurrentIndex")
            }
            BtcNode::InputPrevOutpoint => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1finputPrevOutpoint")
            }
            BtcNode::InputValue => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1finputValue")
            }
            BtcNode::InputSequence => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1finputSequence")
            }
            BtcNode::NumOutputs => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fnumOutputs")
            }
            BtcNode::TotalOutputValue => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1ftotalOutputValue")
            }
            BtcNode::OutputValue => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1foutputValue")
            }
            BtcNode::OutputScriptHash => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1foutputScriptHash")
            }
            BtcNode::ScriptCMR => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fPrimitive\x1fBitcoin\x1fscriptCMR")
            }
        }
    }

    fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        match *self {
            BtcNode::Version => w.write_bits_be(64 + 0, 7),
            BtcNode::LockTime => w.write_bits_be(64 + 1, 7),
            BtcNode::InputsHash => w.write_bits_be(32 + 1, 6),
            BtcNode::OutputsHash => w.write_bits_be(32 + 2, 6),
            BtcNode::NumInputs => w.write_bits_be(32 + 3, 6),
            BtcNode::TotalInputValue => w.write_bits_be(32 + 4, 6),
            BtcNode::CurrentPrevOutpoint => w.write_bits_be(32 + 5, 6),
            BtcNode::CurrentValue => w.write_bits_be(32 + 6, 6),
            BtcNode::CurrentSequence => w.write_bits_be(32 + 7, 6),
            BtcNode::CurrentIndex => w.write_bits_be(64 + 16, 7),
            BtcNode::InputPrevOutpoint => w.write_bits_be(64 + 17, 7),
            BtcNode::InputValue => w.write_bits_be(32 + 9, 6),
            BtcNode::InputSequence => w.write_bits_be(32 + 10, 6),
            BtcNode::NumOutputs => w.write_bits_be(32 + 11, 6),
            BtcNode::TotalOutputValue => w.write_bits_be(32 + 12, 6),
            BtcNode::OutputValue => w.write_bits_be(32 + 13, 6),
            BtcNode::OutputScriptHash => w.write_bits_be(32 + 14, 6),
            BtcNode::ScriptCMR => w.write_bits_be(32 + 15, 6),
        }
    }

    fn exec(&self, mac: &mut exec::BitMachine, txenv: &Self::TxEnv) -> Result<(), Self::JetErr> {
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
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BtcJetErr {}

impl fmt::Display for BtcJetErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO in a later commit")
    }
}

impl error::Error for BtcJetErr {}
impl ExtError for BtcJetErr {}
