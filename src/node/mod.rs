// Rust Simplicity Library
// Written in 2023 by
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

//! Simplicity Program Nodes
//!
//! The types in this module are used to represent Simplicity expressions as
//! DAGs. The nodes of this DAG are individual combinators along with some
//! cached data about the node, which depends on the specific node type.
//!
//! All nodes represent the root of an expression. Expressions whose source
//! and target types are both unit are called "programs". Generally speaking,
//! any nodes that users hold directly will be programs.
//!
//! There are three main node types:
//!
//! 1. [`ReedemNode`] represents a Simplicity node as it exists on the blockchain.
//!    Every witness node is populated with a correctly-typed [`Value`], every
//!    disconnect node has a child, every non-executed branch of a `case` combinator
//!    is pruned, and there is nothing you can do to modify the expression.
//!
//!    `RedeemNode`s can be executed on the bit machine and have complete types
//!    and resource bounds, and can be serialized in the consensus bit-encoding.
//!
//! 2. [`CommitNode`] represents a Simplicity node as it is *committed to* on
//!    the blockchain. This means that witness and (TODO) disconnect nodes are not
//!    populated, but type inference is complete and resource bounds are
//!    available. `case` combinators may have pruned children (in which case they
//!    are instead considered `assertl` or `assertr` combinators), or not.
//!
//!    There is a bit-encoding for `CommitNode`s which is essentially only used
//!    by this library. It consists of the bit-encoding.of the combinators, fully
//!    shared *except* that witness and disconnect nodes (and their ancestors)
//!    are unshared. No witness data is included.
//!
//!    TODO there is also a human-readable encoding.
//!
//! 3. [`ConstructNode`] represents an "under-construction" Simplicity expression.
//!    These nodes' types are not necessarily complete, and are inferred as the
//!    program is constructed. This is the only node that you can programmatically
//!    construct. It has no encoding, human-readable or bitwise, and is intended
//!    to exist only ephemerally.
//!
//! The following conversions are possible between the node types:
//!
//! 1. [`ConstructNode::finalize_types`] converts a [`ConstructNode`] to a
//!    [`CommitNode`] by setting any free variables, as well as the source and
//!    target of the root node, to unit.
//!
//!    This conversion requires no input from the user but may fail with a type
//!    error, in case of infinitely-sized types or in the case that the unit
//!    bounds cannot be applied.
//!
//! 2. [`CommitNode::rtl_finalize`] converts a [`CommitNode`] to a [`RedeemNode`]
//!    by attaching witnesses to each witness node, and deciding whether to hide
//!    branches for each `case` node. It iterates through the entire program, in
//!    right-to-left post-order, to ensure that both children of every `case`
//!    are encountered before the `case` does, and that every `case` will be
//!    encountered before any witness data that feeds into it (via `comp`).
//!
//!    If witnesses are not available for a given node, this is fine; that node
//!    and its ancestors are not finalized. Any non-finalized nodes must be
//!    pruned, or else the method will return an error.
//!
//! 3. [`CommitNode::unfinalize_types`] converts a [`CommitNode`] to a
//!    [`ConstructNode`] by throwing away all types and re-inferring them. It
//!    cannot fail.
//!
//! 4. [`RedeemNode::unfinalize`] converts a [`RedeemNode`] to a [`CommitNode`]
//!    by throwing away witness and (TODO) disconnect data. It cannot recover
//!    pruned branches so is of limited usefulness, but it is included for
//!    completeness.
//!

use crate::dag::{DagLike, MaxSharing, SharingTracker, SwapChildren};
use crate::jet::Jet;
use crate::{types, Cmr, FailEntropy, Value};

use std::sync::Arc;
use std::{fmt, hash};

mod commit;
mod construct;
mod convert;
mod inner;
mod redeem;

pub use commit::{Commit, CommitData, CommitNode};
pub use construct::{Construct, ConstructData, ConstructNode};
pub use convert::{Converter, Hide, SimpleFinalizer};
pub use inner::Inner;
pub use redeem::{Redeem, RedeemData, RedeemNode};

// This trait should only be implemented on empty types, so we can demand
// every trait bound under the sun. Doing so will make #[derive]s easier
// for downstream users.
pub trait NodeData<J: Jet>:
    Copy + Clone + PartialEq + Eq + PartialOrd + Ord + fmt::Debug + hash::Hash
{
    /// Precomputed data about the node, such as its type arrow or various Merkle roots.
    type CachedData: Clone;

    /// Type of witness data attached to DAGs of this node type. Typically either [`Value`]
    /// or [`NoWitness`].
    type Witness: Clone;

    /// A type which uniquely identifies a node, for purposes of sharing
    /// during iteration over the DAG.
    type SharingId: hash::Hash + Clone + Eq;

    /// Yields the sharing ID for a given type, starting from its CMR and its cached data.
    ///
    /// If the type cannot be uniquely identified (e.g. because it is missing data), then
    /// this method returns `None`. In this case, the node will not be shared with any
    /// other node.
    fn compute_sharing_id(cmr: Cmr, cached_data: &Self::CachedData) -> Option<Self::SharingId>;
}

/// Null data type used as dummy for [`NodeData::Witness`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NoWitness;

pub trait Constructible<W, J: Jet>:
    JetConstructible<J> + WitnessConstructible<W> + CoreConstructible + Sized
{
    fn from_inner(inner: Inner<&Self, J, W>) -> Result<Self, types::Error> {
        match inner {
            Inner::Iden => Ok(Self::iden()),
            Inner::Unit => Ok(Self::unit()),
            Inner::InjL(child) => Ok(Self::injl(child)),
            Inner::InjR(child) => Ok(Self::injr(child)),
            Inner::Take(child) => Ok(Self::take(child)),
            Inner::Drop(child) => Ok(Self::drop_(child)),
            Inner::Comp(left, right) => Self::comp(left, right),
            Inner::Case(left, right) => Self::case(left, right),
            Inner::AssertL(left, r_cmr) => Self::assertl(left, r_cmr),
            Inner::AssertR(l_cmr, right) => Self::assertr(l_cmr, right),
            Inner::Pair(left, right) => Self::pair(left, right),
            Inner::Disconnect(left, right) => Self::disconnect(left, right),
            Inner::Fail(entropy) => Ok(Self::fail(entropy)),
            Inner::Word(ref w) => Ok(Self::const_word(Arc::clone(w))),
            Inner::Jet(j) => Ok(Self::jet(j)),
            Inner::Witness(w) => Ok(Self::witness(w)),
        }
    }
}

impl<W, J: Jet, T> Constructible<W, J> for T where
    T: JetConstructible<J> + WitnessConstructible<W> + CoreConstructible + Sized
{
}

pub trait CoreConstructible: Sized {
    fn iden() -> Self;
    fn unit() -> Self;
    fn injl(child: &Self) -> Self;
    fn injr(child: &Self) -> Self;
    fn take(child: &Self) -> Self;
    fn drop_(child: &Self) -> Self;
    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn case(left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn assertl(left: &Self, right: Cmr) -> Result<Self, types::Error>;
    fn assertr(left: Cmr, right: &Self) -> Result<Self, types::Error>;
    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn disconnect(left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn fail(entropy: FailEntropy) -> Self;
    fn const_word(word: Arc<Value>) -> Self;

    /// Create a DAG that takes any input and returns `value` as constant output.
    ///
    /// _Overall type: A → B where value: B_
    fn scribe(value: &Value) -> Self {
        match value {
            Value::Unit => Self::unit(),
            Value::SumL(l) => {
                let l = Self::scribe(l);
                Self::injl(&l)
            }
            Value::SumR(r) => {
                let r = Self::scribe(r);
                Self::injr(&r)
            }
            Value::Prod(l, r) => {
                let l = Self::scribe(l);
                let r = Self::scribe(r);
                Self::pair(&l, &r).expect("source of scribe has no constraints")
            }
        }
    }

    /// Create a DAG that takes any input and returns bit `0` as constant output.
    ///
    /// _Overall type: A → 2_
    fn bit_false() -> Self {
        let unit = Self::unit();
        Self::injl(&unit)
    }

    /// Create a DAG that takes any input and returns bit `1` as constant output.
    ///
    /// _Overall type: A → 2_
    fn bit_true() -> Self {
        let unit = Self::unit();
        Self::injr(&unit)
    }

    /// Create a DAG that takes a bit and an input,
    /// such that the `left` child is evaluated on the input if the bit is `1` _(if branch)_
    /// and the `right` child is evaluated on the input otherwise _(else branch)_.
    ///
    /// _Overall type: 2 × A → B where `left`: A → B and `right`: A → B_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn cond(left: &Self, right: &Self) -> Result<Self, types::Error> {
        let drop_left = Self::drop_(left);
        let drop_right = Self::drop_(right);
        Self::case(&drop_right, &drop_left)
    }

    /// Create a DAG that asserts that its child returns `true`, and fails otherwise.
    /// The `hash` identifies the assertion and is returned upon failure.
    ///
    /// _Overall type: A → 1 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn assert(child: &Self, hash: Cmr) -> Result<Self, types::Error> {
        let unit = Self::unit();
        let pair_child_unit = Self::pair(child, &unit)?;
        let assertr_hidden_unit = Self::assertr(hash, &unit)?;

        Self::comp(&pair_child_unit, &assertr_hidden_unit)
    }

    /// Create a DAG that computes Boolean _NOT_ of the `child`.
    ///
    /// _Overall type: A → 2 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    #[allow(clippy::should_implement_trait)]
    fn not(child: &Self) -> Result<Self, types::Error> {
        let unit = Self::unit();
        let pair_child_unit = Self::pair(child, &unit)?;
        let bit_true = Self::bit_true();
        let bit_false = Self::bit_false();
        let case_true_false = Self::case(&bit_true, &bit_false)?;

        Self::comp(&pair_child_unit, &case_true_false)
    }

    /// Create a DAG that computes Boolean _AND_ of the `left` and `right` child.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn and(left: &Self, right: &Self) -> Result<Self, types::Error> {
        let iden = Self::iden();
        let pair_left_iden = Self::pair(left, &iden)?;
        let bit_false = Self::bit_false();
        let drop_right = Self::drop_(right);
        let case_false_right = Self::case(&bit_false, &drop_right)?;

        Self::comp(&pair_left_iden, &case_false_right)
    }

    /// Create a DAG that computes Boolean _OR_ of the `left` and `right`.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn or(left: &Self, right: &Self) -> Result<Self, types::Error> {
        let iden = Self::iden();
        let pair_left_iden = Self::pair(left, &iden)?;
        let drop_right = Self::drop_(right);
        let bit_true = Self::bit_true();
        let case_right_true = Self::case(&drop_right, &bit_true)?;

        Self::comp(&pair_left_iden, &case_right_true)
    }
}

pub trait JetConstructible<J: Jet>: Sized {
    fn jet(jet: J) -> Self;
}

pub trait WitnessConstructible<W>: Sized {
    fn witness(witness: W) -> Self;
}

/// A node in a Simplicity expression.
///
/// There are three node types provided by this library: `ConstructNode`, `CommitNode`,
/// and `RedeemNode`, which represent Simplicty programs during construction, at
/// commitment time, and at redemption time, respectively.
///
/// This generic structure is used to define conversions and mapping functions over
/// nodes and DAGs, and allows users to define their own custom node types.
///
/// For equality and hashing purposes, nodes are characterized entirely by their
/// CMR and cached data. Users who create custom nodes should define a custom type
/// for [`NodeData::CachedData`] and think carefully about whether and how to
/// implement the [`std::hash::Hash`] or equality traits.
#[derive(Debug)]
pub struct Node<N: NodeData<J>, J: Jet> {
    inner: Inner<Arc<Node<N, J>>, J, N::Witness>,
    cmr: Cmr,
    data: N::CachedData,
}

impl<N: NodeData<J>, J: Jet> PartialEq for Node<N, J>
where
    N::CachedData: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmr == other.cmr && self.data == other.data
    }
}
impl<N: NodeData<J>, J: Jet> Eq for Node<N, J> where N::CachedData: Eq {}

impl<N: NodeData<J>, J: Jet> hash::Hash for Node<N, J>
where
    N::CachedData: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        self.cmr.hash(h);
        self.data.hash(h);
    }
}

impl<N: NodeData<J>, J: Jet> fmt::Display for Node<N, J>
where
    for<'a> &'a Node<N, J>: DagLike,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.post_order_iter::<MaxSharing<N, J>>().into_display(
            f,
            |node, f| fmt::Display::fmt(&node.inner, f),
            |_, _| Ok(()),
        )
    }
}

impl<N, J> CoreConstructible for Arc<Node<N, J>>
where
    N: NodeData<J>,
    N::CachedData: CoreConstructible,
    J: Jet,
{
    fn iden() -> Self {
        Arc::new(Node {
            cmr: Cmr::iden(),
            data: N::CachedData::iden(),
            inner: Inner::Iden,
        })
    }

    fn unit() -> Self {
        Arc::new(Node {
            cmr: Cmr::unit(),
            data: N::CachedData::unit(),
            inner: Inner::Unit,
        })
    }

    fn injl(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::injl(child.cmr()),
            data: N::CachedData::injl(&child.data),
            inner: Inner::InjL(Arc::clone(child)),
        })
    }

    fn injr(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::injr(child.cmr()),
            data: N::CachedData::injr(&child.data),
            inner: Inner::InjR(Arc::clone(child)),
        })
    }

    fn take(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::take(child.cmr()),
            data: N::CachedData::take(&child.data),
            inner: Inner::Take(Arc::clone(child)),
        })
    }

    fn drop_(child: &Self) -> Self {
        Arc::new(Node {
            cmr: Cmr::drop(child.cmr()),
            data: N::CachedData::drop_(&child.data),
            inner: Inner::Drop(Arc::clone(child)),
        })
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::comp(left.cmr(), right.cmr()),
            data: N::CachedData::comp(&left.data, &right.data)?,
            inner: Inner::Comp(Arc::clone(left), Arc::clone(right)),
        }))
    }

    fn case(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::case(left.cmr(), right.cmr()),
            data: N::CachedData::case(&left.data, &right.data)?,
            inner: Inner::Case(Arc::clone(left), Arc::clone(right)),
        }))
    }

    fn assertl(left: &Self, r_cmr: Cmr) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::case(left.cmr(), r_cmr),
            data: N::CachedData::assertl(&left.data, r_cmr)?,
            inner: Inner::AssertL(Arc::clone(left), r_cmr),
        }))
    }

    fn assertr(l_cmr: Cmr, right: &Self) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::case(l_cmr, right.cmr()),
            data: N::CachedData::assertr(l_cmr, &right.data)?,
            inner: Inner::AssertR(l_cmr, Arc::clone(right)),
        }))
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::pair(left.cmr(), right.cmr()),
            data: N::CachedData::pair(&left.data, &right.data)?,
            inner: Inner::Pair(Arc::clone(left), Arc::clone(right)),
        }))
    }

    fn disconnect(left: &Self, right: &Self) -> Result<Self, types::Error> {
        Ok(Arc::new(Node {
            cmr: Cmr::disconnect(left.cmr()),
            data: N::CachedData::disconnect(&left.data, &right.data)?,
            inner: Inner::Disconnect(Arc::clone(left), Arc::clone(right)),
        }))
    }

    fn fail(entropy: FailEntropy) -> Self {
        Arc::new(Node {
            cmr: Cmr::fail(entropy),
            data: N::CachedData::fail(entropy),
            inner: Inner::Fail(entropy),
        })
    }

    fn const_word(value: Arc<Value>) -> Self {
        Arc::new(Node {
            cmr: Cmr::const_word(&value),
            data: N::CachedData::const_word(Arc::clone(&value)),
            inner: Inner::Word(value),
        })
    }
}

impl<N, J> WitnessConstructible<N::Witness> for Arc<Node<N, J>>
where
    N: NodeData<J>,
    N::CachedData: for<'a> WitnessConstructible<&'a N::Witness>,
    J: Jet,
{
    fn witness(value: N::Witness) -> Self {
        Arc::new(Node {
            cmr: Cmr::witness(),
            data: N::CachedData::witness(&value),
            inner: Inner::Witness(value),
        })
    }
}

impl<N, J> JetConstructible<J> for Arc<Node<N, J>>
where
    N: NodeData<J>,
    N::CachedData: JetConstructible<J>,
    J: Jet,
{
    fn jet(jet: J) -> Self {
        Arc::new(Node {
            cmr: Cmr::jet(jet),
            data: N::CachedData::jet(jet),
            inner: Inner::Jet(jet),
        })
    }
}

impl<N: NodeData<J>, J: Jet> Node<N, J> {
    /// Accessor for the node's "inner value", i.e. its combinator
    pub fn inner(&self) -> &Inner<Arc<Node<N, J>>, J, N::Witness> {
        &self.inner
    }

    /// Accessor for the node's CMR
    pub fn cmr(&self) -> Cmr {
        self.cmr
    }

    /// Accessor for the node's cached data
    pub fn sharing_id(&self) -> Option<N::SharingId> {
        N::compute_sharing_id(self.cmr, &self.data)
    }

    /// Accessor for the node's cached data
    pub fn cached_data(&self) -> &N::CachedData {
        &self.data
    }

    /// Generic conversion function from one type of node to another, with the
    /// ability to prune during the conversion.
    ///
    /// Parameterized over what kind of sharing to use when iterating over the
    /// DAG, and what conversion logic to use.
    ///
    /// See the documentation for [`Converter`] for details.
    pub fn convert<S, M, C>(&self, converter: &mut C) -> Result<Arc<Node<M, J>>, C::Error>
    where
        S: for<'a> SharingTracker<SwapChildren<&'a Self>> + Default,
        M: NodeData<J>,
        C: Converter<N, M, J>,
    {
        let mut converted: Vec<Arc<Node<M, J>>> = vec![];
        for data in self.rtl_post_order_iter::<S>() {
            // First, tell the converter about the iterator state..
            converter.visit_node(&data);

            // Construct an Inner<usize> where pointers are replaced by indices.
            // Note that `map_left_right`'s internal logic will ensure that these
            // `unwrap`s are only called when they will succeed.
            let indexed_inner: Inner<usize, J, &N::Witness> = data
                .node
                .inner
                .as_ref()
                .map_left_right(|_| data.left_index.unwrap(), |_| data.right_index.unwrap());

            // Then, convert witness data, if this is a witness node.
            let witness_inner: Inner<&usize, J, M::Witness> = indexed_inner
                .as_ref()
                .map_witness_result(|wit| converter.convert_witness(&data, wit))?;

            // Then put the converted nodes in place (it's easier to do this in this
            // order because of the way the reference types work out).
            let converted_inner: Inner<Arc<Node<M, J>>, J, M::Witness> =
                witness_inner.map(|idx| Arc::clone(&converted[*idx]));

            // Next, prune case nodes into asserts, if applicable
            let pruned_inner = if let Inner::Case(left, right) = converted_inner {
                let hide = converter.prune_case(&data, &left, &right)?;

                match hide {
                    Hide::Neither => Inner::Case(left, right),
                    Hide::Left => Inner::AssertR(left.cmr(), right),
                    Hide::Right => Inner::AssertL(left, right.cmr()),
                }
            } else {
                converted_inner
            };

            // Finally, construct the node
            converted.push(Arc::new(Node {
                data: converter.convert_data(&data, pruned_inner.as_ref())?,
                cmr: data.node.cmr,
                inner: pruned_inner,
            }));
        }
        Ok(converted.pop().unwrap())
    }
}
