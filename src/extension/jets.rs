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

use std::{fmt, io};

use bititer::BitIter;
use super::TypeName;
use Error;
use cmr::Cmr;
use encode;

/// Set of new Simplicity nodes enabled by the Bitcoin extension
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Node {
    Adder32,
    FullAdder32,
    Subtractor32,
    FullSubtractor32,
    Multiplier32,
    FullMultiplier32,
    Sha256HashBlock,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Node::Adder32 => "adder32",
            Node::FullAdder32 => "fulladder32",
            Node::Subtractor32 => "subtractor32",
            Node::FullSubtractor32 => "fullsubtractor32",
            Node::Multiplier32 => "multiplier32",
            Node::FullMultiplier32 =>  "fullmultiplier32",
            Node::Sha256HashBlock => "sha256hashblock",
        })
    }
}

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper. Assumes that a 11 has
/// already been read from the stream
pub fn decode_node<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<Node, Error> {
    match iter.next() {
        Some(false) => {
            let code = match iter.read_bits_be(2) {
                Some(code) => code,
                None => return Err(Error::EndOfStream),
            };
            match code {
                0 => match iter.next() {
                    Some(false) => Ok(Node::Adder32),
                    Some(true) => Ok(Node::Subtractor32),
                    None => Err(Error::EndOfStream),
                },
                1 => Ok(Node::Multiplier32),
                2 => match iter.next() {
                    Some(false) => Ok(Node::FullAdder32),
                    Some(true) => Ok(Node::FullSubtractor32),
                    None => Err(Error::EndOfStream),
                },
                3 => Ok(Node::FullMultiplier32),
                _ => unreachable!(),
            }
        },
        Some(true) => {
            match iter.next() {
                Some(false) => Ok(Node::Sha256HashBlock),
                Some(true) => Err(Error::ParseError("invalid parse 1111")),
                None => Err(Error::EndOfStream),
            }
        },
        None => Err(Error::EndOfStream),
    }
}

impl Node {
    /// Name of the source type for this node
    pub fn source_type(&self) -> TypeName {
        match *self {
            Node::Adder32 => TypeName::Word64,
            Node::FullAdder32 => TypeName::Word64TimesTwo,
            Node::Subtractor32 => TypeName::Word64,
            Node::FullSubtractor32 => TypeName::Word64TimesTwo,
            Node::Multiplier32 => TypeName::Word64,
            Node::FullMultiplier32 => TypeName::Word128,
            Node::Sha256HashBlock => TypeName::Word256Word512,
        }
    }

    /// Name of the target type for this node
    pub fn target_type(&self) -> TypeName {
        match *self {
            Node::Adder32 => TypeName::TwoTimesWord32,
            Node::FullAdder32 => TypeName::TwoTimesWord32,
            Node::Subtractor32 => TypeName::TwoTimesWord32,
            Node::FullSubtractor32 => TypeName::TwoTimesWord32,
            Node::Multiplier32 => TypeName::Word64,
            Node::FullMultiplier32 => TypeName::Word64,
            Node::Sha256HashBlock => TypeName::Word256,
        }
    }

    /// CMR for this node
    pub fn cmr(&self) -> Cmr {
        match *self {
            Node::Adder32 => Cmr::from([
                0x8e, 0x38, 0x9a, 0x7d, 0x75, 0x42, 0x9a, 0x8a, 0x6f, 0x5b, 0x44, 0x8e, 0xc8, 0xe8, 0x45, 0x85,
                0x20, 0xe2, 0x76, 0xfc, 0x8e, 0x09, 0xef, 0x5a, 0x68, 0xf3, 0xf3, 0x2d, 0x9f, 0xb9, 0x79, 0x35,
            ]),
            Node::FullAdder32 => Cmr::from([
                0xb9, 0x14, 0xe4, 0xb5, 0x9f, 0x8e, 0xde, 0xd4, 0xcd, 0x03, 0x6e, 0x03, 0xff, 0xa5, 0xf1, 0x1a,
                0xa8, 0x66, 0x8a, 0xe4, 0x98, 0x63, 0xbb, 0xb4, 0x3a, 0x0d, 0x7c, 0x3a, 0x14, 0xc9, 0x16, 0xf0,
            ]),
            Node::Subtractor32 => Cmr::from([
                0x75, 0xeb, 0xd5, 0x69, 0xbf, 0xce, 0x7a, 0xf8, 0x03, 0x0c, 0x49, 0xc7, 0x3e, 0x10, 0x4c, 0x03,
                0x65, 0xde, 0x89, 0x8e, 0xa8, 0xd5, 0x26, 0x70, 0xbf, 0xfe, 0x9f, 0x6e, 0x31, 0x2f, 0xf6, 0xe6,
            ]),
            Node::FullSubtractor32 => Cmr::from([
                0x7a, 0x52, 0xe8, 0x3e, 0x25, 0x3a, 0xe7, 0x76, 0xb0, 0xb9, 0x48, 0xf1, 0x50, 0x83, 0x52, 0x8e,
                0x1c, 0x5d, 0x58, 0xcd, 0x5e, 0x03, 0xd4, 0xf2, 0xf0, 0x4a, 0x96, 0x26, 0xe0, 0x47, 0x6a, 0xeb,
            ]),
            Node::Multiplier32 => Cmr::from([
                0x40, 0x59, 0x14, 0xc9, 0x52, 0x4c, 0x48, 0x73, 0xce, 0x5d, 0xdb, 0x06, 0xfd, 0x30, 0xd6, 0xd5,
                0xfc, 0x4a, 0xc1, 0xfa, 0xc0, 0xee, 0xf8, 0xd8, 0x2d, 0xe6, 0xc6, 0x22, 0x7f, 0xb2, 0xd2, 0xcd,
            ]),
            Node::FullMultiplier32 => Cmr::from([
                0x89, 0xa0, 0xae, 0x09, 0x8a, 0xff, 0x5e, 0x9c, 0x40, 0x90, 0x74, 0x47, 0x91, 0xff, 0x5c, 0x8e,
                0xe1, 0x7a, 0x8c, 0xeb, 0x9e, 0x49, 0x42, 0x24, 0xe9, 0x19, 0xde, 0xb1, 0x1c, 0x5b, 0x8a, 0xf4,
            ]),
            Node::Sha256HashBlock => Cmr::from([
                0xee, 0xae, 0x47, 0xe2, 0xf7, 0x87, 0x6c, 0x3b, 0x9c, 0xbc, 0xd4, 0x04, 0xa3, 0x38, 0xb0, 0x89,
                0xfd, 0xea, 0xdf, 0x1b, 0x9b, 0xb3, 0x82, 0xec, 0x6e, 0x69, 0x71, 0x9d, 0x31, 0xba, 0xec, 0x9a,
            ]),
        }
    }

    /// Encode the node into a bitstream
    pub fn encode_node<W: encode::BitWrite>(&self, w: &mut W) -> io::Result<usize> {
        match *self {
            Node::Adder32 => w.write_u8(48 + 0, 6),
            Node::Subtractor32 => w.write_u8(48 + 1, 6),
            Node::Multiplier32 => w.write_u8(24 + 1, 5),
            Node::FullAdder32 => w.write_u8(48 + 2, 6),
            Node::FullSubtractor32 => w.write_u8(48 + 3, 6),
            Node::FullMultiplier32 => w.write_u8(24 + 3, 5),
            Node::Sha256HashBlock => w.write_u8(14, 4),
        }
    }
}

