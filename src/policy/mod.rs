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

mod ast;
mod compiler;
pub mod descriptor;
mod embed;
mod error;
pub mod key;
pub mod satisfy;

use crate::miniscript::{MiniscriptKey, Translator};

use std::fmt;
use std::sync::Arc;

pub use descriptor::Descriptor;
pub use error::Error;

/// Policy that expresses spending conditions for Simplicity.
///
/// The policy can be encoded directly as a Simplicity program which can be committed to the
/// blockchain; when finalized, it can executed on the Bit Machine.
///
/// Policies can be normalized and are amenable to semantic analysis.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Policy<Pk: MiniscriptKey> {
    fragment: ast::Fragment<Pk>,
}

impl<Pk: MiniscriptKey> Policy<Pk> {
    /// Create an unsatisfiable policy.
    pub fn unsatisfiable() -> Self {
        Policy {
            fragment: ast::Fragment::Unsatisfiable,
        }
    }

    /// Create a trivial policy.
    pub fn trivial() -> Self {
        Policy {
            fragment: ast::Fragment::Trivial,
        }
    }

    /// Create a signature check.
    pub fn key(key: Pk) -> Self {
        Policy {
            fragment: ast::Fragment::Key(key),
        }
    }

    /// Create an absolute locktime check.
    pub fn after(n: u32) -> Self {
        Policy {
            fragment: ast::Fragment::After(n),
        }
    }

    /// Create a relative locktime check.
    pub fn older(n: u16) -> Self {
        Policy {
            fragment: ast::Fragment::Older(n),
        }
    }

    /// Create a SHA256 preimage check.
    pub fn sha256(hash: Pk::Sha256) -> Self {
        Policy {
            fragment: ast::Fragment::Sha256(hash),
        }
    }

    /// Create a conjunction of two policies.
    pub fn and(left: Arc<Self>, right: Arc<Self>) -> Self {
        Policy {
            fragment: ast::Fragment::And { left, right },
        }
    }

    /// Create a disjunction of two policies.
    pub fn or(left: Arc<Self>, right: Arc<Self>) -> Self {
        Policy {
            fragment: ast::Fragment::Or { left, right },
        }
    }

    /// Create a disjunction of two policies.
    pub fn threshold(k: usize, subs: Vec<Self>) -> Self {
        Policy {
            fragment: ast::Fragment::Threshold(k, subs),
        }
    }

    /// Convert a fragment using one kind of public key to another type of public key
    pub fn translate<T, Q, E>(&self, translator: &mut T) -> Result<Policy<Q>, E>
    where
        T: Translator<Pk, Q, E>,
        Q: MiniscriptKey,
    {
        Ok(Policy {
            fragment: self.fragment.translate(translator)?,
        })
    }
}

impl<Pk: MiniscriptKey> fmt::Debug for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.fragment, f)
    }
}

impl<Pk: MiniscriptKey> fmt::Display for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.fragment, f)
    }
}
