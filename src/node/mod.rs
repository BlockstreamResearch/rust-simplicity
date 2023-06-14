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

use crate::dag::{DagLike, InternalSharing};
use crate::jet::Jet;
use crate::{types, Cmr, Context, FailEntropy, Value};

use std::sync::Arc;
use std::{fmt, hash};

mod inner;

pub use inner::Inner;

// This trait should only be implemented on empty types, so we can demand
// every trait bound under the sun. Doing so will make #[derive]s easier
// for downstream users.
pub trait NodeData<J: Jet>:
    Copy + Clone + PartialEq + Eq + PartialOrd + Ord + fmt::Debug + hash::Hash
{
    type CachedData: Clone;
    type Witness: Clone;
}

/// Null data type used as dummy for [`NodeData::Witness`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct NoWitness;

pub trait Constructible<W, J: Jet>: Sized {
    fn iden(ctx: &mut Context<J>) -> Self;
    fn unit(ctx: &mut Context<J>) -> Self;
    fn injl(ctx: &mut Context<J>, child: &Self) -> Self;
    fn injr(ctx: &mut Context<J>, child: &Self) -> Self;
    fn take(ctx: &mut Context<J>, child: &Self) -> Self;
    fn drop_(ctx: &mut Context<J>, child: &Self) -> Self;
    fn comp(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn case(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn assertl(ctx: &mut Context<J>, left: &Self, right: Cmr) -> Result<Self, types::Error>;
    fn assertr(ctx: &mut Context<J>, left: Cmr, right: &Self) -> Result<Self, types::Error>;
    fn pair(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn disconnect(ctx: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error>;
    fn witness(ctx: &mut Context<J>, witness: W) -> Self;
    fn fail(ctx: &mut Context<J>, entropy: FailEntropy) -> Self;
    fn jet(ctx: &mut Context<J>, jet: J) -> Self;
    fn const_word(ctx: &mut Context<J>, word: Arc<Value>) -> Self;

    /// Create a DAG that takes any input and returns `value` as constant output.
    ///
    /// _Overall type: A → B where value: B_
    fn scribe(context: &mut Context<J>, value: &Value) -> Self {
        match value {
            Value::Unit => Self::unit(context),
            Value::SumL(l) => {
                let l = Self::scribe(context, l);
                Self::injl(context, &l)
            }
            Value::SumR(r) => {
                let r = Self::scribe(context, r);
                Self::injr(context, &r)
            }
            Value::Prod(l, r) => {
                let l = Self::scribe(context, l);
                let r = Self::scribe(context, r);
                Self::pair(context, &l, &r).expect("source of scribe has no constraints")
            }
        }
    }

    /// Create a DAG that takes any input and returns bit `0` as constant output.
    ///
    /// _Overall type: A → 2_
    fn bit_false(context: &mut Context<J>) -> Self {
        let unit = Self::unit(context);
        Self::injl(context, &unit)
    }

    /// Create a DAG that takes any input and returns bit `1` as constant output.
    ///
    /// _Overall type: A → 2_
    fn bit_true(context: &mut Context<J>) -> Self {
        let unit = Self::unit(context);
        Self::injr(context, &unit)
    }

    /// Create a DAG that takes a bit and an input,
    /// such that the `left` child is evaluated on the input if the bit is `1` _(if branch)_
    /// and the `right` child is evaluated on the input otherwise _(else branch)_.
    ///
    /// _Overall type: 2 × A → B where `left`: A → B and `right`: A → B_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn cond(context: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error> {
        let drop_left = Self::drop_(context, left);
        let drop_right = Self::drop_(context, right);
        Self::case(context, &drop_right, &drop_left)
    }

    /// Create a DAG that asserts that its child returns `true`, and fails otherwise.
    /// The `hash` identifies the assertion and is returned upon failure.
    ///
    /// _Overall type: A → 1 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn assert(context: &mut Context<J>, child: &Self, hash: Cmr) -> Result<Self, types::Error> {
        let unit = Self::unit(context);
        let pair_child_unit = Self::pair(context, child, &unit)?;
        let assertr_hidden_unit = Self::assertr(context, hash, &unit)?;

        Self::comp(context, &pair_child_unit, &assertr_hidden_unit)
    }

    /// Create a DAG that computes Boolean _NOT_ of the `child`.
    ///
    /// _Overall type: A → 2 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    #[allow(clippy::should_implement_trait)]
    fn not(context: &mut Context<J>, child: &Self) -> Result<Self, types::Error> {
        let unit = Self::unit(context);
        let pair_child_unit = Self::pair(context, child, &unit)?;
        let bit_true = Self::bit_true(context);
        let bit_false = Self::bit_false(context);
        let case_true_false = Self::case(context, &bit_true, &bit_false)?;

        Self::comp(context, &pair_child_unit, &case_true_false)
    }

    /// Create a DAG that computes Boolean _AND_ of the `left` and `right` child.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn and(context: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error> {
        let iden = Self::iden(context);
        let pair_left_iden = Self::pair(context, left, &iden)?;
        let bit_false = Self::bit_false(context);
        let drop_right = Self::drop_(context, right);
        let case_false_right = Self::case(context, &bit_false, &drop_right)?;

        Self::comp(context, &pair_left_iden, &case_false_right)
    }

    /// Create a DAG that computes Boolean _OR_ of the `left` and `right`.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    fn or(context: &mut Context<J>, left: &Self, right: &Self) -> Result<Self, types::Error> {
        let iden = Self::iden(context);
        let pair_left_iden = Self::pair(context, left, &iden)?;
        let drop_right = Self::drop_(context, right);
        let bit_true = Self::bit_true(context);
        let case_right_true = Self::case(context, &drop_right, &bit_true)?;

        Self::comp(context, &pair_left_iden, &case_right_true)
    }
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
        self.post_order_iter::<InternalSharing>().into_display(
            f,
            |node, f| fmt::Display::fmt(&node.inner, f),
            |_, _| Ok(()),
        )
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
}
