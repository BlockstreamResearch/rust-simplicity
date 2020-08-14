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

pub mod ast;
pub mod compiler;
pub mod lift;
