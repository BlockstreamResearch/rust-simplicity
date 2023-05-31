// Rust Simplicity Library
// Written in 2023 by
//   Christian Lewe <clewe@blockstream.com>
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

use crate::core::commit::CommitNodeInner;
use crate::core::{RedeemNode, Value};
use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::merkle::common::{CommitMerkleRoot, MerkleRoot};
use crate::merkle::tmr::Tmr;
use crate::types::arrow::FinalArrow;
use bitcoin_hashes::sha256::Midstate;
use std::rc::Rc;

/// Annotated Merkle root
///
/// A Merkle root that commits to a node's combinator, its source and target type,
/// its witness data (if present), and recursively its children.
///
/// Uniquely identifies a program's structure in terms of types at redemption time.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Amr(pub(crate) Midstate);

impl_midstate_wrapper!(Amr);

impl From<Cmr> for Amr {
    fn from(cmr: Cmr) -> Self {
        cmr.into_inner().into()
    }
}

impl From<Tmr> for Amr {
    fn from(tmr: Tmr) -> Self {
        tmr.into_inner().into()
    }
}

impl CommitMerkleRoot for Amr {
    fn get_iv<J: Jet>(node: &CommitNodeInner<J>) -> Self {
        match *node {
            CommitNodeInner::Iden => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fiden"),
            CommitNodeInner::Unit => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1funit"),
            CommitNodeInner::InjL(_) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1finjl"),
            CommitNodeInner::InjR(_) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1finjr"),
            CommitNodeInner::Take(_) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1ftake"),
            CommitNodeInner::Drop(_) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fdrop"),
            CommitNodeInner::Comp(_, _) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fcomp"),
            CommitNodeInner::Case(_, _) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fcase"),
            CommitNodeInner::AssertL(_, _) => {
                Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fassertl")
            }
            CommitNodeInner::AssertR(_, _) => {
                Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fassertr")
            }
            CommitNodeInner::Pair(_, _) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fpair"),
            CommitNodeInner::Disconnect(_, _) => {
                Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fdisconnect")
            }
            CommitNodeInner::Witness => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1fwitness"),
            CommitNodeInner::Fail(_, _) => Amr::tag_iv(b"Simplicity-Draft\x1fAnnotated\x1ffail"),
            CommitNodeInner::Hidden(h) => h.into(),
            CommitNodeInner::Jet(jet) => jet.amr(),
            CommitNodeInner::Word(..) => Cmr::compute(node).into(),
        }
    }
}

impl Amr {
    /// Compute the AMR of the given node (once finalized).
    ///
    /// Nodes with left children require their finalized left child,
    /// while nodes with right children require their finalized right child.
    /// Witness nodes require their value.
    /// All nodes require their node type.
    pub fn compute<J: Jet>(
        node: &CommitNodeInner<J>,
        left: Option<Rc<RedeemNode<J>>>,
        right: Option<Rc<RedeemNode<J>>>,
        value: Option<&Value>,
        ty: &FinalArrow,
    ) -> Amr {
        let amr_iv = Amr::get_iv(node);

        match *node {
            CommitNodeInner::Hidden(..) | CommitNodeInner::Jet(..) | CommitNodeInner::Word(..) => {
                amr_iv
            }
            CommitNodeInner::Fail(left, right) => amr_iv.update(left.into(), right.into()),
            CommitNodeInner::Witness => amr_iv.update_value(value.unwrap(), ty.target.as_ref()),
            CommitNodeInner::Iden | CommitNodeInner::Unit => {
                let a = &ty.source;
                amr_iv.update_1(a.tmr().into())
            }
            CommitNodeInner::InjL(_) | CommitNodeInner::InjR(_) => {
                let a = &ty.source;
                let (b, c) = ty.target.split().unwrap();
                amr_iv
                    .update(a.tmr().into(), b.tmr().into())
                    .update(c.tmr().into(), left.unwrap().amr)
            }
            CommitNodeInner::Take(_) | CommitNodeInner::Drop(_) => {
                let (a, b) = ty.source.split().unwrap();
                let c = &ty.target;
                amr_iv
                    .update(a.tmr().into(), b.tmr().into())
                    .update(c.tmr().into(), left.unwrap().amr)
            }
            CommitNodeInner::Comp(_, _) | CommitNodeInner::Pair(_, _) => {
                let a = &ty.source;
                let b = &left.as_ref().unwrap().ty.target;
                let c = &right.as_ref().unwrap().ty.target;
                amr_iv
                    .update_1(a.tmr().into())
                    .update(b.tmr().into(), c.tmr().into())
                    .update(left.unwrap().amr, right.unwrap().amr)
            }
            CommitNodeInner::Case(_, _)
            | CommitNodeInner::AssertL(_, _)
            | CommitNodeInner::AssertR(_, _) => {
                let (sum_a_b, c) = ty.source.split().unwrap();
                let (a, b) = sum_a_b.split().unwrap();
                let d = &ty.target;
                amr_iv
                    .update(a.tmr().into(), b.tmr().into())
                    .update(c.tmr().into(), d.tmr().into())
                    .update(left.unwrap().amr, right.unwrap().amr)
            }
            CommitNodeInner::Disconnect(_, _) => {
                let a = &ty.source;
                let (b, d) = ty.target.split().unwrap();
                let c = &right.as_ref().unwrap().ty.source;
                amr_iv
                    .update(a.tmr().into(), b.tmr().into())
                    .update(c.tmr().into(), d.tmr().into())
                    .update(left.unwrap().amr, right.unwrap().amr)
            }
        }
    }
}
