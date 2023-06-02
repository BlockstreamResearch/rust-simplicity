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

#![allow(clippy::redundant_field_names, clippy::identity_op)]

#[cfg(feature = "serde")]
pub use actual_serde as serde;
#[cfg(feature = "bitcoin")]
pub use bitcoin;
#[cfg(feature = "elements")]
pub use elements;

pub use bitcoin_hashes;
pub use byteorder;
pub use elements_miniscript as miniscript;

#[macro_use]
mod macros;

mod analysis;
pub mod bit_machine;
pub mod bititer;
pub mod bitwriter;
pub mod core;
mod decode;
mod encode;
pub mod jet;
pub mod merkle;
#[cfg(feature = "elements")]
pub mod policy;
mod sharing;
pub mod types;
// #[cfg(test)]
// mod test_progs;
mod util;

#[cfg(feature = "elements")]
pub use crate::policy::Policy;

pub use crate::bit_machine::exec;
pub use crate::core::commit::CommitNodeInner;
pub use crate::core::redeem::RedeemNodeInner;
pub use crate::core::{CommitNode, Context, RedeemNode};
pub use crate::decode::{decode_natural, decode_value, WitnessDecoder};
pub use crate::encode::{encode_natural, encode_value, encode_witness};
pub use crate::merkle::{amr::Amr, cmr::Cmr, imr::Imr, tmr::Tmr};
pub use simplicity_sys as ffi;
use std::fmt;

/// Error type for simplicity
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Type-checking error
    Type(crate::types::Error),
    /// Node made a back-reference past the beginning of the program
    BadIndex,
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// Both children of a node are hidden
    BothChildrenHidden,
    /// Bitstream ended early
    EndOfStream,
    /// Program must not be empty
    EmptyProgram,
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJetName(String),
    /// Tried to allocate too many nodes in a program
    TooManyNodes(usize),
    /// Cannot parse bitstream
    ParseError(&'static str),
    /// Program is not in canonical order
    NotInCanonicalOrder,
    /// Witness has different length than defined in its preamble
    InconsistentWitnessLength,
    /// Program does not have maximal sharing
    SharingNotMaximal,
    /// Miniscript error
    MiniscriptError(miniscript::Error),
    /// Policy error
    #[cfg(feature = "elements")]
    Policy(policy::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Type(ref e) => fmt::Display::fmt(e, f),
            Error::BadIndex => {
                f.write_str("Node made a back-reference past the beginning of the program")
            }
            Error::NaturalOverflow => f.write_str("Number exceeded 32 bits"),
            Error::BothChildrenHidden => f.write_str("Both children of a node are hidden"),
            Error::EndOfStream => f.write_str("Bitstream ended early"),
            Error::EmptyProgram => f.write_str("Program must not be empty"),
            Error::InvalidJetName(s) => write!(f, "unknown jet `{}`", s),
            Error::TooManyNodes(k) => {
                write!(f, "Tried to allocate too many nodes in a program: {}", k)
            }
            Error::ParseError(s) => write!(f, "Cannot parse bitstream {}", s),
            Error::NotInCanonicalOrder => f.write_str("Program is not in canonical order"),
            Error::InconsistentWitnessLength => {
                f.write_str("Witness has different length than defined in its preamble")
            }
            Error::SharingNotMaximal => f.write_str("Decoded programs must have maximal sharing"),
            Error::MiniscriptError(ref e) => fmt::Display::fmt(e, f),
            #[cfg(feature = "elements")]
            Error::Policy(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Type(ref e) => Some(e),
            Error::BadIndex => None,
            Error::NaturalOverflow => None,
            Error::BothChildrenHidden => None,
            Error::EndOfStream => None,
            Error::EmptyProgram => None,
            Error::InvalidJetName(..) => None,
            Error::TooManyNodes(..) => None,
            Error::ParseError(..) => None,
            Error::NotInCanonicalOrder => None,
            Error::InconsistentWitnessLength => None,
            Error::SharingNotMaximal => None,
            Error::MiniscriptError(ref e) => Some(e),
            #[cfg(feature = "elements")]
            Error::Policy(ref e) => Some(e),
        }
    }
}

impl From<crate::types::Error> for Error {
    fn from(e: crate::types::Error) -> Error {
        Error::Type(e)
    }
}

impl From<miniscript::Error> for Error {
    fn from(e: miniscript::Error) -> Error {
        Error::MiniscriptError(e)
    }
}

#[cfg(feature = "elements")]
impl From<policy::Error> for Error {
    fn from(e: policy::Error) -> Error {
        Error::Policy(e)
    }
}
