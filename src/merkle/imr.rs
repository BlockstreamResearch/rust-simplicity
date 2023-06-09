// Rust Simplicity Library
// Written in 2022 by
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
use crate::core::Value;
use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::merkle::tmr::Tmr;
use crate::merkle::{CommitMerkleRoot, MerkleRoot};
use crate::types::arrow::FinalArrow;
use bitcoin_hashes::sha256::Midstate;

/// Identity Merkle root
///
/// A Merkle root that commits to a node's combinator, its witness data (if present),
/// and recursively its children.
///
/// Uniquely identifies a program's structure in terms of combinators at redemption time.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Imr(pub(crate) Midstate);

impl_midstate_wrapper!(Imr);

impl From<Cmr> for Imr {
    fn from(cmr: Cmr) -> Self {
        cmr.into_inner().into()
    }
}

impl From<Tmr> for Imr {
    fn from(tmr: Tmr) -> Self {
        tmr.into_inner().into()
    }
}

impl CommitMerkleRoot for Imr {
    fn get_iv<J: Jet>(node: &CommitNodeInner<J>) -> Self {
        match node {
            CommitNodeInner::Disconnect(_, _) => {
                Imr::tag_iv(b"Simplicity-Draft\x1fIdentity\x1fdisconnect")
            }
            CommitNodeInner::Witness => Imr::tag_iv(b"Simplicity-Draft\x1fIdentity\x1fwitness"),
            _ => Cmr::get_iv(node).into_inner().into(),
        }
    }
}

impl Imr {
    /// The IV used in the second pass of IMR computation.
    fn pass_two_iv<J: Jet>(node: &CommitNodeInner<J>) -> Self {
        match node {
            CommitNodeInner::Hidden(_) => Imr::tag_iv(b"Simplicity-Draft\x1fHidden"),
            _ => Imr::tag_iv(b"Simplicity-Draft\x1fIdentity"),
        }
    }

    /// Compute the IMR of the given node (once finalized).
    ///
    /// Nodes with left children require their finalized left child,
    /// while nodes with right children require their finalized right child.
    /// Witness nodes require their value and node type.
    pub(crate) fn compute<J: Jet>(
        node: &CommitNodeInner<J>,
        left: Option<Imr>,
        right: Option<Imr>,
        value: Option<&Value>,
        ty: &FinalArrow,
    ) -> Imr {
        let imr_iv = Imr::get_iv(node);

        match *node {
            CommitNodeInner::Iden
            | CommitNodeInner::Unit
            | CommitNodeInner::Hidden(..)
            | CommitNodeInner::Jet(..) => imr_iv,
            CommitNodeInner::Word(ref value) => Cmr::const_word_cmr(value).into_inner().into(),
            CommitNodeInner::Fail(left, right) => imr_iv.update(left.into(), right.into()),
            CommitNodeInner::InjL(_)
            | CommitNodeInner::InjR(_)
            | CommitNodeInner::Take(_)
            | CommitNodeInner::Drop(_) => imr_iv.update_1(left.unwrap()),
            CommitNodeInner::Comp(_, _)
            | CommitNodeInner::Case(_, _)
            | CommitNodeInner::Pair(_, _)
            | CommitNodeInner::Disconnect(_, _) => imr_iv.update(left.unwrap(), right.unwrap()),
            CommitNodeInner::AssertL(_, r_cmr) => imr_iv.update(left.unwrap(), r_cmr.into()),
            CommitNodeInner::AssertR(l_cmr, _) => imr_iv.update(l_cmr.into(), left.unwrap()),
            CommitNodeInner::Witness => imr_iv.update_value(value.unwrap(), &ty.target),
        }
    }

    /// Do the second pass of the IMR computation. This must be called on the result
    /// of first pass.
    //
    // TODO: None of the fields in [`RedeemNode`] should be pub.
    pub(crate) fn compute_pass2<J: Jet>(
        self, // The IMR computed in the first pass.
        node: &CommitNodeInner<J>,
        ty: &FinalArrow,
    ) -> Imr {
        let first_pass = self;
        let iv = Imr::pass_two_iv(node);
        if let CommitNodeInner::Hidden(_) = node {
            iv.update_1(first_pass)
        } else {
            iv.update_1(first_pass).update(
                Imr::from(<[u8; 32]>::from(ty.source.tmr())),
                Imr::from(<[u8; 32]>::from(ty.target.tmr())),
            )
        }
    }
}
