// SPDX-License-Identifier: CC0-1.0

#![cfg_attr(bench, feature(test))]
#![allow(
    // we use `bool` to represent bits and frequentely assert_eq against them
    clippy::bool_assert_comparison,
    // we use () as the environment for Core (FIXME we should probabl use a newtype)
    clippy::let_unit_value,
    // We write map(Arc::clone) to signify that a cheap Arc is being cloned
    clippy::map_clone
)]

#[cfg(feature = "bitcoin")]
pub extern crate bitcoin;
#[cfg(feature = "elements")]
pub extern crate elements;
#[cfg(feature = "serde")]
pub extern crate serde;

/// Re-export of base64 crate
#[cfg(feature = "base64")]
pub use bitcoin::base64;
/// Re-export of byteorder crate
pub extern crate byteorder;
/// Re-export of ghost_cell crate
pub extern crate ghost_cell;
/// Re-export of hashes crate
pub extern crate hashes;
/// Re-export of hex crate
pub extern crate hex;

#[cfg(bench)]
extern crate test;

#[macro_use]
mod macros;

mod analysis;
mod bit_encoding;
pub mod bit_machine;
pub mod dag;
pub mod human_encoding;
pub mod jet;
mod merkle;
pub mod node;
#[cfg(feature = "elements")]
pub mod policy;
pub mod types;
mod value;

pub use bit_encoding::decode;
pub use bit_encoding::encode;
pub use bit_encoding::{
    u2, BitCollector, BitIter, CloseError as BitIterCloseError, EarlyEndOfStreamError,
};
pub use bit_encoding::{write_to_vec, BitWriter};

#[cfg(feature = "elements")]
pub use crate::policy::{
    sighash, Policy, Preimage32, Satisfier, SimplicityKey, ToXOnlyPubkey, Translator,
};

pub use crate::analysis::{Cost, NodeBounds};
pub use crate::bit_machine::BitMachine;
pub use crate::encode::{encode_natural, encode_value, encode_witness};
pub use crate::merkle::{
    amr::Amr,
    cmr::Cmr,
    ihr::{Ihr, Imr},
    tmr::Tmr,
    FailEntropy, HasCmr,
};
pub use crate::node::{CommitNode, ConstructNode, Hiding, RedeemNode};
pub use crate::value::{Value, ValueRef, Word};
pub use simplicity_sys as ffi;
use std::fmt;

/// Return the version of Simplicity leaves inside a tap tree.
#[cfg(feature = "elements")]
pub fn leaf_version() -> elements::taproot::LeafVersion {
    elements::taproot::LeafVersion::from_u8(0xbe).expect("constant leaf version")
}

/// Error decoding a program from the bit encoding.
#[non_exhaustive]
#[derive(Debug)]
pub enum DecodeError {
    /// Decoder error
    Decode(decode::Error),
    /// A disconnect node was *not* populated at redeem time
    DisconnectRedeemTime,
    /// Type-checking error
    Type(types::Error),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Decode(ref e) => fmt::Display::fmt(e, f),
            Self::DisconnectRedeemTime => {
                f.write_str("disconnect node had one child (redeem time); must have two")
            }
            Self::Type(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Decode(ref e) => Some(e),
            Self::DisconnectRedeemTime => None,
            Self::Type(ref e) => Some(e),
        }
    }
}

/// Error parsing a program or witness (decoding it from a string encoding).
#[non_exhaustive]
#[derive(Debug)]
pub enum ParseError {
    /// Bit-encoding error.
    Decode(DecodeError),
    /// Base64 decoding error
    #[cfg(feature = "base64")]
    Base64(base64::DecodeError),
    /// Hex decoding error
    Hex(hex::error::HexToBytesError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Decode(ref e) => e.fmt(f),
            #[cfg(feature = "base64")]
            Self::Base64(ref e) => e.fmt(f),
            Self::Hex(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Decode(ref e) => Some(e),
            #[cfg(feature = "base64")]
            Self::Base64(ref e) => Some(e),
            Self::Hex(ref e) => Some(e),
        }
    }
}

/// Error finalizing a program (i.e. typechecking and pruning).
#[non_exhaustive]
#[derive(Debug)]
pub enum FinalizeError {
    /// A disconnect node was *not* populated at redeem time
    DisconnectRedeemTime,
    // Execution error
    Execution(bit_machine::ExecutionError),
    /// Type-checking error
    Type(types::Error),
}

impl fmt::Display for FinalizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DisconnectRedeemTime => {
                f.write_str("disconnect node had one child (redeem time); must have two")
            }
            Self::Execution(ref e) => fmt::Display::fmt(e, f),
            Self::Type(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for FinalizeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DisconnectRedeemTime => None,
            Self::Execution(ref e) => Some(e),
            Self::Type(ref e) => Some(e),
        }
    }
}

/// Error type for simplicity
// FIXME we should collapse this error to its single variant; but need to update
// autogenerated code to do so, so we leave it be for now.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJetName(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidJetName(s) => write!(f, "unknown jet `{}`", s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::InvalidJetName(..) => None,
        }
    }
}
