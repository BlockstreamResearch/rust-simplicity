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

use std::{error, fmt, io};

use super::{ExtError, TypeName};
use crate::bitcoin_hashes::{sha256, Hash, HashEngine};
use crate::bititer::BitIter;
use crate::cmr::Cmr;
use crate::encode;
use crate::exec;
use crate::extension;
use crate::Error;

/// Set of new Simplicity nodes enabled by the Bitcoin extension
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum JetsNode {
    Adder32,
    FullAdder32,
    Subtractor32,
    FullSubtractor32,
    Multiplier32,
    FullMultiplier32,
    Sha256HashBlock,
    SchnorrAssert,
    // Temparory jets for compiler
    EqV256,
    Sha256,
    LessThanV32, // less than verify for u32
    EqV32,
}

impl fmt::Display for JetsNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            JetsNode::Adder32 => "adder32",
            JetsNode::FullAdder32 => "fulladder32",
            JetsNode::Subtractor32 => "subtractor32",
            JetsNode::FullSubtractor32 => "fullsubtractor32",
            JetsNode::Multiplier32 => "multiplier32",
            JetsNode::FullMultiplier32 => "fullmultiplier32",
            JetsNode::Sha256HashBlock => "sha256hashblock",
            JetsNode::SchnorrAssert => "schnorrassert",
            JetsNode::EqV256 => "eqv256",
            JetsNode::Sha256 => "sha256",
            JetsNode::LessThanV32 => "le32",
            JetsNode::EqV32 => "eqv32",
        })
    }
}

impl extension::Jet for JetsNode {
    type TxEnv = ();
    type JetErr = ArithJetErr;
    /// Name of the source type for this node
    fn source_type(&self) -> TypeName {
        match *self {
            JetsNode::Adder32 => TypeName(b"l"),
            JetsNode::FullAdder32 => TypeName(b"*l2"),
            JetsNode::Subtractor32 => TypeName(b"l"),
            JetsNode::FullSubtractor32 => TypeName(b"*l2"),
            JetsNode::Multiplier32 => TypeName(b"l"),
            JetsNode::FullMultiplier32 => TypeName(b"*ll"),
            JetsNode::Sha256HashBlock => TypeName(b"*h*hh"),
            JetsNode::SchnorrAssert => TypeName(b"*h*hh"),
            JetsNode::EqV256 => TypeName(b"*hh"),
            JetsNode::Sha256 => TypeName(b"*hh"),
            JetsNode::LessThanV32 => TypeName(b"l"),
            JetsNode::EqV32 => TypeName(b"l"),
        }
    }

    /// Name of the target type for this node
    fn target_type(&self) -> TypeName {
        match *self {
            JetsNode::Adder32 => TypeName(b"*2i"),
            JetsNode::FullAdder32 => TypeName(b"*2i"),
            JetsNode::Subtractor32 => TypeName(b"*2i"),
            JetsNode::FullSubtractor32 => TypeName(b"*2i"),
            JetsNode::Multiplier32 => TypeName(b"l"),
            JetsNode::FullMultiplier32 => TypeName(b"l"),
            JetsNode::Sha256HashBlock => TypeName(b"h"),
            JetsNode::SchnorrAssert => TypeName(b"1"),
            JetsNode::EqV256 => TypeName(b"1"),
            JetsNode::Sha256 => TypeName(b"h"),
            JetsNode::LessThanV32 => TypeName(b"1"),
            JetsNode::EqV32 => TypeName(b"1"),
        }
    }

    /// CMR for this node
    fn cmr(&self) -> Cmr {
        let cmr = Cmr::new(b"Simplicity-Draft\x1fJet");
        match *self {
            JetsNode::Adder32 => cmr.update_1(Cmr::from([
                0x6c, 0xd6, 0x15, 0x93, 0xe8, 0xe2, 0x43, 0xe5, 0xb7, 0x54, 0x4d, 0x2a, 0x12, 0x93,
                0x15, 0xa0, 0x19, 0x2d, 0xc5, 0x66, 0xb2, 0xd8, 0x09, 0x26, 0x33, 0x3b, 0x7e, 0xda,
                0x5c, 0x37, 0x79, 0x78,
            ])),
            JetsNode::FullAdder32 => cmr.update_1(Cmr::from([
                0x15, 0x4f, 0x44, 0x0c, 0x3b, 0xb4, 0x66, 0x3c, 0xfb, 0xf7, 0x95, 0xdc, 0xde, 0x41,
                0x0b, 0x09, 0x22, 0x52, 0x85, 0x82, 0x5f, 0x3b, 0x3e, 0x6f, 0xab, 0x36, 0x45, 0x69,
                0x64, 0x3f, 0x50, 0xeb,
            ])),
            JetsNode::Subtractor32 => cmr.update_1(Cmr::from([
                0x90, 0xdd, 0x60, 0x22, 0xe8, 0x6b, 0xf2, 0xe9, 0x67, 0x9d, 0x63, 0x16, 0x78, 0x6d,
                0x5e, 0x97, 0x4a, 0x42, 0xbe, 0x02, 0xda, 0x96, 0x35, 0x4c, 0x0b, 0xa1, 0xae, 0x6b,
                0x76, 0x2b, 0x2e, 0x65,
            ])),
            JetsNode::FullSubtractor32 => cmr.update_1(Cmr::from([
                0x52, 0x44, 0xcb, 0xda, 0xe2, 0xdc, 0xbd, 0x1a, 0x9c, 0xcf, 0x93, 0xad, 0x9d, 0xf9,
                0x02, 0xce, 0x00, 0xde, 0x3e, 0xc5, 0xfe, 0x16, 0x3b, 0xe4, 0x1e, 0x1f, 0xd5, 0xbc,
                0xe4, 0xdb, 0x6d, 0x9b,
            ])),
            JetsNode::Multiplier32 => cmr.update_1(Cmr::from([
                0x23, 0x83, 0xd3, 0x01, 0x3c, 0x15, 0xf7, 0x48, 0xd8, 0xaa, 0xa8, 0xb6, 0xcb, 0x4b,
                0xff, 0x29, 0xfe, 0x46, 0x45, 0xc8, 0x5a, 0x34, 0xe2, 0x7b, 0xa8, 0x5c, 0x52, 0xd5,
                0xc8, 0x8e, 0x5e, 0xc3,
            ])),
            JetsNode::FullMultiplier32 => cmr.update_1(Cmr::from([
                0x40, 0xd2, 0xd4, 0x61, 0x8b, 0x84, 0x4f, 0xfc, 0x56, 0x2f, 0xbe, 0xf6, 0x9e, 0x01,
                0xbd, 0x91, 0x47, 0x1b, 0xe4, 0xd9, 0x86, 0x38, 0xc6, 0xb5, 0xe2, 0xde, 0xea, 0x23,
                0xd5, 0x83, 0xe2, 0xf5,
            ])),
            JetsNode::Sha256HashBlock => cmr.update_1(Cmr::from([
                0x59, 0x3b, 0x9d, 0xf9, 0x72, 0x7f, 0xe2, 0x98, 0x66, 0xda, 0x10, 0x4c, 0x93, 0x32,
                0x26, 0x16, 0x72, 0xc0, 0x95, 0xe6, 0x77, 0xd0, 0x00, 0x01, 0x99, 0x78, 0x56, 0x74,
                0x04, 0x47, 0x6d, 0xd8,
            ])),
            JetsNode::SchnorrAssert => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9b, //only last `a` changed to `b` from sha2 block cmr
            ])),
            JetsNode::EqV256 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9c, //only last `a` changed to `c` from sha2 block cmr
            ])),
            JetsNode::Sha256 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9d, //only last `a` changed to `d` from sha2 block cmr
            ])),
            JetsNode::LessThanV32 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9e, //only last `a` changed to `e` from sha2 block cmr
            ])),
            JetsNode::EqV32 => cmr.update_1(Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38,
                0xb0, 0x89, 0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d,
                0x31, 0xba, 0xec, 0x9f, //only last `a` changed to `f` from sha2 block cmr
            ])),
        }
    }

    fn wmr(&self) -> Cmr {
        self.cmr()
    }

    /// Encode the node into a bitstream
    fn encode<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize> {
        match *self {
            JetsNode::Adder32 => w.write_u8(48 + 0, 6),
            JetsNode::Subtractor32 => w.write_u8(48 + 1, 6),
            JetsNode::Multiplier32 => w.write_u8(24 + 1, 5),
            JetsNode::FullAdder32 => w.write_u8(48 + 4, 6),
            JetsNode::FullSubtractor32 => w.write_u8(48 + 5, 6),
            JetsNode::FullMultiplier32 => w.write_u8(24 + 3, 5),
            JetsNode::Sha256HashBlock => w.write_u8(14, 4),
            JetsNode::SchnorrAssert => w.write_u8(15 * 16 + 0, 8),
            JetsNode::EqV256 => w.write_u8(15 * 16 + 1, 8),
            JetsNode::Sha256 => w.write_u8(15 * 16 + 2, 8),
            JetsNode::LessThanV32 => w.write_u8(15 * 16 + 3, 8),
            JetsNode::EqV32 => w.write_u8(15 * 16 + 4, 8),
        }
    }

    /// Decode a natural number according to section 7.2.1
    /// of the Simplicity whitepaper. Assumes that a 11 has
    /// already been read from the stream
    fn decode<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<Self, Error> {
        match iter.next() {
            Some(false) => {
                let code = match iter.read_bits_be(2) {
                    Some(code) => code,
                    None => return Err(Error::EndOfStream),
                };
                match code {
                    0 => match iter.next() {
                        Some(false) => Ok(JetsNode::Adder32),
                        Some(true) => Ok(JetsNode::Subtractor32),
                        None => Err(Error::EndOfStream),
                    },
                    1 => Ok(JetsNode::Multiplier32),
                    2 => match iter.next() {
                        Some(false) => Ok(JetsNode::FullAdder32),
                        Some(true) => Ok(JetsNode::FullSubtractor32),
                        None => Err(Error::EndOfStream),
                    },
                    3 => Ok(JetsNode::FullMultiplier32),
                    _ => unreachable!(),
                }
            }
            Some(true) => match iter.next() {
                Some(false) => Ok(JetsNode::Sha256HashBlock),
                Some(true) => {
                    // Some custom jets for fast developement
                    // FIXME: Get a consensus for encoding with Rusell
                    let code = match iter.read_bits_be(4) {
                        Some(code) => code,
                        None => return Err(Error::EndOfStream),
                    };
                    match code {
                        0 => Ok(JetsNode::SchnorrAssert),
                        1 => Ok(JetsNode::EqV256),
                        2 => Ok(JetsNode::Sha256),
                        3 => Ok(JetsNode::LessThanV32),
                        4 => Ok(JetsNode::EqV32),
                        _ => Err(Error::ParseError("bad jet")),
                    }
                }
                None => Err(Error::EndOfStream),
            },
            None => Err(Error::EndOfStream),
        }
    }

    fn exec(&self, mac: &mut exec::BitMachine, _tx_env: &Self::TxEnv) -> Result<(), Self::JetErr> {
        match *self {
            JetsNode::Adder32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let (res, overflow) = a.overflowing_add(b);
                mac.write_bit(overflow);
                mac.write_u32(res);
            }
            JetsNode::FullAdder32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let carry = mac.read_bit();
                let (res, overflow_1) = a.overflowing_add(b);
                let (res, overflow_2) = res.overflowing_add(carry as u32);
                mac.write_bit(overflow_1 || overflow_2);
                mac.write_u32(res);
            }
            JetsNode::Subtractor32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let (res, overflow) = a.overflowing_sub(b);
                mac.write_bit(overflow);
                mac.write_u32(res);
            }
            JetsNode::FullSubtractor32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();
                let carry = mac.read_bit();
                let (res, overflow_1) = a.overflowing_sub(b);
                let (res, overflow_2) = res.overflowing_sub(carry as u32);
                mac.write_bit(overflow_1 || overflow_2);
                mac.write_u32(res);
            }
            JetsNode::Multiplier32 => {
                let a = mac.read_u32() as u64;
                let b = mac.read_u32() as u64;
                mac.write_u64(a * b);
            }
            JetsNode::FullMultiplier32 => {
                let a = mac.read_u32() as u64;
                let b = mac.read_u32() as u64;
                let c = mac.read_u32() as u64;
                let d = mac.read_u32() as u64;
                mac.write_u64(a * b + c + d);
            }
            JetsNode::Sha256HashBlock => {
                let hash = mac.read_32bytes();
                let block = mac.read_bytes(64);
                let sha2_midstate = sha256::Midstate::from_inner(hash);
                let mut engine = sha256::HashEngine::from_midstate(sha2_midstate, 0);
                engine.input(&block);
                let h = engine.midstate();
                mac.write_bytes(&h.into_inner());
            }
            JetsNode::SchnorrAssert => {
                let _pubkey = mac.read_32bytes();
                let _sig = mac.read_bytes(64);
                //Check the signature here later
            }
            JetsNode::EqV256 => {
                let a = mac.read_32bytes();
                let b = mac.read_32bytes();

                // FIXME:
                // Get Error here instead of assert
                assert!(a == b);
            }
            JetsNode::Sha256 => {
                let data = mac.read_32bytes();
                let h = sha256::Hash::hash(&data);

                mac.write_bytes(&h);
            }
            JetsNode::LessThanV32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();

                // FIXME: error
                assert!(a < b);
            }
            JetsNode::EqV32 => {
                let a = mac.read_u32();
                let b = mac.read_u32();

                // FIXME: error
                assert!(a == b);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArithJetErr {}

impl fmt::Display for ArithJetErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO in a later commit")
    }
}

impl error::Error for ArithJetErr {}
impl ExtError for ArithJetErr {}
