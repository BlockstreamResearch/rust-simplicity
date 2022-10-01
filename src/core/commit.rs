// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

use crate::bititer::BitIter;
use crate::bitwriter::BitWriter;
use crate::core::iter::{DagIterable, WitnessIterator};
use crate::core::redeem::{NodeType, RedeemNodeInner};
use crate::core::{RedeemNode, Value};
use crate::inference::UnificationArrow;
use crate::jet::{Application, JetNode};
use crate::merkle::cmr::Cmr;
use crate::merkle::imr;
use crate::{analysis, decode, impl_ref_wrapper, Error};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;
use std::{fmt, io};

/// Underlying combinator of a [`CommitNode`].
/// May contain references to children, hash payloads or jets.
#[derive(Debug)]
pub enum CommitNodeInner<App: Application> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(Rc<CommitNode<App>>),
    /// Right injection of some child
    InjR(Rc<CommitNode<App>>),
    /// Take of some child
    Take(Rc<CommitNode<App>>),
    /// Drop of some child
    Drop(Rc<CommitNode<App>>),
    /// Composition of a left and right child
    Comp(Rc<CommitNode<App>>, Rc<CommitNode<App>>),
    /// Case of a left and right child
    Case(Rc<CommitNode<App>>, Rc<CommitNode<App>>),
    /// Left assertion of a left and right child.
    AssertL(Rc<CommitNode<App>>, Rc<CommitNode<App>>),
    /// Right assertion of a left and right child.
    AssertR(Rc<CommitNode<App>>, Rc<CommitNode<App>>),
    /// Pair of a left and right child
    Pair(Rc<CommitNode<App>>, Rc<CommitNode<App>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<CommitNode<App>>, Rc<CommitNode<App>>),
    /// Witness data (missing during commitment, inserted during redemption)
    Witness,
    /// Universal fail
    Fail(Cmr, Cmr),
    /// Hidden CMR
    Hidden(Cmr),
    /// Application jet
    Jet(&'static JetNode<App>),
}

impl<App: Application> CommitNodeInner<App> {
    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&CommitNode<App>> {
        match self {
            CommitNodeInner::Iden
            | CommitNodeInner::Unit
            | CommitNodeInner::Witness
            | CommitNodeInner::Fail(_, _)
            | CommitNodeInner::Hidden(_)
            | CommitNodeInner::Jet(_) => None,
            CommitNodeInner::InjL(l)
            | CommitNodeInner::InjR(l)
            | CommitNodeInner::Take(l)
            | CommitNodeInner::Drop(l)
            | CommitNodeInner::Comp(l, _)
            | CommitNodeInner::Case(l, _)
            | CommitNodeInner::AssertL(l, _)
            | CommitNodeInner::AssertR(l, _)
            | CommitNodeInner::Pair(l, _)
            | CommitNodeInner::Disconnect(l, _) => Some(l),
        }
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&CommitNode<App>> {
        match self {
            CommitNodeInner::Iden
            | CommitNodeInner::Unit
            | CommitNodeInner::Witness
            | CommitNodeInner::Fail(_, _)
            | CommitNodeInner::Hidden(_)
            | CommitNodeInner::Jet(_)
            | CommitNodeInner::InjL(_)
            | CommitNodeInner::InjR(_)
            | CommitNodeInner::Take(_)
            | CommitNodeInner::Drop(_) => None,
            CommitNodeInner::Comp(_, r)
            | CommitNodeInner::Case(_, r)
            | CommitNodeInner::AssertL(_, r)
            | CommitNodeInner::AssertR(_, r)
            | CommitNodeInner::Pair(_, r)
            | CommitNodeInner::Disconnect(_, r) => Some(r),
        }
    }
}

impl<App: Application> fmt::Display for CommitNodeInner<App> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommitNodeInner::Iden => f.write_str("iden"),
            CommitNodeInner::Unit => f.write_str("unit"),
            CommitNodeInner::InjL(_) => f.write_str("injl"),
            CommitNodeInner::InjR(_) => f.write_str("injr"),
            CommitNodeInner::Take(_) => f.write_str("take"),
            CommitNodeInner::Drop(_) => f.write_str("drop"),
            CommitNodeInner::Comp(_, _) => f.write_str("comp"),
            CommitNodeInner::Case(_, _) => f.write_str("case"),
            CommitNodeInner::AssertL(_, _) => f.write_str("assertl"),
            CommitNodeInner::AssertR(_, _) => f.write_str("assertr"),
            CommitNodeInner::Pair(_, _) => f.write_str("pair"),
            CommitNodeInner::Disconnect(_, _) => f.write_str("disconnect"),
            CommitNodeInner::Witness => f.write_str("witness"),
            CommitNodeInner::Fail(..) => f.write_str("fail"),
            CommitNodeInner::Hidden(..) => f.write_str("hidden"),
            CommitNodeInner::Jet(jet) => write!(f, "jet({})", jet.name),
        }
    }
}

/// Root node of a Simplicity DAG for some application.
/// The DAG contains metadata for committing to it via its Merkle root.
///
/// We also refer to DAGs as _(Simplicity) programs_.
///
/// A DAG is a _directed acyclic graph_ consisting of _nodes_ and _edges_.
/// There is a _root_,
/// nodes may have left or right _children_,
/// and nodes without children are called _leaves_.
///
/// Nodes refer to other nodes via reference-counted pointers to heap memory.
/// If possible, duplicate DAGs make use of this fact and reference the same memory.
#[derive(Debug)]
pub struct CommitNode<App: Application> {
    /// Underlying combinator of the node
    pub inner: CommitNodeInner<App>,
    /// Commitment Merkle root of the node
    pub cmr: Cmr,
    /// Unification arrow of the node
    pub arrow: UnificationArrow,
}

impl<App: Application> CommitNode<App> {
    /// Create a node from its underlying combinator.
    fn from_inner(_inner: CommitNodeInner<App>) -> Rc<Self> {
        todo!()
    }

    /// Create a DAG that computes the identity function.
    ///
    /// _Overall type: A → A_
    pub fn iden() -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Iden)
    }

    /// Create a DAG that returns the unit constant.
    ///
    /// _Overall type: A → 1_
    pub fn unit() -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Unit)
    }

    /// Create a DAG that computes the left injection of the given `child`.
    ///
    /// _Overall type: A → B + C where child: A → B_
    pub fn injl(child: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::InjL(child))
    }

    /// Create a DAG that computes the right injection of the given `child`.
    ///
    /// _Overall type: A → B + C where child: A → C_
    pub fn injr(child: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::InjR(child))
    }

    /// Create a DAG with that computes _take_ of the given `child`.
    ///
    /// _Overall type: A × B → C where child: A → C_
    pub fn take(child: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Take(child))
    }

    /// Create a DAG with that computes _drop_ of the given `child`.
    ///
    /// _Overall type: A × B → C where child: B → C_
    pub fn drop(child: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Drop(child))
    }

    /// Create a DAG that computes the composition of the given `left` and `right` child.
    ///
    /// _Overall type: A → C where left: A → B and right: B → C_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn comp(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Comp(left, right))
    }

    /// Create a DAG that computes _case_ of the given `left` and `right` child.
    ///
    /// _Overall type: (A + B) × C → D where left: A × C → D and right: B × C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn case(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Case(left, right))
    }

    /// Create a DAG that computes the left assertion of the given `left` child.
    /// The `right` child must be a hidden node.
    ///
    /// _Overall type: (A + B) × C → D where left: A × C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn assertl(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        if let CommitNodeInner::Hidden(_) = right.inner {
            Self::from_inner(CommitNodeInner::AssertL(left, right))
        } else {
            panic!("The right child of a left assertion must be a hidden node")
        }
    }

    /// Create a DAG that computes the right assertion of the given `right` child.
    /// The `left` child must be a hidden node.
    ///
    /// _Overall type: (A + B) × C → D where right: B × C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn assertr(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        if let CommitNodeInner::Hidden(_) = left.inner {
            Self::from_inner(CommitNodeInner::AssertR(left, right))
        } else {
            panic!("The left child of a right assertion must be a hidden node")
        }
    }

    /// Create a DAG that computes the pair of the given `left` and `right` child.
    ///
    /// _Overall type: A → B × C where left: A → B and right: A → C_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn pair(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Pair(left, right))
    }

    /// Create a DAG that computes _disconnect_ of the given `left` and `right` child.
    ///
    /// _Overall type: A → B × D where left: 2^256 × A → B × C and right: C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn disconnect(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Disconnect(left, right))
    }

    /// Create a DAG that returns a given witness value as constant.
    /// The value is missing during commitment and inserted during redemption.
    ///
    /// _Overall type: A → B_
    pub fn witness() -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Witness)
    }

    /// Create a DAG that universally fails.
    /// The given `left` and `right` hashes form a block for the CMR computation.
    ///
    /// _Overall type: A → B_
    pub fn fail(left: Cmr, right: Cmr) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Fail(left, right))
    }

    /// Create a hidden DAG that contains the given `hash` as its CMR.
    ///
    /// **This DAG must only be used as child for left or right assertions.**
    ///
    /// _The Bit Machine will crash upon seeing this node._
    pub fn hidden(hash: Cmr) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Hidden(hash))
    }

    /// Create a DAG that computes some black-box function that is associated with the given `jet`.
    ///
    /// _Overall type: A → B where jet: A → B_
    pub fn jet(jet: &'static JetNode<App>) -> Rc<Self> {
        Self::from_inner(CommitNodeInner::Jet(jet))
    }

    /// Create a DAG that takes any input and returns `value` as constant output.
    ///
    /// _Overall type: A → B where value: B_
    pub fn scribe(value: &Value) -> Rc<CommitNode<App>> {
        match value {
            Value::Unit => CommitNode::unit(),
            Value::SumL(l) => {
                let l = CommitNode::scribe(l);
                CommitNode::injl(l)
            }
            Value::SumR(r) => {
                let r = CommitNode::scribe(r);
                CommitNode::injr(r)
            }
            Value::Prod(l, r) => {
                let l = CommitNode::scribe(l);
                let r = CommitNode::scribe(r);
                CommitNode::pair(l, r)
            }
        }
    }

    /// Create a DAG that takes any input and returns bit `0` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_false() -> Rc<Self> {
        CommitNode::injl(CommitNode::unit())
    }

    /// Create a DAG that takes any input and returns bit `1` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_true() -> Rc<Self> {
        CommitNode::injr(CommitNode::unit())
    }

    /// Create a DAG that takes a bit and an input,
    /// such that the `left` child is evaluated on the input if the bit is `1` _(if branch)_
    /// and the `right` child is evaluated on the input otherwise _(else branch)_.
    ///
    /// _Overall type: 2 × A → B where left: A → B and right: A → B_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn cond(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        CommitNode::case(CommitNode::drop(right), CommitNode::drop(left))
    }

    /// Create a DAG that asserts that its child returns `true`, and fails otherwise.
    ///
    /// _Overall type: A → 1 where child: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn assert(child: Rc<Self>) -> Rc<Self> {
        let fail_zeroes_cmr = Cmr::from([
            177, 133, 253, 158, 70, 96, 76, 160, 2, 45, 209, 68, 83, 153, 159, 186, 164, 51, 151,
            174, 72, 121, 107, 12, 64, 35, 186, 249, 151, 31, 21, 102,
        ]);

        CommitNode::comp(
            CommitNode::pair(child, CommitNode::unit()),
            CommitNode::assertr(CommitNode::hidden(fail_zeroes_cmr), CommitNode::unit()),
        )
    }

    /// Create a DAG that computes Boolean _NOT_ of the `child`.
    ///
    /// _Overall type: A → 2 where child: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    #[allow(clippy::should_implement_trait)]
    pub fn not(child: Rc<Self>) -> Rc<Self> {
        CommitNode::comp(
            CommitNode::pair(child, CommitNode::unit()),
            CommitNode::cond(CommitNode::bit_false(), CommitNode::bit_true()),
        )
    }

    /// Create a DAG that computes Boolean _AND_ of the `left` and `right` child.
    ///
    /// _Overall type: A → 2 where left: A → 2 and right: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn and(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        CommitNode::comp(
            CommitNode::pair(left, CommitNode::iden()),
            CommitNode::cond(right, CommitNode::bit_false()),
        )
    }

    /// Create a DAG that computes Boolean _OR_ of the `left` and `right`.
    ///
    /// _Overall type: A → 2 where left: A → 2 and right: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn or(left: Rc<Self>, right: Rc<Self>) -> Rc<Self> {
        CommitNode::comp(
            CommitNode::pair(left, CommitNode::iden()),
            CommitNode::cond(CommitNode::bit_true(), right),
        )
    }

    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&Self> {
        self.inner.get_left()
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&Self> {
        self.inner.get_right()
    }

    /// Create a new DAG, enriched with the witness and computed metadata.
    pub fn finalize<W: WitnessIterator>(
        &self,
        mut witness: W,
    ) -> Result<Rc<RedeemNode<App>>, Error> {
        let root = RefWrapper(self);
        let post_order_it = root.iter_post_order();
        let mut to_finalized: HashMap<RefWrapper<App>, Rc<RedeemNode<App>>> = HashMap::new();

        for commit in post_order_it {
            let left = commit.get_left().map(|x| {
                to_finalized
                    .get(&x)
                    .expect("Children come before parent in post order")
                    .clone()
            });
            let right = commit.get_right().map(|x| {
                to_finalized
                    .get(&x)
                    .expect("Children come before parent in post order")
                    .clone()
            });

            let ty = NodeType::try_from(commit.0)?;
            let value = if let CommitNodeInner::Witness = commit.0.inner {
                Some(witness.next(&ty.target)?)
            } else {
                None
            };
            let imr = imr::compute_imr(
                &commit.0.inner,
                left.clone(),
                right.clone(),
                value.as_ref(),
                &ty,
            );
            let bounds = analysis::compute_bounds(commit.0, left.clone(), right.clone(), &ty);

            // Verbose but necessary thanks to Rust
            let inner = match commit.0.inner {
                CommitNodeInner::Iden => RedeemNodeInner::Iden,
                CommitNodeInner::Unit => RedeemNodeInner::Unit,
                CommitNodeInner::InjL(_) => RedeemNodeInner::InjL(left.unwrap()),
                CommitNodeInner::InjR(_) => RedeemNodeInner::InjR(left.unwrap()),
                CommitNodeInner::Take(_) => RedeemNodeInner::Take(left.unwrap()),
                CommitNodeInner::Drop(_) => RedeemNodeInner::Drop(left.unwrap()),
                CommitNodeInner::Comp(_, _) => RedeemNodeInner::Comp(left.unwrap(), right.unwrap()),
                CommitNodeInner::Case(_, _) => RedeemNodeInner::Case(left.unwrap(), right.unwrap()),
                CommitNodeInner::AssertL(_, _) => {
                    RedeemNodeInner::AssertL(left.unwrap(), right.unwrap())
                }
                CommitNodeInner::AssertR(_, _) => {
                    RedeemNodeInner::AssertR(left.unwrap(), right.unwrap())
                }
                CommitNodeInner::Pair(_, _) => RedeemNodeInner::Pair(left.unwrap(), right.unwrap()),
                CommitNodeInner::Disconnect(_, _) => {
                    RedeemNodeInner::Disconnect(left.unwrap(), right.unwrap())
                }
                CommitNodeInner::Witness => RedeemNodeInner::Witness(value.unwrap()),
                CommitNodeInner::Fail(hl, hr) => RedeemNodeInner::Fail(hl, hr),
                CommitNodeInner::Hidden(h) => RedeemNodeInner::Hidden(h),
                CommitNodeInner::Jet(jet) => RedeemNodeInner::Jet(jet),
            };
            let node = RedeemNode {
                inner,
                cmr: commit.0.cmr,
                imr,
                ty,
                bounds,
            };

            to_finalized.insert(commit, Rc::new(node));
        }

        witness.finish()?;
        Ok(to_finalized.get(&root).unwrap().clone())
    }

    /// Decode a Simplicity program from bits, without the witness data.
    ///
    /// # Usage
    ///
    /// Use this method only if the serialization **does not** include the witness data.
    /// This means, the program simply has no witness during commitment,
    /// or the witness is provided by other means.
    ///
    /// If the serialization contains the witness data, then use [`RedeemNode::decode()`].
    pub fn decode<I: Iterator<Item = u8>>(bits: &mut BitIter<I>) -> Result<Rc<Self>, Error> {
        decode::decode_program_fresh_witness(bits)
    }

    /// Encode a Simplicity program to bits, without witness data.
    ///
    /// # Usage
    ///
    /// Use this method only if the program has no witness data.
    /// Otherwise, add the witness via [`Self::finalize()`] and use [`RedeemNode::encode()`].
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let empty_witness = std::iter::repeat(Value::Unit);
        let program = self.finalize(empty_witness).expect("finalize");
        program.encode(w)
    }
}

impl<App: Application> fmt::Display for CommitNode<App> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        RefWrapper(self).display(
            f,
            |node, f| fmt::Display::fmt(&node.0.inner, f),
            |_, _| Ok(()),
        )
    }
}

/// Wrapper of references to [`CommitNode`].
/// Zero-cost implementation of `Copy`, `Eq` and `Hash` via pointer equality.
#[derive(Debug)]
pub struct RefWrapper<'a, App: Application>(pub &'a CommitNode<App>);

impl_ref_wrapper!(RefWrapper);
