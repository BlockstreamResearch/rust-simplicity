// SPDX-License-Identifier: CC0-1.0

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

mod ast;
mod error;
mod key;
mod satisfy;
mod serialize;
pub mod sighash;

pub use ast::Policy;
pub use error::Error;
pub use key::{SimplicityKey, ToXOnlyPubkey, Translator};
pub use satisfy::{Preimage32, Satisfier, SatisfierError};
