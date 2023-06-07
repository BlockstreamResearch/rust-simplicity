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
use crate::merkle::tmr::Tmr;
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
            CommitNodeInner::Pair(_, _) => Cmr::PAIR_IV,
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
    #[rustfmt::skip]
    const PAIR_IV: Cmr = Cmr(Midstate([
        0x8c, 0x86, 0x65, 0xb4, 0x6b, 0x90, 0x3c, 0x23,
        0x7a, 0x2e, 0x1c, 0x54, 0x77, 0xb6, 0x9a, 0xc3,
        0x28, 0x98, 0x76, 0x61, 0x28, 0x70, 0x92, 0xd3,
        0x6a, 0x3c, 0x99, 0x76, 0x96, 0x85, 0xc6, 0x58,
    ]));

    /// CMRs for the bits 0 and 1 -- injl(unit) and injr(unit) respectively
    #[rustfmt::skip]
    pub const BITS: [Cmr; 2] = [
        Cmr(Midstate([
            0xbd, 0x0c, 0xce, 0x93, 0xe7, 0x13, 0xa2, 0xae,
            0x96, 0x1b, 0xf9, 0x1c, 0x7d, 0x11, 0x3e, 0xdb,
            0x06, 0x71, 0xc7, 0x86, 0x9c, 0x72, 0x25, 0x13,
            0x64, 0x68, 0x2a, 0xc8, 0x97, 0x7e, 0xad, 0xe7,
        ])),
        Cmr(Midstate([
            0x79, 0xa7, 0x0c, 0x6a, 0xe1, 0x18, 0x97, 0xac,
            0xc1, 0x42, 0x8c, 0x38, 0x56, 0x8a, 0x45, 0x22,
            0x2e, 0x7c, 0x3e, 0xa6, 0x4c, 0x66, 0xab, 0x4a,
            0x10, 0x43, 0x24, 0xee, 0x39, 0x1b, 0xff, 0x9d,
        ])),
    ];

    /// Compute the CMR of a word
    ///
    /// This is equal to the IMR of the equivalent scribe, converted to a CMR in
    /// the usual way for jets.
    pub fn const_word_cmr(v: &Value) -> Cmr {
        assert_eq!(v.len().count_ones(), 1);
        let w = 1 + v.len().trailing_zeros() as usize;

        let mut cmr_stack = Vec::with_capacity(33);
        // 1. Compute the CMR for the `scribe` corresponding to this word jet
        let mut bit_idx = 0;
        v.do_each_bit(|bit| {
            cmr_stack.push(Cmr::BITS[usize::from(bit)]);
            let mut j = bit_idx;
            while j & 1 == 1 {
                let right_cmr = cmr_stack.pop().unwrap();
                let left_cmr = cmr_stack.pop().unwrap();
                cmr_stack.push(Cmr::PAIR_IV.update(left_cmr, right_cmr));
                j >>= 1;
            }

            bit_idx += 1;
        });
        assert_eq!(cmr_stack.len(), 1);

        let imr_iv = Cmr::tag_iv(b"Simplicity-Draft\x1fIdentity");
        let imr_pass1 = imr_iv.update_1(cmr_stack[0]);
        // 2. Add TMRs to get the pass-two IMR
        let imr_pass2 = imr_pass1.update(
            Tmr::unit().into_inner().into(),
            Tmr::POWERS_OF_TWO[w - 1].into_inner().into(),
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
    use crate::{CommitNode, Context};

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

    #[test]
    fn bit_cmr() {
        let mut ctx = Context::<crate::jet::Core>::new();
        let unit = CommitNode::unit(&mut ctx);
        let bit0 = CommitNode::injl(&mut ctx, unit.clone());
        assert_eq!(bit0.cmr(), Cmr::BITS[0]);

        let bit1 = CommitNode::injr(&mut ctx, unit);
        assert_eq!(bit1.cmr(), Cmr::BITS[1]);
    }

    #[test]
    fn pair_iv() {
        assert_eq!(
            Cmr::PAIR_IV,
            Cmr::tag_iv(b"Simplicity-Draft\x1fCommitment\x1fpair"),
        );
    }
}
