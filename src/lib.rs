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

extern crate bitcoin_hashes;
extern crate byteorder;
extern crate miniscript;

#[macro_use]
mod macros;

mod analysis;
pub mod bit_machine;
pub mod bititer;
pub mod bitwriter;
pub mod core;
#[cfg(fuzzing)]
pub mod decode;
#[cfg(not(fuzzing))]
mod decode;
#[cfg(fuzzing)]
pub mod encode;
#[cfg(not(fuzzing))]
mod encode;
mod inference;
pub mod jet;
pub mod merkle;
#[cfg(feature = "bitcoin")]
pub mod policy;
mod sharing;
#[cfg(test)]
mod test_progs;
mod util;

pub use crate::bit_machine::exec;
pub use crate::core::{CommitNode, RedeemNode};
use crate::inference::UnificationArrow;
use std::fmt;

/// Error type for simplicity
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
        /// Unification arrow of left child, if it exists
        left: Option<UnificationArrow>,
        /// Unification arrow of right child, if it exists
        right: Option<UnificationArrow>,
    },
    /// Node made a back-reference past the beginning of the program
    BadIndex,
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// Non-'case' nodes may not have hidden children
    NonCaseHiddenChild,
    /// 'case' nodes may have at most one hidden child
    CaseMultipleHiddenChildren,
    /// Right child of left assertion must be hidden
    RightChildNotHidden,
    /// Left child of right assertion must be hidden
    LeftChildNotHidden,
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
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Unification(s) => write!(f, "Unification failed. Hint: {}", s),
            Error::OccursCheck => f.write_str("A type is recursive (i.e., occurs within itself)"),
            Error::TypeCheck {
                unification_hint,
                root_hint,
                left,
                right,
            } => {
                if let Some(left) = left {
                    if let Some(right) = right {
                        write!(
                            f,
                            "Type checking failed. Hint: {}\n{}\nActual left  child: {}\nActual right child: {}",
                            unification_hint, root_hint, left, right
                        )
                    } else {
                        write!(
                            f,
                            "Type checking failed. Hint: {}\n{}\nActual child: {}",
                            unification_hint, root_hint, left
                        )
                    }
                } else {
                    write!(
                        f,
                        "Type checking failed. Hint: {}\n{}",
                        unification_hint, root_hint
                    )
                }
            }
            Error::BadIndex => {
                f.write_str("Node made a back-reference past the beginning of the program")
            }
            Error::NaturalOverflow => f.write_str("Number exceeded 32 bits"),
            Error::NonCaseHiddenChild => {
                f.write_str("Non-'case' nodes may not have hidden children")
            }
            Error::CaseMultipleHiddenChildren => {
                f.write_str("'case' nodes may have at most one hidden child")
            }
            Error::RightChildNotHidden => {
                f.write_str("The right child of a left assertion must be a hidden node")
            }
            Error::LeftChildNotHidden => {
                f.write_str("The left child of a right assertion must be a hidden node")
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
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[doc(hidden)]
impl From<miniscript::Error> for Error {
    fn from(e: miniscript::Error) -> Error {
        Error::MiniscriptError(e)
    }
}
