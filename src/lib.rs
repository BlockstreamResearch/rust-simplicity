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

/// Re-export of byteorder crate
pub extern crate byteorder;
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
    imr::{FirstPassImr, Imr},
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

/// Error type for simplicity
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Decoder error
    Decode(decode::Error),
    /// A disconnect node was *not* populated at redeem time
    DisconnectRedeemTime,
    /// Type-checking error
    Type(types::Error),
    // Execution error
    Execution(bit_machine::ExecutionError),
    /// Tried to parse a jet but the name wasn't recognized
    InvalidJetName(String),
    /// Policy error
    #[cfg(feature = "elements")]
    Policy(policy::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Decode(ref e) => fmt::Display::fmt(e, f),
            Error::DisconnectRedeemTime => {
                f.write_str("disconnect node had one child (redeem time); must have two")
            }
            Error::Type(ref e) => fmt::Display::fmt(e, f),
            Error::Execution(ref e) => fmt::Display::fmt(e, f),
            Error::InvalidJetName(s) => write!(f, "unknown jet `{}`", s),
            #[cfg(feature = "elements")]
            Error::Policy(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Decode(ref e) => Some(e),
            Error::DisconnectRedeemTime => None,
            Error::Type(ref e) => Some(e),
            Error::Execution(ref e) => Some(e),
            Error::InvalidJetName(..) => None,
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

#[cfg(feature = "elements")]
impl From<policy::Error> for Error {
    fn from(e: policy::Error) -> Error {
        Error::Policy(e)
    }
}
