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
use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::merkle::compact_value;
use crate::types::arrow::FinalArrow;
use crate::{Cmr, RedeemNode, Tmr, Value};
use bitcoin_hashes::sha256::Midstate;
use std::rc::Rc;

use super::bip340_iv;

/// Annotated Merkle root
///
/// A Merkle root that commits to a node's combinator, its source and target type,
/// its witness data (if present), and recursively its children.
///
/// Uniquely identifies a program's structure in terms of types at redemption time.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Amr(Midstate);

impl_midstate_wrapper!(Amr);

impl From<Cmr> for Amr {
    fn from(cmr: Cmr) -> Self {
        Amr::from_byte_array(cmr.to_byte_array())
    }
}

impl From<Tmr> for Amr {
    fn from(tmr: Tmr) -> Self {
        Amr::from_byte_array(tmr.to_byte_array())
    }
}

impl Amr {
    fn get_iv<J: Jet>(node: &CommitNodeInner<J>) -> Self {
        match *node {
            CommitNodeInner::Iden => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fiden")),
            CommitNodeInner::Unit => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1funit")),
            CommitNodeInner::InjL(_) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1finjl")),
            CommitNodeInner::InjR(_) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1finjr")),
            CommitNodeInner::Take(_) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1ftake")),
            CommitNodeInner::Drop(_) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fdrop")),
            CommitNodeInner::Comp(_, _) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fcomp")),
            CommitNodeInner::Case(_, _) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fcase")),
            CommitNodeInner::AssertL(_, _) => {
                Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fassertl"))
            }
            CommitNodeInner::AssertR(_, _) => {
                Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fassertr"))
            }
            CommitNodeInner::Pair(_, _) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fpair")),
            CommitNodeInner::Disconnect(_, _) => {
                Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fdisconnect"))
            }
            CommitNodeInner::Witness => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1fwitness")),
            CommitNodeInner::Fail(_, _) => Amr(bip340_iv(b"Simplicity-Draft\x1fAnnotated\x1ffail")),
            CommitNodeInner::Jet(..) => Cmr::get_iv(node).into(),
            CommitNodeInner::Word(..) => Cmr::compute(node).into(),
        }
    }

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
            CommitNodeInner::Jet(..) | CommitNodeInner::Word(..) => amr_iv,
            CommitNodeInner::Fail(left, right) => amr_iv.update(left.into(), right.into()),
            CommitNodeInner::Witness => {
                let a = &ty.source; // will always be unit
                let b = &ty.target;
                amr_iv.update_1(a.tmr().into()).update(
                    b.tmr().into(),
                    Amr::from_byte_array(compact_value(value.unwrap())),
                )
            }
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
            CommitNodeInner::Comp(_, _) => {
                let a = &ty.source;
                let b = &right.as_ref().unwrap().ty.source;
                let c = &ty.target;
                amr_iv
                    .update_1(a.tmr().into())
                    .update(b.tmr().into(), c.tmr().into())
                    .update(left.unwrap().amr, right.unwrap().amr)
            }
            CommitNodeInner::Pair(_, _) => {
                let a = &ty.source;
                let b = &left.as_ref().unwrap().ty.target;
                let c = &right.as_ref().unwrap().ty.target;
                amr_iv
                    .update_1(a.tmr().into())
                    .update(b.tmr().into(), c.tmr().into())
                    .update(left.unwrap().amr, right.unwrap().amr)
            }
            CommitNodeInner::Case(_, _) => {
                let (sum_a_b, c) = ty.source.split().unwrap();
                let (a, b) = sum_a_b.split().unwrap();
                let d = &ty.target;
                amr_iv
                    .update(a.tmr().into(), b.tmr().into())
                    .update(c.tmr().into(), d.tmr().into())
                    .update(left.unwrap().amr, right.unwrap().amr)
            }
            CommitNodeInner::AssertL(_, r_cmr) => {
                let l_amr = left.as_ref().unwrap().amr;
                let r_amr = r_cmr.into();
                let (sum_a_b, c) = ty.source.split().unwrap();
                let (a, b) = sum_a_b.split().unwrap();
                let d = &ty.target;
                amr_iv
                    .update(a.tmr().into(), b.tmr().into())
                    .update(c.tmr().into(), d.tmr().into())
                    .update(l_amr, r_amr)
            }
            CommitNodeInner::AssertR(l_cmr, _) => {
                let l_amr = l_cmr.into();
                let r_amr = left.as_ref().unwrap().amr;
                let (sum_a_b, c) = ty.source.split().unwrap();
                let (a, b) = sum_a_b.split().unwrap();
                let d = &ty.target;
                amr_iv
                    .update(a.tmr().into(), b.tmr().into())
                    .update(c.tmr().into(), d.tmr().into())
                    .update(l_amr, r_amr)
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::jet::Core;
    use crate::{CommitNode, Context};
    use std::iter;

    #[test]
    fn fixed_amr() {
        let mut ctx = Context::new();
        let node = CommitNode::jet(&mut ctx, Core::Verify)
            .finalize(iter::empty(), true)
            .unwrap();
        // Checked against C implementation
        #[rustfmt::skip]
        assert_eq!(
            node.amr,
            Amr::from_byte_array([
                0x02, 0x0e, 0x84, 0x01, 0x30, 0x30, 0xec, 0x69,
                0xd9, 0xa9, 0x3f, 0xec, 0x71, 0x10, 0xe7, 0x27,
                0xea, 0xd5, 0x12, 0x88, 0x5f, 0xa3, 0xc5, 0x72,
                0xd8, 0xcf, 0xc3, 0x47, 0x2c, 0xa5, 0xc8, 0xe8, 
            ]),
        );
    }
}
