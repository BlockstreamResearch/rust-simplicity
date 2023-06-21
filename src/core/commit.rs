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

use crate::core::iter::WitnessIterator;
use crate::core::redeem::RedeemNodeInner;
use crate::dag::{self, DagLike, InternalSharing, NoSharing, PostOrderIter};
use crate::jet::Jet;
use crate::merkle::amr::Amr;
use crate::merkle::cmr::Cmr;
use crate::merkle::imr::{FirstPassImr, Imr};
use crate::types::{self, arrow::Arrow};
use crate::{analysis, Error};
use crate::{BitIter, BitWriter, FailEntropy, RedeemNode, Value};
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
    AssertL(Rc<CommitNode<J>>, Cmr),
    /// Right assertion of a left and right child.
    AssertR(Cmr, Rc<CommitNode<J>>),
    /// Pair of a left and right child
    Pair(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Disconnect of a left and right child
    Disconnect(Rc<CommitNode<J>>, Rc<CommitNode<J>>),
    /// Witness data (missing during commitment, inserted during redemption)
    Witness,
    /// Universal fail
    Fail(FailEntropy),
    /// Application jet
    Jet(J),
    /// Constant word
    Word(Value),
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
    pub fn iter(&self) -> PostOrderIter<&Self, NoSharing> {
        <&Self as DagLike>::post_order_iter(self)
    }

    /// Return an iterator over the nodes of the program, returning
    /// refcounted pointers to each node. Each refcounted pointer
    /// is returned only once.
    pub fn rc_iter(self: Rc<Self>) -> PostOrderIter<Rc<Self>, InternalSharing> {
        <Rc<Self> as DagLike>::post_order_iter(self)
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
    pub fn iden() -> Rc<Self> {
        let inner = CommitNodeInner::Iden;
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_iden(),
        })
    }

    /// Create a DAG that returns the unit constant.
    ///
    /// _Overall type: A → 1_
    pub fn unit() -> Rc<Self> {
        let inner = CommitNodeInner::Unit;
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_unit(),
        })
    }

    /// Create a DAG that computes the left injection of the given `child`.
    ///
    /// _Overall type: A → B + C where `child`: A → B_
    pub fn injl(child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_injl(&child.arrow);
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
    pub fn injr(child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_injr(&child.arrow);
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
    pub fn take(child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_take(&child.arrow);
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
    pub fn drop(child: Rc<Self>) -> Rc<Self> {
        let arrow = Arrow::for_drop(&child.arrow);
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
    pub fn comp(left: Rc<Self>, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_comp(&left.arrow, &right.arrow)?;
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
    pub fn case(left: Rc<Self>, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_case(Some(&left.arrow), Some(&right.arrow))?;
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
    pub fn assertl(left: Rc<Self>, right: Cmr) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_case(Some(&left.arrow), None)?;
        let inner = CommitNodeInner::AssertL(left, right);
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
    pub fn assertr(left: Cmr, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_case(None, Some(&right.arrow))?;
        let inner = CommitNodeInner::AssertR(left, right);
        Ok(Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow,
        }))
    }

    pub fn case_branch(
        left: Rc<Self>,
        right: Rc<Self>,
        branch: UsedCaseBranch,
    ) -> Result<Rc<Self>, types::Error> {
        match branch {
            UsedCaseBranch::Left => Self::assertl(left, right.cmr),
            UsedCaseBranch::Right => Self::assertr(left.cmr, right),
            UsedCaseBranch::Both => Self::case(left, right),
        }
    }

    /// Create a DAG that computes the pair of the given `left` and `right` child.
    ///
    /// _Overall type: A → B × C where `left`: A → B and `right`: A → C_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn pair(left: Rc<Self>, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_pair(&left.arrow, &right.arrow)?;
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
    pub fn disconnect(left: Rc<Self>, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let arrow = Arrow::for_disconnect(&left.arrow, &right.arrow)?;
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
    pub fn witness() -> Rc<Self> {
        let inner = CommitNodeInner::Witness;
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_witness(),
        })
    }

    /// Create a DAG that universally fails.
    /// The given `left` and `right` hashes form a block for the CMR computation.
    ///
    /// _Overall type: A → B_
    pub fn fail(entropy: FailEntropy) -> Rc<Self> {
        let inner = CommitNodeInner::Fail(entropy);
        Rc::new(CommitNode {
            cmr: Cmr::compute(&inner),
            inner,
            arrow: Arrow::for_fail(),
        })
    }

    /// Create a DAG that computes some black-box function that is associated with the given `jet`.
    ///
    /// _Overall type: A → B where jet: A → B_
    pub fn jet(jet: J) -> Rc<Self> {
        let arrow = Arrow::for_jet(jet);
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
    pub fn const_word(word: Value) -> Rc<Self> {
        let arrow = Arrow::for_const_word(&word);
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
    pub fn scribe(value: &Value) -> Rc<CommitNode<J>> {
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
                CommitNode::pair(l, r).expect("source of scribe has no constraints")
            }
        }
    }

    /// Create a DAG that takes any input and returns bit `0` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_false() -> Rc<Self> {
        let unit = Self::unit();
        Self::injl(unit)
    }

    /// Create a DAG that takes any input and returns bit `1` as constant output.
    ///
    /// _Overall type: A → 2_
    pub fn bit_true() -> Rc<Self> {
        let unit = Self::unit();
        Self::injr(unit)
    }

    /// Create a DAG that takes a bit and an input,
    /// such that the `left` child is evaluated on the input if the bit is `1` _(if branch)_
    /// and the `right` child is evaluated on the input otherwise _(else branch)_.
    ///
    /// _Overall type: 2 × A → B where `left`: A → B and `right`: A → B_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn cond(left: Rc<Self>, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let drop_left = Self::drop(left);
        let drop_right = Self::drop(right);
        Self::case(drop_right, drop_left)
    }

    pub fn cond_branch(
        left: Rc<Self>,
        right: Rc<Self>,
        branch: UsedCaseBranch,
    ) -> Result<Rc<Self>, types::Error> {
        let drop_left = Self::drop(left);
        let drop_right = Self::drop(right);

        match branch {
            UsedCaseBranch::Left => Self::assertr(drop_right.cmr, drop_left),
            UsedCaseBranch::Right => Self::assertl(drop_right, drop_left.cmr),
            UsedCaseBranch::Both => Self::case(drop_right, drop_left),
        }
    }

    /// Create a DAG that asserts that its child returns `true`, and fails otherwise.
    /// The `hash` identifies the assertion and is returned upon failure.
    ///
    /// _Overall type: A → 1 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn assert(child: Rc<Self>, hash: Cmr) -> Result<Rc<Self>, types::Error> {
        let unit = Self::unit();
        let pair_child_unit = Self::pair(child, unit)?;
        let unit = Self::unit();
        let assertr_hidden_unit = Self::assertr(hash, unit)?;

        Self::comp(pair_child_unit, assertr_hidden_unit)
    }

    /// Create a DAG that computes Boolean _NOT_ of the `child`.
    ///
    /// _Overall type: A → 2 where `child`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    #[allow(clippy::should_implement_trait)]
    pub fn not(child: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let unit = Self::unit();
        let pair_child_unit = Self::pair(child, unit)?;
        let bit_true = Self::bit_true();
        let bit_false = Self::bit_false();
        let case_true_false = Self::case(bit_true, bit_false)?;

        Self::comp(pair_child_unit, case_true_false)
    }

    /// Create a DAG that computes Boolean _AND_ of the `left` and `right` child.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn and(left: Rc<Self>, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let iden = Self::iden();
        let pair_left_iden = Self::pair(left, iden)?;
        let bit_false = Self::bit_false();
        let drop_right = Self::drop(right);
        let case_false_right = Self::case(bit_false, drop_right)?;

        Self::comp(pair_left_iden, case_false_right)
    }

    /// Create a DAG that computes Boolean _OR_ of the `left` and `right`.
    ///
    /// _Overall type: A → 2 where `left`: A → 2 and `right`: A → 2_
    ///
    /// _Type inference will fail if children are not of the correct type._
    pub fn or(left: Rc<Self>, right: Rc<Self>) -> Result<Rc<Self>, types::Error> {
        let iden = Self::iden();
        let pair_left_iden = Self::pair(left, iden)?;
        let drop_right = Self::drop(right);
        let bit_true = Self::bit_true();
        let case_right_true = Self::case(drop_right, bit_true)?;

        Self::comp(pair_left_iden, case_right_true)
    }

    /// Return the left child of the node, if there is such a child.
    pub fn get_left(&self) -> Option<&Self> {
        <&Self as DagLike>::left_child(&self)
    }

    /// Return the right child of the node, if there is such a child.
    pub fn get_right(&self) -> Option<&Self> {
        <&Self as DagLike>::right_child(&self)
    }

    /// Create a new DAG, enriched with the witness and computed metadata.
    pub fn finalize<W: WitnessIterator>(
        &self,
        mut witness: W,
        unshare_witnesses: bool,
    ) -> Result<Rc<RedeemNode<J>>, Error> {
        // Iterate over either the internally-shared DAG, or the internally-shared DAG
        // with witnesses unshared. The borrowck makes this a little irritating to express.
        let mut share = InternalSharing::default();
        let mut unshare = dag::UnshareWitnessDisconnect::new(share.clone());
        let tracker: &mut dyn dag::SharingTracker<&CommitNode<J>> = if unshare_witnesses {
            &mut unshare
        } else {
            &mut share
        };

        // Map from a node's index to its first-pass IMR
        let mut first_pass = vec![];
        // Map from a node's index to its finalized node
        let mut finalized = vec![];
        // IMR of the final node to be iterated over
        for data in self.post_order_iter_with_tracker(tracker) {
            // 0. Obtain data needed for IMR
            let left_imr = data.left_index.map(|idx| first_pass[idx]);
            let right_imr = data.right_index.map(|idx| first_pass[idx]);

            let final_ty = data.node.arrow.finalize()?;
            let value = if let CommitNodeInner::Witness = data.node.inner {
                Some(witness.next(&final_ty.target)?)
            } else {
                None
            };

            // 1. Compute first-pass IMR and record.
            let first_pass_imr = FirstPassImr::compute(
                &data.node.inner,
                left_imr,
                right_imr,
                value.as_ref(),
                &final_ty,
            );
            first_pass.insert(data.index, first_pass_imr);

            // 2. Finalize the node
            let left = data.left_index.map(|idx| Rc::clone(&finalized[idx]));
            let right = data.right_index.map(|idx| Rc::clone(&finalized[idx]));

            let imr = Imr::compute_pass2(first_pass_imr, &final_ty);
            let amr = Amr::compute(
                &data.node.inner,
                left.clone(),
                right.clone(),
                value.as_ref(),
                &final_ty,
            );
            let bounds =
                analysis::compute_bounds(data.node, left.clone(), right.clone(), &final_ty);

            let inner = match data.node.inner {
                CommitNodeInner::Iden => RedeemNodeInner::Iden,
                CommitNodeInner::Unit => RedeemNodeInner::Unit,
                CommitNodeInner::InjL(_) => RedeemNodeInner::InjL(left.unwrap()),
                CommitNodeInner::InjR(_) => RedeemNodeInner::InjR(left.unwrap()),
                CommitNodeInner::Take(_) => RedeemNodeInner::Take(left.unwrap()),
                CommitNodeInner::Drop(_) => RedeemNodeInner::Drop(left.unwrap()),
                CommitNodeInner::Comp(_, _) => RedeemNodeInner::Comp(left.unwrap(), right.unwrap()),
                CommitNodeInner::Case(_, _) => RedeemNodeInner::Case(left.unwrap(), right.unwrap()),
                CommitNodeInner::AssertL(_, cmr) => RedeemNodeInner::AssertL(left.unwrap(), cmr),
                CommitNodeInner::AssertR(cmr, _) => RedeemNodeInner::AssertR(cmr, left.unwrap()),
                CommitNodeInner::Pair(_, _) => RedeemNodeInner::Pair(left.unwrap(), right.unwrap()),
                CommitNodeInner::Disconnect(_, _) => {
                    RedeemNodeInner::Disconnect(left.unwrap(), right.unwrap())
                }
                CommitNodeInner::Witness => RedeemNodeInner::Witness(value.unwrap()),
                CommitNodeInner::Fail(entropy) => RedeemNodeInner::Fail(entropy),
                CommitNodeInner::Jet(jet) => RedeemNodeInner::Jet(jet),
                CommitNodeInner::Word(ref w) => RedeemNodeInner::Word(w.clone()),
            };
            finalized.push(Rc::new(RedeemNode {
                inner,
                cmr: data.node.cmr,
                imr,
                amr,
                ty: final_ty,
                bounds,
            }));
        }
        witness.finish()?;
        Ok(finalized.pop().unwrap())
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
    pub fn decode<I: Iterator<Item = u8>>(
        bits: &mut BitIter<I>,
    ) -> Result<Rc<Self>, crate::decode::Error> {
        crate::decode::decode_expression(bits)
    }

    /// Encode a Simplicity program to bits, without witness data.
    ///
    /// # Usage
    ///
    /// Use this method only if the program has no witness data.
    /// Otherwise, add the witness via [`Self::finalize()`] and use [`RedeemNode::encode()`].
    pub fn encode<W: io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<usize> {
        let empty_witness = std::iter::repeat(Value::Unit);
        let program = self.finalize(empty_witness, false).expect("finalize");
        program.encode(w)
    }
}

impl<J: Jet> fmt::Display for CommitNode<J> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().into_display(
            f,
            |node, f| fmt::Display::fmt(&node.inner, f),
            |_, _| Ok(()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::Core;

    #[test]
    fn occurs_check_error() {
        let iden = CommitNode::<Core>::iden();
        let node = CommitNode::<Core>::disconnect(iden.clone(), iden).unwrap();

        if let Err(Error::Type(types::Error::OccursCheck { .. })) =
            node.finalize(std::iter::empty(), false)
        {
        } else {
            panic!("Expected occurs check error")
        }
    }

    #[test]
    fn type_check_error() {
        let unit = CommitNode::<Core>::unit();
        let case = CommitNode::<Core>::case(unit.clone(), unit.clone()).unwrap();

        if let Err(types::Error::Bind { .. }) = CommitNode::disconnect(case, unit) {
        } else {
            panic!("Expected type check error")
        }
    }
}
