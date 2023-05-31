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
use crate::core::iter::{DagIterable, PostOrderIter, WitnessIterator};
use crate::core::redeem::RedeemNodeInner;
use crate::core::{Context, RedeemNode, Value};
use crate::jet::Jet;
use crate::merkle::amr::Amr;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::Imr;
use crate::types::{
    self,
    arrow::{Arrow, FinalArrow},
};
use crate::{analysis, decode, impl_ref_wrapper, Error};
use std::collections::HashMap;
use std::rc::Rc;
use std::{fmt, io};

/// Underlying combinator of a [`CommitNode`].
/// May contain references to children, hash payloads or jets.
#[derive(Debug)]
pub enum CommitNodeInner<J: Jet> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(Rc<CommitNode<J>>),
    /// Right injection of some child
    InjR(Rc<CommitNode<J>>),
    /// Take of some child
    Take(Rc<CommitNode<J>>),
    /// Drop of some child
    Drop(Rc<CommitNode<J>>),
    /// Composition of a left and right child
    Comp(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Case of a left and right child
    Case(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Left assertion of a left and right child.
    AssertL(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Right assertion of a left and right child.
    AssertR(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Pair of a left and right child
    Pair(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Witness data (missing during commitment, inserted during redemption)
    Witness,
    /// Universal fail
    Fail(Cmr, Cmr),
    /// Hidden CMR
    Hidden(Cmr),
    /// Application jet
    Jet(J),
    /// Constant word
    Word(Value),
}

impl<J: Jet> CommitNodeInner<J> {
    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&CommitNode<J>> {
        match self {
            CommitNodeInner::Iden
            | CommitNodeInner::Unit
            | CommitNodeInner::Witness
            | CommitNodeInner::Fail(_, _)
            | CommitNodeInner::Hidden(_)
            | CommitNodeInner::Jet(_)
            | CommitNodeInner::Word(_) => None,
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
    pub fn get_right(&self) -> Option<&CommitNode<J>> {
        match self {
            CommitNodeInner::Iden
            | CommitNodeInner::Unit
            | CommitNodeInner::Witness
            | CommitNodeInner::Fail(_, _)
            | CommitNodeInner::Hidden(_)
            | CommitNodeInner::Jet(_)
            | CommitNodeInner::Word(_)
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

impl<J: Jet> fmt::Display for CommitNodeInner<J> {
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
            CommitNodeInner::Jet(jet) => write!(f, "jet({})", jet),
            CommitNodeInner::Word(w) => write!(f, "word({})", w),
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
pub struct CommitNode<J: Jet> {
    /// Underlying combinator of the node
    inner: CommitNodeInner<J>,
    /// Commitment Merkle root of the node
    cmr: Cmr,
    /// Source and target types of the node
    arrow: Arrow,
}

impl<J: Jet> PartialEq for CommitNode<J> {
    fn eq(&self, other: &Self) -> bool {
        self.cmr == other.cmr
    }
}
impl<J: Jet> Eq for CommitNode<J> {}

/// Case branches used during execution
#[derive(Debug)]
pub enum UsedCaseBranch {
    /// Only the left branch
    Left,
    /// Only the right branch
    Right,
    /// The left and the right branch
    Both,
}

impl<J: Jet> CommitNode<J> {
    /// Accessor for the node's "inner value", i.e. its combinator
    pub fn inner(&self) -> &CommitNodeInner<J> {
        &self.inner
    }

    /// Accessor for the node's CMR
    pub fn cmr(&self) -> Cmr {
        self.cmr
    }

    /// Accessor for the nodes's unification arrow
    pub fn arrow(&self) -> &Arrow {
        &self.arrow
    }

    /// Return an iterator over the unshared nodes of the program
    pub fn iter(&self) -> PostOrderIter<self::RefWrapper<J>> {
        RefWrapper(self).iter_post_order()
    }

    // FIXME: Compute length without iterating over entire DAG?
    /// Return the number of unshared nodes in the program
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Create a DAG that computes the identity function.
    ///
    /// _Overall type: A → A_
    pub fn iden(context: &mut Context<J>) -> Rc<Self> {
        let inner = CommitNodeInner::Iden;
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_iden(context),
        })
    }

    /// Create a DAG that returns the unit constant.
    ///
    /// _Overall type: A → 1_
    pub fn unit(context: &mut Context<J>) -> Rc<Self> {
        let inner = CommitNodeInner::Unit;
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_unit(context),
        })
    }

    /// Create a DAG that computes the left injection of the given `child`.
    ///
    /// _Overall type: A → B + C where `child`: A → B_
    pub fn injl(context: &mut Context<J>, child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_injl(context, &child.arrow);
        let inner = CommitNodeInner::InjL(child);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        })
    }

    /// Create a DAG that computes the right injection of the given `child`.
    ///
    /// _Overall type: A → B + C where `child`: A → C_
    pub fn injr(context: &mut Context<J>, child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_injr(context, &child.arrow);
        let inner = CommitNodeInner::InjR(child);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        })
    }

    /// Create a DAG with that computes _take_ of the given `child`.
    ///
    /// _Overall type: A × B → C where `child`: A → C_
    pub fn take(context: &mut Context<J>, child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_take(context, &child.arrow);
        let inner = CommitNodeInner::Take(child);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        })
    }

    /// Create a DAG with that computes _drop_ of the given `child`.
    ///
    /// _Overall type: A × B → C where `child`: B → C_
    pub fn drop(context: &mut Context<J>, child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_drop(context, &child.arrow);
        let inner = CommitNodeInner::Drop(child);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        })
    }

    /// Create a DAG that computes the composition of the given `left` and `right` child.
    ///
    /// _Overall type: A → C where `left`: A → B and `right`: B → C_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn comp(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_comp(context, &left.arrow, &right.arrow)?;
        let inner = CommitNodeInner::Comp(left, right);
        Ok(Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        }))
    }

    /// Create a DAG that computes _case_ of the given `left` and `right` child.
    ///
    /// _Overall type: (A + B) × C → D where `left`: A × C → D and `right`: B × C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn case(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_case(context, Some(&left.arrow), Some(&right.arrow))?;
        let inner = CommitNodeInner::Case(left, right);
        Ok(Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        }))
    }

    /// Create a DAG that computes the left assertion of the given `left` child.
    /// The `right` child must be a hidden node.
    ///
    /// _Overall type: (A + B) × C → D where `left`: A × C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn assertl(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Cmr,
    ) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_case(context, Some(&left.arrow), None)?;
        let inner = CommitNodeInner::AssertL(left, Self::hidden(context, right));
        Ok(Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        }))
    }

    /// Create a DAG that computes the right assertion of the given `right` child.
    /// The `left` child must be a hidden node.
    ///
    /// _Overall type: (A + B) × C → D where `right`: B × C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn assertr(
        context: &mut Context<J>,
        left: Cmr,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_case(context, None, Some(&right.arrow))?;
        let inner = CommitNodeInner::AssertR(Self::hidden(context, left), right);
        Ok(Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        }))
    }

    pub fn case_branch(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
        branch: UsedCaseBranch,
    ) -> Result<Rc<Self>, types::Error> {
        match branch {
            UsedCaseBranch::Left => Self::assertl(context, left, right.cmr),
            UsedCaseBranch::Right => Self::assertr(context, left.cmr, right),
            UsedCaseBranch::Both => Self::case(context, left, right),
        }
    }

    /// Create a DAG that computes the pair of the given `left` and `right` child.
    ///
    /// _Overall type: A → B × C where `left`: A → B and `right`: A → C_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn pair(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_pair(context, &left.arrow, &right.arrow)?;
        let inner = CommitNodeInner::Pair(left, right);
        Ok(Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        }))
    }

    /// Create a DAG that computes _disconnect_ of the given `left` and `right` child.
    ///
    /// _Overall type: A → B × D where `left`: 2^256 × A → B × C and `right`: C → D_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn disconnect(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_disconnect(context, &left.arrow, &right.arrow)?;
        let inner = CommitNodeInner::Disconnect(left, right);
        Ok(Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        }))
    }

    /// Create a DAG that returns a given witness value as constant.
    /// The value is missing during commitment and inserted during redemption.
    ///
    /// _Overall type: A → B_
    pub fn witness(context: &mut Context<J>) -> Rc<Self> {
        let inner = CommitNodeInner::Witness;
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_witness(context),
        })
    }

    /// Create a DAG that universally fails.
    /// The given `left` and `right` hashes form a block for the CMR computation.
    ///
    /// _Overall type: A → B_
    pub fn fail(context: &mut Context<J>, left: Cmr, right: Cmr) -> Rc<Self> {
        let inner = CommitNodeInner::Fail(left, right);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_fail(context),
        })
    }

    /// Create a hidden DAG that contains the given `hash` as its CMR.
    ///
    /// **This DAG must only be used as child for left or right assertions.**
    ///
    /// _The Bit Machine will crash upon seeing this node._
    pub fn hidden(context: &mut Context<J>, hash: Cmr) -> Rc<Self> {
        let inner = CommitNodeInner::Hidden(hash);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_hidden(context),
        })
    }

    /// Create a DAG that computes some black-box function that is associated with the given `jet`.
    ///
    /// _Overall type: A → B where jet: A → B_
    pub fn jet(context: &mut Context<J>, jet: J) -> Rc<Self> {
        let arrow = Arrow::for_jet(context, &jet);
        let inner = CommitNodeInner::Jet(jet);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        })
    }

    /// Create a DAG which is a "constant word" jet. This is equivalent to a tree of `pair`s
    /// with bits at the tips.
    ///
    /// _Overall type: () → 2^n for some n between 1 and 32.
    // FIXME if the `word` is not of the correct form we should error out here.
    pub fn const_word(context: &mut Context<J>, word: Value) -> Rc<Self> {
        let arrow = Arrow::for_const_word(context, &word);
        let inner = CommitNodeInner::Word(word);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        })
    }

    /// Create a DAG that takes any input and returns `value` as constant output.
    ///
    /// _Overall type: A → B where value: B_
    pub fn scribe(context: &mut Context<J>, value: &Value) -> Rc<CommitNode<J>> {
        match value {
            Value::Unit => CommitNode::unit(context),
            Value::SumL(l) => {
                let l = CommitNode::scribe(context, l);
                CommitNode::injl(context, l)
            }
            Value::SumR(r) => {
                let r = CommitNode::scribe(context, r);
                CommitNode::injr(context, r)
            }
            Value::Prod(l, r) => {
                let l = CommitNode::scribe(context, l);
                let r = CommitNode::scribe(context, r);
                CommitNode::pair(context, l, r).expect("source of scribe has no constraints")
            }
        }
    }

    /// Create a DAG that takes any input and returns bit `0` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_false(context: &mut Context<J>) -> Rc<Self> {
        let unit = Self::unit(context);
        Self::injl(context, unit)
    }

    /// Create a DAG that takes any input and returns bit `1` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_true(context: &mut Context<J>) -> Rc<Self> {
        let unit = Self::unit(context);
        Self::injr(context, unit)
    }

    /// Create a DAG that takes a bit and an input,
    /// such that the `left` child is evaluated on the input if the bit is `1` _(if branch)_
    /// and the `right` child is evaluated on the input otherwise _(else branch)_.
    ///
    /// _Overall type: 2 × A → B where `left`: A → B and `right`: A → B_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn cond(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let drop_left = Self::drop(context, left);
        let drop_right = Self::drop(context, right);
        Self::case(context, drop_right, drop_left)
    }

    pub fn cond_branch(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
        branch: UsedCaseBranch,
    ) -> Result<Rc<Self>, types::Error> {
        let drop_left = Self::drop(context, left);
        let drop_right = Self::drop(context, right);

        match branch {
            UsedCaseBranch::Left => Self::assertr(context, drop_right.cmr, drop_left),
            UsedCaseBranch::Right => Self::assertl(context, drop_right, drop_left.cmr),
            UsedCaseBranch::Both => Self::case(context, drop_right, drop_left),
        }
    }

    /// Create a DAG that asserts that its child returns `true`, and fails otherwise.
    /// The `hash` identifies the assertion and is returned upon failure.
    ///
    /// _Overall type: A → 1 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn assert(
        context: &mut Context<J>,
        child: Rc<Self>,
        hash: Cmr,
    ) -> Result<Rc<Self>, types::Error> {
        let unit = Self::unit(context);
        let pair_child_unit = Self::pair(context, child, unit)?;
        let unit = Self::unit(context);
        let assertr_hidden_unit = Self::assertr(context, hash, unit)?;

        Self::comp(context, pair_child_unit, assertr_hidden_unit)
    }

    /// Create a DAG that computes Boolean _NOT_ of the `child`.
    ///
    /// _Overall type: A → 2 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    #[allow(clippy::should_implement_trait)]
    pub fn not(context: &mut Context<J>, child: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let unit = Self::unit(context);
        let pair_child_unit = Self::pair(context, child, unit)?;
        let bit_true = Self::bit_true(context);
        let bit_false = Self::bit_false(context);
        let case_true_false = Self::case(context, bit_true, bit_false)?;

        Self::comp(context, pair_child_unit, case_true_false)
    }

    /// Create a DAG that computes Boolean _AND_ of the `left` and `right` child.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn and(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let iden = Self::iden(context);
        let pair_left_iden = Self::pair(context, left, iden)?;
        let bit_false = Self::bit_false(context);
        let drop_right = Self::drop(context, right);
        let case_false_right = Self::case(context, bit_false, drop_right)?;

        Self::comp(context, pair_left_iden, case_false_right)
    }

    /// Create a DAG that computes Boolean _OR_ of the `left` and `right`.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn or(
        context: &mut Context<J>,
        left: Rc<Self>,
        right: Rc<Self>,
    ) -> Result<Rc<Self>, types::Error> {
        let iden = Self::iden(context);
        let pair_left_iden = Self::pair(context, left, iden)?;
        let drop_right = Self::drop(context, right);
        let bit_true = Self::bit_true(context);
        let case_right_true = Self::case(context, drop_right, bit_true)?;

        Self::comp(context, pair_left_iden, case_right_true)
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
    pub fn finalize<W: WitnessIterator>(&self, mut witness: W) -> Result<Rc<RedeemNode<J>>, Error> {
        let root = RefWrapper(self);
        let post_order_it = root.iter_post_order();
        let mut to_finalized: HashMap<RefWrapper<J>, Rc<RedeemNode<J>>> = HashMap::new();
        // Iterate through the DAG again to calculate the value of IMR in the first pass.
        // Unfortunately, we cannot directly mutate the nodes here, because they are RC
        // and we cannot have multiple mutable references to the same object.
        // It is possible to overcome this with Rc<RefCell> as we do in type computation,
        // but that would leak, Rc<RefCell> everywhere through out the code.
        // We also cache Value and NodeType that are needed in the second pass.
        let mut first_pass: HashMap<RefWrapper<J>, (Imr, Option<Value>, FinalArrow)> =
            HashMap::new();

        for commit in post_order_it.clone() {
            let left_imr = commit.get_left().map(|x| {
                first_pass
                    .get(&x)
                    .expect("Children come before parent in post order")
                    .0
            });
            let right_imr = commit.get_right().map(|x| {
                first_pass
                    .get(&x)
                    .expect("Children come before parent in post order")
                    .0
            });
            let final_ty = commit.0.arrow.finalize()?;
            let value = if let CommitNodeInner::Witness = commit.0.inner {
                let v = witness.next(&final_ty.target)?;
                Some(v)
            } else {
                None
            };
            let imr = Imr::compute(
                &commit.0.inner,
                left_imr,
                right_imr,
                value.as_ref(),
                &final_ty,
            );
            first_pass.insert(commit, (imr, value, final_ty));
        }

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
            // The value must exist, and we get ownership by removing it
            let (first_pass_imr, value, ty) = first_pass.remove(&commit).unwrap();
            let imr = Imr::compute_pass2(first_pass_imr, &commit.0.inner, &ty);
            let amr = Amr::compute(
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
                CommitNodeInner::Word(ref w) => RedeemNodeInner::Word(w.clone()),
            };
            let node = RedeemNode {
                inner,
                cmr: commit.0.cmr,
                imr,
                amr,
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

impl<J: Jet> fmt::Display for CommitNode<J> {
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
pub struct RefWrapper<'a, J: Jet>(pub &'a CommitNode<J>);

impl_ref_wrapper!(RefWrapper);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::Core;

    #[test]
    fn occurs_check_error() {
        let mut context = Context::<Core>::new();
        let iden = CommitNode::iden(&mut context);
        let node = CommitNode::disconnect(&mut context, iden.clone(), iden).unwrap();

        if let Err(Error::TypeInference(types::Error::OccursCheck { .. })) =
            node.finalize(std::iter::empty())
        {
        } else {
            panic!("Expected occurs check error")
        }
    }

    #[test]
    fn type_check_error() {
        let mut context = Context::<Core>::new();
        let unit = CommitNode::unit(&mut context);
        let case = CommitNode::case(&mut context, unit.clone(), unit.clone()).unwrap();

        if let Err(types::Error::Bind { .. }) = CommitNode::disconnect(&mut context, case, unit) {
        } else {
            panic!("Expected type check error")
        }
    }
}
