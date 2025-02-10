// SPDX-License-Identifier: CC0-1.0

//! # Abstract Policies
//!
//! These policies represent spending conditions in the most simplest form
//! Policies are combination of `and`, `or` and `thresh` fragments. For example,
//! or(pk(A),pk(B)) represents a policy that can be spend with either pk(A) or pk(B).
//! These policies can be compiled to Simplicity and also be lifted back up from
//! Simplicity expressions to policy.

use std::convert::TryFrom;
use std::sync::Arc;
use std::{fmt, iter, mem};

use crate::jet::Elements;
use crate::node::{ConstructNode, CoreConstructible, JetConstructible, WitnessConstructible};
use crate::policy::serialize::{self, AssemblyConstructible};
use crate::{types, Value};
use crate::{Cmr, CommitNode, FailEntropy};
use crate::{SimplicityKey, ToXOnlyPubkey, Translator};

/// Policy that expresses spending conditions for Simplicity.
///
/// The policy can be compiled into a Simplicity program and executed on the Bit Machine,
/// given a witness.
///
/// Furthermore, the policy can be normalized and is amenable to semantic analysis.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Policy<Pk: SimplicityKey> {
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
    /// Satisfy the program with the given CMR
    Assembly(Cmr),
}

impl<Pk: ToXOnlyPubkey> Policy<Pk> {
    /// Serializes the policy as a Simplicity fragment, with all witness nodes unpopulated.
    fn serialize_no_witness<N>(&self, inference_context: &types::Context) -> Option<N>
    where
        N: CoreConstructible
            + JetConstructible<Elements>
            + WitnessConstructible<Option<Value>>
            + AssemblyConstructible,
    {
        match *self {
            Policy::Unsatisfiable(entropy) => {
                Some(serialize::unsatisfiable(inference_context, entropy))
            }
            Policy::Trivial => Some(serialize::trivial(inference_context)),
            Policy::After(n) => Some(serialize::after(inference_context, n)),
            Policy::Older(n) => Some(serialize::older(inference_context, n)),
            Policy::Key(ref key) => Some(serialize::key(inference_context, key, None)),
            Policy::Sha256(ref hash) => {
                Some(serialize::sha256::<Pk, _, _>(inference_context, hash, None))
            }
            Policy::And {
                ref left,
                ref right,
            } => {
                let left = left.serialize_no_witness(inference_context)?;
                let right = right.serialize_no_witness(inference_context)?;
                Some(serialize::and(&left, &right))
            }
            Policy::Or {
                ref left,
                ref right,
            } => {
                let left = left.serialize_no_witness(inference_context)?;
                let right = right.serialize_no_witness(inference_context)?;
                Some(serialize::or(&left, &right, None))
            }
            Policy::Threshold(k, ref subs) => {
                let k = u32::try_from(k).expect("can have k at most 2^32 in a threshold");
                let subs = subs
                    .iter()
                    .map(|sub| sub.serialize_no_witness(inference_context))
                    .collect::<Option<Vec<N>>>()?;
                let wits = iter::repeat(None).take(subs.len()).collect::<Vec<_>>();
                Some(serialize::threshold(k, &subs, &wits))
            }
            Policy::Assembly(cmr) => N::assembly(inference_context, cmr),
        }
    }

    /// Return the program commitment of the policy.
    pub fn commit(&self) -> Option<Arc<CommitNode<Elements>>> {
        let construct: Arc<ConstructNode<Elements>> =
            self.serialize_no_witness(&types::Context::new())?;
        let commit = construct.finalize_types().expect("policy has sound types");
        Some(commit)
    }

    /// Return the CMR of the policy.
    pub fn cmr(&self) -> Cmr {
        self.serialize_no_witness::<crate::merkle::cmr::ConstructibleCmr>(&types::Context::new())
            .expect("CMR is defined for asm fragment")
            .cmr
    }
}

impl<Pk: SimplicityKey> Policy<Pk> {
    /// Convert a policy using one kind of public key to another
    /// type of public key
    pub fn translate<T, Q, E>(&self, translator: &mut T) -> Result<Policy<Q>, E>
    where
        T: Translator<Pk, Q, E>,
        Q: SimplicityKey,
    {
        match *self {
            Policy::Unsatisfiable(entropy) => Ok(Policy::Unsatisfiable(entropy)),
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
            Policy::Assembly(cmr) => Ok(Policy::Assembly(cmr)),
        }
    }

    /// Flatten out trees of `And`s and `Or`s; eliminate `Trivial` and
    /// `Unsatisfiable`s. Does not reorder any branches; use `.sort`.
    pub fn normalized(self) -> Policy<Pk> {
        match self {
            Policy::And { left, right } => {
                if let Policy::Unsatisfiable(entropy) = *left {
                    Policy::Unsatisfiable(entropy)
                } else if let Policy::Unsatisfiable(entropy) = *right {
                    Policy::Unsatisfiable(entropy)
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
                } else if let Policy::Unsatisfiable(..) = *left {
                    right.as_ref().clone().normalized()
                } else if let Policy::Unsatisfiable(..) = *right {
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

    /// Return an iterator over the fragments of the policy.
    pub fn iter(&self) -> PolicyIter<'_, Pk> {
        PolicyIter::new(self)
    }

    /// Return an iterator over the public keys of the policy.
    pub fn iter_pk(&self) -> impl Iterator<Item = Pk> + '_ {
        self.iter().filter_map(|fragment| match fragment {
            Policy::Key(key) => Some(key.clone()),
            _ => None,
        })
    }
}

impl<Pk: SimplicityKey> fmt::Debug for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Policy::Unsatisfiable(..) => f.write_str("UNSATISFIABLE"),
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
            Policy::Assembly(cmr) => write!(f, "asm({})", cmr),
        }
    }
}

impl<Pk: SimplicityKey> fmt::Display for Policy<Pk> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// Iterator over the fragments of a Simplicity policy.
///
/// The fragments are visited in preorder:
/// We first visit the parent, then the left subtree, then the right subtree.
pub struct PolicyIter<'a, Pk: SimplicityKey> {
    stack: Vec<&'a Policy<Pk>>,
}

impl<'a, Pk: SimplicityKey> PolicyIter<'a, Pk> {
    /// Create an iterator for the given policy.
    pub fn new(policy: &'a Policy<Pk>) -> Self {
        Self {
            stack: vec![policy],
        }
    }
}

impl<'a, Pk: SimplicityKey> Iterator for PolicyIter<'a, Pk> {
    type Item = &'a Policy<Pk>;

    fn next(&mut self) -> Option<Self::Item> {
        let top = self.stack.pop()?;
        match top {
            Policy::And { left, right } | Policy::Or { left, right } => {
                self.stack.push(right);
                self.stack.push(left);
            }
            Policy::Threshold(_, children) => {
                self.stack.extend(children.iter().rev());
            }
            _ => {}
        }
        Some(top)
    }
}
