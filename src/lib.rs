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
mod inference;
pub mod jet;
pub mod merkle;
#[cfg(feature = "elements")]
pub mod policy;
mod sharing;
// #[cfg(test)]
// mod test_progs;
mod util;

#[cfg(feature = "elements")]
pub use crate::policy::Policy;

pub use crate::bit_machine::exec;
pub use crate::core::{CommitNode, Context, RedeemNode};
pub use crate::decode::{decode_natural, WitnessDecoder};
pub use crate::encode::{encode_natural, encode_witness};
pub use simplicity_sys as ffi;
use std::fmt;

/// Error type for simplicity
#[non_exhaustive]
pub enum Error {
    /// A type cannot be unified with another type
    Unification(&'static str),
    /// A type is recursive (i.e., occurs within itself), violating the "occurs check"
    OccursCheck,
    /// A DAG cannot be created because the children of the root cannot be unified
    TypeCheck {
        /// Hint why unification failed
        unification_hint: &'static str,
        /// Hint why root type does not match children types
        root_hint: &'static str,
    },
    /// Node made a back-reference past the beginning of the program
    BadIndex,
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// 'case' nodes may have at most one hidden child
    CaseMultipleHiddenChildren,
    /// Bitstream ended early
    EndOfStream,
    /// Program must not be empty
    EmptyProgram,
    /// Tried to allocate too many nodes in a program
    TooManyNodes(usize),
    /// Unrecognized node
    ParseError(&'static str),
    /// Program is not in canonical order
    NotInCanonicalOrder,
    /// Witness has different length than defined in its preamble
    InconsistentWitnessLength,
    /// Program does not have maximal sharing
    SharingNotMaximal,
    /// Miniscript Error
    MiniscriptError(miniscript::Error),
    /// Policy-related error
    #[cfg(feature = "elements")]
    Policy(policy::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unification(s) => write!(f, "Unification failed. Hint: {}", s),
            Error::OccursCheck => f.write_str("A type is recursive (i.e., occurs within itself)"),
            Error::TypeCheck {
                unification_hint,
                root_hint,
            } => {
                write!(
                    f,
                    "Type checking failed. Hint: {}\n{}",
                    unification_hint, root_hint
                )
            }
            Error::BadIndex => {
                f.write_str("Node made a back-reference past the beginning of the program")
            }
            Error::NaturalOverflow => f.write_str("Number exceeded 32 bits"),
            Error::CaseMultipleHiddenChildren => {
                f.write_str("'case' nodes may have at most one hidden child")
            }
            Error::EndOfStream => f.write_str("Bitstream ended early"),
            Error::EmptyProgram => f.write_str("Program must not be empty"),
            Error::TooManyNodes(k) => {
                write!(f, "Tried to allocate too many nodes in a program: {}", k)
            }
            Error::ParseError(s) => write!(f, "Unrecognized node {}", s),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

#[doc(hidden)]
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
