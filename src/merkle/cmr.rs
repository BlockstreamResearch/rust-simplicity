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

//! # Commitment Merkle roots
//!
//! Used at time of commitment.
//! Importantly, `witness` data and right `disconnect` branches are _not_ included in the hash.

use crate::core::commit::CommitNodeInner;
use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::merkle::common::{CommitMerkleRoot, MerkleRoot};
use bitcoin_hashes::sha256::Midstate;

/// Commitment Merkle root
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cmr(pub(crate) Midstate);

impl_midstate_wrapper!(Cmr);

impl CommitMerkleRoot for Cmr {
    fn get_iv<App: Jet>(node: &CommitNodeInner<App>) -> Self {
        match node {
            CommitNodeInner::Iden => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fiden"),
            CommitNodeInner::Unit => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1funit"),
            CommitNodeInner::InjL(_) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1finjl"),
            CommitNodeInner::InjR(_) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1finjr"),
            CommitNodeInner::Take(_) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1ftake"),
            CommitNodeInner::Drop(_) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fdrop"),
            CommitNodeInner::Comp(_, _) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fcomp"),
            CommitNodeInner::Case(_, _)
            | CommitNodeInner::AssertL(_, _)
            | CommitNodeInner::AssertR(_, _) => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fcase")
            }
            CommitNodeInner::Pair(_, _) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fpair"),
            CommitNodeInner::Disconnect(_, _) => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fdisconnect")
            }
            CommitNodeInner::Witness => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fwitness"),
            CommitNodeInner::Fail(_, _) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1ffail"),
            CommitNodeInner::Hidden(h) => *h,
            CommitNodeInner::Jet(jet) => jet.cmr(),
        }
    }
}

/// Compute the CMR of the given `node`.
pub(crate) fn compute_cmr<App: Jet>(node: &CommitNodeInner<App>) -> Cmr {
    let cmr_iv = Cmr::get_iv(node);

    match node {
        CommitNodeInner::Iden
        | CommitNodeInner::Unit
        | CommitNodeInner::Witness
        | CommitNodeInner::Fail(..)
        | CommitNodeInner::Hidden(..)
        | CommitNodeInner::Jet(..) => cmr_iv,
        CommitNodeInner::InjL(l)
        | CommitNodeInner::InjR(l)
        | CommitNodeInner::Take(l)
        | CommitNodeInner::Drop(l)
        | CommitNodeInner::Disconnect(l, _) => cmr_iv.update_1(l.cmr),
        CommitNodeInner::Comp(l, r)
        | CommitNodeInner::Case(l, r)
        | CommitNodeInner::Pair(l, r)
        | CommitNodeInner::AssertL(l, r)
        | CommitNodeInner::AssertR(l, r) => cmr_iv.update(l.cmr, r.cmr),
    }
}
