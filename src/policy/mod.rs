// Simplcity Policy language
// Written in 2020 by
//     Sanket Kanjalkar <sanket1729@gmail.com>
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

//! # Function-like Expression Language
//! This language is similar to policy language in miniscript, but
//! whose target is simplicity instead of bitcoin Script. Policy
//!
//! # Simplicity Script Policies
//!
//! Tools for representing simplicity programs as spending policies.
//! These may be compiled to Simplicity programs.
//!
//! The format represents public keys abstractly to allow wallets to replace
//! these with BIP32 paths, pay-to-contract instructions, etc.
//!

pub mod compiler;
pub mod policy;
use std::fmt;

// FIXME: Make this some sane number?
const MAX_POLICY_RECURSION_DEPTH: u32 = 100;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Error {
    /// Encountered unprintable character in descriptor
    Unprintable(u8),
    /// expected character while parsing descriptor; didn't find one
    ExpectedChar(char),
    /// While parsing backward, hit beginning of script
    UnexpectedStart,
    /// Got something we were not expecting
    Unexpected(String),
    /// Highly nested policy
    DeepNestedPolicy,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Unprintable(x) => write!(f, "unprintable character 0x{:02x}", x),
            Error::ExpectedChar(c) => write!(f, "expected {}", c),
            Error::UnexpectedStart => f.write_str("unexpected start of script"),
            Error::Unexpected(ref s) => write!(f, "unexpected «{}»", s),
            Error::DeepNestedPolicy => write!(
                f,
                "Max Policy depth exceeded {}",
                MAX_POLICY_RECURSION_DEPTH
            ),
        }
    }
}
