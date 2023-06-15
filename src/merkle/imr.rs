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
use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::types::arrow::FinalArrow;
use crate::{Cmr, Tmr, Value};
use bitcoin_hashes::sha256::Midstate;

use super::{bip340_iv, compact_value};

/// Identity Merkle root
///
/// A Merkle root that commits to a node's combinator, its witness data (if present),
/// and recursively its children.
///
/// Uniquely identifies a program's structure in terms of combinators at redemption time.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Imr(Midstate);

impl_midstate_wrapper!(Imr);

impl From<Cmr> for Imr {
    fn from(cmr: Cmr) -> Self {
        Imr::from_byte_array(cmr.to_byte_array())
    }
}

impl From<Tmr> for Imr {
    fn from(tmr: Tmr) -> Self {
        Imr::from_byte_array(tmr.to_byte_array())
    }
}

impl Imr {
    fn get_iv<J: Jet>(node: &CommitNodeInner<J>) -> Self {
        match node {
            CommitNodeInner::Disconnect(_, _) => {
                Imr(bip340_iv(b"Simplicity-Draft\x1fIdentity\x1fdisconnect"))
            }
            CommitNodeInner::Witness => Imr(bip340_iv(b"Simplicity-Draft\x1fIdentity\x1fwitness")),
            _ => Cmr::get_iv(node).into(),
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
            CommitNodeInner::Iden | CommitNodeInner::Unit | CommitNodeInner::Jet(..) => imr_iv,
            CommitNodeInner::Word(ref value) => Cmr::const_word_cmr(value).into(),
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
            CommitNodeInner::Witness => {
                use bitcoin_hashes::{sha256, HashEngine};

                // 1 Bit-wise hash of `value`
                let value_hash = compact_value(value.unwrap());
                // 2 Hash of hash of `value` and TMR of `value_type`
                let mut engine = sha256::HashEngine::from_midstate(imr_iv.0, 0);
                engine.input(&value_hash[..]);
                engine.input(ty.target.tmr().as_ref());
                Imr(engine.midstate())
            }
        }
    }

    /// Do the second pass of the IMR computation. This must be called on the result
    /// of first pass.
    pub(crate) fn compute_pass2(
        self, // The IMR computed in the first pass.
        ty: &FinalArrow,
    ) -> Imr {
        let first_pass = self;
        let iv = Imr(bip340_iv(b"Simplicity-Draft\x1fIdentity"));
        iv.update_1(first_pass)
            .update(ty.source.tmr().into(), ty.target.tmr().into())
    }
}
