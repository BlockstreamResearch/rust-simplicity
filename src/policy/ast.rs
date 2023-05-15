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
use std::rc::Rc;

use crate::miniscript::{MiniscriptKey, Translator};

use crate::core::{CommitNode, Context};
use crate::jet::Elements;
use crate::policy::compiler;
use crate::policy::key::PublicKey32;
use crate::Error;

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
    /// Satisfy all of the given sub-policies
    And(Vec<Policy<Pk>>),
    /// Satisfy exactly one of the given sub-policies
    Or(Vec<Policy<Pk>>),
    /// Satisfy exactly `k` of the given sub-policies
    Threshold(usize, Vec<Policy<Pk>>),
}

impl<Pk: MiniscriptKey + PublicKey32> Policy<Pk> {
    /// Compile the policy into a Simplicity program
    pub fn compile(
        &self,
        context: &mut Context<Elements>,
    ) -> Result<Rc<CommitNode<Elements>>, Error> {
        compiler::compile(context, self)
    }
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
            Policy::And(ref subs) => Ok(Policy::And(
                subs.iter()
                    .map(|sub| sub.translate(translator))
                    .collect::<Result<Vec<Policy<Q>>, E>>()?,
            )),
            Policy::Or(ref subs) => Ok(Policy::Or(
                subs.iter()
                    .map(|sub| sub.translate(translator))
                    .collect::<Result<Vec<Policy<Q>>, E>>()?,
            )),
        }
    }

    /// Flatten out trees of `And`s and `Or`s; eliminate `Trivial` and
    /// `Unsatisfiable`s. Does not reorder any branches; use `.sort`.
    pub fn normalized(self) -> Policy<Pk> {
        match self {
            Policy::And(subs) => {
                let mut ret_subs = Vec::with_capacity(subs.len());
                for sub in subs {
                    match sub.normalized() {
                        Policy::Trivial => {}
                        Policy::Unsatisfiable => return Policy::Unsatisfiable,
                        Policy::And(and_subs) => ret_subs.extend(and_subs),
                        x => ret_subs.push(x),
                    }
                }
                match ret_subs.len() {
                    0 => Policy::Trivial,
                    1 => ret_subs.pop().unwrap(),
                    _ => Policy::And(ret_subs),
                }
            }
            Policy::Or(subs) => {
                let mut ret_subs = Vec::with_capacity(subs.len());
                for sub in subs {
                    match sub {
                        Policy::Trivial => return Policy::Trivial,
                        Policy::Unsatisfiable => {}
                        Policy::Or(or_subs) => ret_subs.extend(or_subs),
                        x => ret_subs.push(x),
                    }
                }
                match ret_subs.len() {
                    0 => Policy::Trivial,
                    1 => ret_subs.pop().unwrap(),
                    _ => Policy::Or(ret_subs),
                }
            }
            x => x,
        }
    }

    /// "Sort" a policy to bring it into a canonical form to allow comparisons.
    /// Does **not** allow policies to be compared for functional equivalence;
    /// in general this appears to require Gröbner basis techniques that are not
    /// implemented.
    pub fn sorted(self) -> Policy<Pk> {
        match self {
            Policy::And(subs) => {
                let mut new_subs: Vec<_> = subs.into_iter().map(Policy::sorted).collect();
                new_subs.sort();
                Policy::And(new_subs)
            }
            Policy::Or(subs) => {
                let mut new_subs: Vec<_> = subs.into_iter().map(Policy::sorted).collect();
                new_subs.sort();
                Policy::Or(new_subs)
            }
            Policy::Threshold(k, subs) => {
                let mut new_subs: Vec<_> = subs.into_iter().map(Policy::sorted).collect();
                new_subs.sort();
                Policy::Threshold(k, new_subs)
            }
            x => x,
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
            Policy::And(sub_policies) | Policy::Or(sub_policies) => {
                match self {
                    Policy::And(_) => f.write_str("and(")?,
                    Policy::Or(_) => f.write_str("or(")?,
                    _ => unreachable!(),
                }

                if !sub_policies.is_empty() {
                    write!(f, "{:?}", sub_policies[0])?;
                    for sub in &sub_policies[1..] {
                        write!(f, ",{:?}", sub)?;
                    }
                }
                f.write_str(")")
            }
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
