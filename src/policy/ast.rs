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

use std::sync::Arc;
use std::{fmt, mem};

use crate::miniscript::{MiniscriptKey, Translator};

/// Policy that expresses spending conditions for Simplicity.
///
/// The policy can be compiled into a Simplicity program and executed on the Bit Machine,
/// given a witness.
///
/// Furthermore, the policy can be normalized and is amenable to semantic analysis.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Policy<Pk: MiniscriptKey> {
    /// Unsatisfiable
    Unsatisfiable,
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

impl<Pk: MiniscriptKey> Policy<Pk> {
    /// Convert a policy using one kind of public key to another
    /// type of public key
    pub fn translate<T, Q, E>(&self, translator: &mut T) -> Result<Policy<Q>, E>
    where
        T: Translator<Pk, Q, E>,
        Q: MiniscriptKey,
    {
        match *self {
            Policy::Unsatisfiable => Ok(Policy::Unsatisfiable),
            Policy::Trivial => Ok(Policy::Trivial),
            Policy::Key(ref pk) => translator.pk(pk).map(Policy::Key),
            Policy::Sha256(ref h) => translator.sha256(h).map(Policy::Sha256),
            Policy::After(n) => Ok(Policy::After(n)),
            Policy::Older(n) => Ok(Policy::Older(n)),
            Policy::Threshold(k, ref subs) => {
                let new_subs: Result<Vec<Policy<Q>>, _> =
                    subs.iter().map(|sub| sub.translate(translator)).collect();
                new_subs.map(|ok| Policy::Threshold(k, ok))
            }
            Policy::And {
                ref left,
                ref right,
            } => Ok(Policy::And {
                left: Arc::new(left.translate(translator)?),
                right: Arc::new(right.translate(translator)?),
            }),
            Policy::Or {
                ref left,
                ref right,
            } => Ok(Policy::Or {
                left: Arc::new(left.translate(translator)?),
                right: Arc::new(right.translate(translator)?),
            }),
        }
    }

    /// Flatten out trees of `And`s and `Or`s; eliminate `Trivial` and
    /// `Unsatisfiable`s. Does not reorder any branches; use `.sort`.
    pub fn normalized(self) -> Policy<Pk> {
        match self {
            Policy::And { left, right } => {
                if *left == Policy::Unsatisfiable || *right == Policy::Unsatisfiable {
                    Policy::Unsatisfiable
                } else if *left == Policy::Trivial {
                    right.as_ref().clone().normalized()
                } else if *right == Policy::Trivial {
                    left.as_ref().clone().normalized()
                } else {
                    Policy::And {
                        left: Arc::new(left.as_ref().clone().normalized()),
                        right: Arc::new(right.as_ref().clone().normalized()),
                    }
                }
            }
            Policy::Or { left, right } => {
                if *left == Policy::Trivial || *right == Policy::Trivial {
                    Policy::Trivial
                } else if *left == Policy::Unsatisfiable {
                    right.as_ref().clone().normalized()
                } else if *right == Policy::Unsatisfiable {
                    left.as_ref().clone().normalized()
                } else {
                    Policy::Or {
                        left: Arc::new(left.as_ref().clone().normalized()),
                        right: Arc::new(right.as_ref().clone().normalized()),
                    }
                }
            }
            x => x,
        }
    }

    /// "Sort" a policy to bring it into a canonical form to allow comparisons.
    /// Does **not** allow policies to be compared for functional equivalence;
    /// in general this appears to require Gröbner basis techniques that are not
    /// implemented.
    pub fn sorted(mut self) -> Policy<Pk> {
        self.sort();
        self
    }

    fn sort(&mut self) {
        match self {
            Policy::And {
                ref mut left,
                ref mut right,
            }
            | Policy::Or {
                ref mut left,
                ref mut right,
            } => {
                left.as_ref().clone().sort();
                right.as_ref().clone().sort();
                if right > left {
                    mem::swap(left, right);
                }
            }
            Policy::Threshold(_, ref mut subs) => {
                for sub in &mut *subs {
                    sub.sort();
                }
                subs.sort();
            }
            _ => {}
        }
    }
}

impl<Pk: MiniscriptKey> fmt::Debug for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Policy::Unsatisfiable => f.write_str("UNSATISFIABLE"),
            Policy::Trivial => f.write_str("TRIVIAL"),
            Policy::Key(pk) => write!(f, "pk({})", pk),
            Policy::After(n) => write!(f, "after({})", n),
            Policy::Older(n) => write!(f, "older({})", n),
            Policy::Sha256(h) => write!(f, "sha256({})", h),
            Policy::And { left, right } => write!(f, "and({},{})", left, right),
            Policy::Or { left, right } => write!(f, "or({},{})", left, right),
            Policy::Threshold(k, sub_policies) => {
                write!(f, "thresh({}", k)?;
                for sub in sub_policies {
                    write!(f, ",{:?}", sub)?;
                }
                f.write_str(")")
            }
        }
    }
}

impl<Pk: MiniscriptKey> fmt::Display for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
