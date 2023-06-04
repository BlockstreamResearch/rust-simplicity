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
use crate::merkle::{CommitMerkleRoot, MerkleRoot};
use bitcoin_hashes::sha256::Midstate;

/// Commitment Merkle root
///
/// A Merkle root that commits to a node's combinator and recursively its children.
///
/// Importantly, witness data and right disconnect branches are _not_ included in the hash.
/// This makes these elements malleable while preserving program identity (SegWit, delegation).
///
/// Uniquely identifies a program's structure in terms of combinators at commitment time.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cmr(pub(crate) Midstate);

impl_midstate_wrapper!(Cmr);

impl CommitMerkleRoot for Cmr {
    fn get_iv<J: Jet>(node: &CommitNodeInner<J>) -> Self {
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
            CommitNodeInner::Jet(j) => Cmr::tag_iv(b"Simplicity-Draft\x1fJet").update_1(j.cmr()),
            CommitNodeInner::Word(_) => Cmr::tag_iv(b"Simplicity-Draft\x1fIdentity"),
        }
    }
}

impl Cmr {
    /// Compute the CMR of a word
    ///
    /// This is equal to the IMR of the equivalent scribe, converted to a CMR in
    /// the usual way for jets.
    // FIXME avoid calling `powers_of_two_vec`; also replace this "construct the scribe then
    //  compute its CMR" logic with the inline algorithm from the C code, which avoids any
    //  allocations.
    pub fn const_word_cmr(v: &Value) -> Cmr {
        assert_eq!(v.len().count_ones(), 1);
        let w = 1 + v.len().trailing_zeros() as usize;
        // 1. compute scribe pass-one IMR
        let scribe = crate::core::CommitNode::<crate::jet::Core>::scribe(
            &mut crate::core::Context::new(),
            v,
        );
        let imr_iv = Cmr::tag_iv(b"Simplicity-Draft\x1fIdentity");
        let imr_pass1 = imr_iv.update_1(scribe.cmr());
        // 2. Add TMRs to get the pass-two IMR
        let types = crate::types::Type::powers_of_two(15);
        let imr_pass2 = imr_pass1.update(
            crate::merkle::tmr::Tmr::unit().into_inner().into(),
            types[w - 1].expect_finalized().tmr().0.into_inner().into(),
        );
        // 3. Convert to a jet CMR
        Cmr::tag_iv(b"Simplicity-Draft\x1fJet").update_1(imr_pass2)
    }

    /// Compute the CMR of the given node.
    pub(crate) fn compute<J: Jet>(node: &CommitNodeInner<J>) -> Cmr {
        let cmr_iv = Cmr::get_iv(node);

        match node {
            CommitNodeInner::Iden
            | CommitNodeInner::Unit
            | CommitNodeInner::Witness
            | CommitNodeInner::Hidden(..)
            | CommitNodeInner::Jet(..) => cmr_iv,
            CommitNodeInner::Word(ref w) => Cmr::const_word_cmr(w),
            CommitNodeInner::Fail(left, right) => cmr_iv.update(*left, *right),
            CommitNodeInner::InjL(l)
            | CommitNodeInner::InjR(l)
            | CommitNodeInner::Take(l)
            | CommitNodeInner::Drop(l)
            | CommitNodeInner::Disconnect(l, _) => cmr_iv.update_1(l.cmr()),
            CommitNodeInner::Comp(l, r)
            | CommitNodeInner::Case(l, r)
            | CommitNodeInner::Pair(l, r)
            | CommitNodeInner::AssertL(l, r)
            | CommitNodeInner::AssertR(l, r) => cmr_iv.update(l.cmr(), r.cmr()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn cmr_display_unit() {
        let mut ctx = crate::Context::<crate::jet::Core>::new();
        let c = crate::CommitNode::unit(&mut ctx);

        assert_eq!(
            c.cmr().to_string(),
            "62274a89833ece8ba5ff57b28118c0063d3d4a85dd25aae06f87617604402715"
        );
        assert_eq!(format!("{:.8}", c.cmr()), "62274a89");

        assert_eq!(
            Cmr::from_str("62274a89833ece8ba5ff57b28118c0063d3d4a85dd25aae06f87617604402715"),
            Ok(c.cmr()),
        );
    }

    #[test]
    fn fixed_const_word_cmr() {
        // Checked against C implementation
        let bit0 = Value::SumL(Box::new(Value::Unit));
        #[rustfmt::skip]
        assert_eq!(
            Cmr::const_word_cmr(&bit0),
            [
                0xb1, 0xaa, 0x07, 0xfb, 0x32, 0xc5, 0xa4, 0xe5,
                0xf5, 0xf9, 0x11, 0x7b, 0x45, 0xbf, 0xf8, 0xb3,
                0x51, 0xdc, 0x1d, 0x59, 0x80, 0x47, 0xeb, 0x64,
                0x70, 0x3e, 0x36, 0xa6, 0x97, 0x19, 0x24, 0x17, 
            ].into(),
        );
    }
}
