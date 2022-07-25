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

use crate::core::{Term, TypedNode};
use crate::impl_midstate_wrapper;
use crate::jet::Application;
use crate::merkle::common::{MerkleRoot, TermMerkleRoot};
use bitcoin_hashes::sha256::Midstate;

/// Commitment Merkle root
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cmr(pub(crate) Midstate);

impl_midstate_wrapper!(Cmr);

impl TermMerkleRoot for Cmr {
    fn get_iv<Witness, App: Application>(term: &Term<Witness, App>) -> Self {
        match term {
            Term::Iden => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fiden"),
            Term::Unit => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1funit"),
            Term::InjL(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1finjl"),
            Term::InjR(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1finjr"),
            Term::Take(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1ftake"),
            Term::Drop(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fdrop"),
            Term::Comp(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fcomp"),
            Term::Case(..) | Term::AssertL(..) | Term::AssertR(..) => {
                Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fcase")
            }
            Term::Pair(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fpair"),
            Term::Disconnect(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fdisconnect"),
            Term::Witness(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fwitness"),
            Term::Fail(..) => Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1ffail"),
            Term::Hidden(h) => *h,
            Term::Jet(j) => j.cmr(),
        }
    }
}

/// Compute the CMR of the given `node` that will be appended to the given `program`
/// at the given `index`.
pub(crate) fn compute_cmr<Witness, App: Application>(
    program: &[TypedNode<Witness, App>],
    node: &Term<Witness, App>,
    index: usize,
) -> Cmr {
    let cmr_iv = Cmr::get_iv(node);

    match *node {
        Term::Iden
        | Term::Unit
        | Term::Witness(..)
        | Term::Fail(..)
        | Term::Hidden(..)
        | Term::Jet(..) => cmr_iv,
        Term::InjL(i) | Term::InjR(i) | Term::Take(i) | Term::Drop(i) | Term::Disconnect(i, _) => {
            cmr_iv.update_1(program[index - i].cmr)
        }
        Term::Comp(i, j)
        | Term::Case(i, j)
        | Term::Pair(i, j)
        | Term::AssertL(i, j)
        | Term::AssertR(i, j) => cmr_iv.update(program[index - i].cmr, program[index - j].cmr),
    }
}
