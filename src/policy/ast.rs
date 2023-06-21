// Simplicity Policy
// Written in 2020 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//     Sanket Kanjalkar <sanket1729@gmail.com>
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Abstract Policies
//!
//! These policies represent spending conditions in the most simplest form
//! Policies are combination of `and`, `or` and `thresh` fragments. For example,
//! or(pk(A),pk(B)) represents a policy that can be spend with either pk(A) or pk(B).
//! These policies can be compiled to Simplicity and also be lifted back up from
//! Simplicity expressions to policy.

use std::fmt;
use std::sync::Arc;

use super::Policy;

use crate::miniscript::{MiniscriptKey, Translator};
use crate::FailEntropy;

/// Policy that expresses spending conditions for Simplicity.
///
/// The policy can be compiled into a Simplicity program and executed on the Bit Machine,
/// given a witness.
///
/// Furthermore, the policy can be normalized and is amenable to semantic analysis.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Fragment<Pk: MiniscriptKey> {
    /// Unsatisfiable
    Unsatisfiable(FailEntropy),
    /// Trivially satisfiable
    Trivial,
    /// Provide a signature that matches the given public key and some given message hash
    Key(Pk),
    /// Absolute timelock
    After(u32),
    /// Relative timelock
    Older(u16),
    /// Provide the preimage of the given SHA256 hash image
    Sha256(Pk::Sha256),
    /// Satisfy both of the given sub-policies
    And {
        left: Arc<Policy<Pk>>,
        right: Arc<Policy<Pk>>,
    },
    /// Satisfy exactly one of the given sub-policies
    Or {
        left: Arc<Policy<Pk>>,
        right: Arc<Policy<Pk>>,
    },
    /// Satisfy exactly `k` of the given sub-policies
    Threshold(usize, Vec<Policy<Pk>>),
}

impl<Pk: MiniscriptKey> Fragment<Pk> {
    /// Convert a fragment using one kind of public key to another type of public key
    pub fn translate<T, Q, E>(&self, translator: &mut T) -> Result<Fragment<Q>, E>
    where
        T: Translator<Pk, Q, E>,
        Q: MiniscriptKey,
    {
        match *self {
            Fragment::Unsatisfiable(entropy) => Ok(Fragment::Unsatisfiable(entropy)),
            Fragment::Trivial => Ok(Fragment::Trivial),
            Fragment::Key(ref pk) => translator.pk(pk).map(Fragment::Key),
            Fragment::Sha256(ref h) => translator.sha256(h).map(Fragment::Sha256),
            Fragment::After(n) => Ok(Fragment::After(n)),
            Fragment::Older(n) => Ok(Fragment::Older(n)),
            Fragment::Threshold(k, ref subs) => {
                let new_subs: Result<Vec<Policy<Q>>, _> =
                    subs.iter().map(|sub| sub.translate(translator)).collect();
                new_subs.map(|ok| Fragment::Threshold(k, ok))
            }
            Fragment::And {
                ref left,
                ref right,
            } => Ok(Fragment::And {
                left: Arc::new(left.translate(translator)?),
                right: Arc::new(right.translate(translator)?),
            }),
            Fragment::Or {
                ref left,
                ref right,
            } => Ok(Fragment::Or {
                left: Arc::new(left.translate(translator)?),
                right: Arc::new(right.translate(translator)?),
            }),
        }
    }
}

impl<Pk: MiniscriptKey> fmt::Debug for Fragment<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Fragment::Unsatisfiable(..) => f.write_str("UNSATISFIABLE"),
            Fragment::Trivial => f.write_str("TRIVIAL"),
            Fragment::Key(pk) => write!(f, "pk({})", pk),
            Fragment::After(n) => write!(f, "after({})", n),
            Fragment::Older(n) => write!(f, "older({})", n),
            Fragment::Sha256(h) => write!(f, "sha256({})", h),
            Fragment::And { left, right } => write!(f, "and({},{})", left, right),
            Fragment::Or { left, right } => write!(f, "or({},{})", left, right),
            Fragment::Threshold(k, sub_policies) => {
                write!(f, "thresh({}", k)?;
                for sub in sub_policies {
                    write!(f, ",{:?}", sub)?;
                }
                f.write_str(")")
            }
        }
    }
}

impl<Pk: MiniscriptKey> fmt::Display for Fragment<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
