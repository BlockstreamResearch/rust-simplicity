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

#![allow(
    // we use `bool` to represent bits and frequentely assert_eq against them
    clippy::bool_assert_comparison,
    // we use () as the environment for Core (FIXME we should probabl use a newtype)
    clippy::let_unit_value
)]

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
pub mod bit_encoding;
pub mod bit_machine;
mod context;
pub mod core;
pub mod dag;
pub mod jet;
mod merkle;
#[cfg(feature = "elements")]
pub mod policy;
pub mod types;
// #[cfg(test)]
// mod test_progs;
mod value;

use bit_encoding::bititer; // FIXME used by autogenerator jet code
pub use bit_encoding::bititer::EarlyEndOfStreamError;
use bit_encoding::bitwriter; // FIXME used by autogenerator jet code
use bit_encoding::decode;
use bit_encoding::encode;
pub use bit_encoding::BitIter;
pub use bit_encoding::BitWriter;

#[cfg(feature = "elements")]
pub use crate::policy::{Descriptor, Policy};

pub use crate::bit_machine::exec;
pub use crate::context::Context;
pub use crate::core::commit::CommitNodeInner;
pub use crate::core::redeem::RedeemNodeInner;
pub use crate::core::{CommitNode, RedeemNode};
pub use crate::decode::{decode_program, WitnessDecoder};
pub use crate::encode::{encode_natural, encode_value, encode_witness};
pub use crate::merkle::{
    amr::Amr,
    cmr::Cmr,
    imr::{FirstPassImr, Imr},
    tmr::Tmr,
};
pub use crate::value::Value;
pub use simplicity_sys as ffi;
use std::fmt;

/// Error type for simplicity
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Decoder error
    Decode(crate::decode::Error),
    /// Type-checking error
    Type(crate::types::Error),
    /// Witness iterator ended early
    NoMoreWitnesses,
    /// Witness has different length than defined in its preamble
    InconsistentWitnessLength,
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJetName(String),
    /// Miniscript error
    MiniscriptError(miniscript::Error),
    /// Policy error
    #[cfg(feature = "elements")]
    Policy(policy::Error),
    /// Program does not have maximal sharing
    SharingNotMaximal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Decode(ref e) => fmt::Display::fmt(e, f),
            Error::Type(ref e) => fmt::Display::fmt(e, f),
            Error::InconsistentWitnessLength => {
                f.write_str("witness has different length than defined in its preamble")
            }
            Error::InvalidJetName(s) => write!(f, "unknown jet `{}`", s),
            Error::NoMoreWitnesses => f.write_str("no more witness data available"),
            Error::MiniscriptError(ref e) => fmt::Display::fmt(e, f),
            #[cfg(feature = "elements")]
            Error::Policy(ref e) => fmt::Display::fmt(e, f),
            Error::SharingNotMaximal => f.write_str("Decoded programs must have maximal sharing"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Decode(ref e) => Some(e),
            Error::Type(ref e) => Some(e),
            Error::NoMoreWitnesses => None,
            Error::InconsistentWitnessLength => None,
            Error::InvalidJetName(..) => None,
            Error::SharingNotMaximal => None,
            Error::MiniscriptError(ref e) => Some(e),
            #[cfg(feature = "elements")]
            Error::Policy(ref e) => Some(e),
        }
    }
}

impl From<crate::decode::Error> for Error {
    fn from(e: crate::decode::Error) -> Error {
        Error::Decode(e)
    }
}

impl From<EarlyEndOfStreamError> for Error {
    fn from(e: EarlyEndOfStreamError) -> Error {
        Error::Decode(e.into())
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
