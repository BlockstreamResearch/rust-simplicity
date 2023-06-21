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

use super::FailEntropy;

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
    /// Produce a CMR for an iden combinator
    pub fn iden(ty: &FinalArrow) -> Self {
        let a = &ty.source;
        Self::IDEN_IV.update_1(a.tmr().into())
    }

    /// Produce a CMR for a unit combinator
    pub fn unit(ty: &FinalArrow) -> Self {
        let a = &ty.source;
        Self::UNIT_IV.update_1(a.tmr().into())
    }

    /// Produce a CMR for an injl combinator
    pub fn injl(ty: &FinalArrow, child: Amr) -> Self {
        let a = &ty.source;
        let (b, c) = ty.target.split().unwrap();
        Self::INJL_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), child)
    }

    /// Produce a CMR for an injr combinator
    pub fn injr(ty: &FinalArrow, child: Amr) -> Self {
        let a = &ty.source;
        let (b, c) = ty.target.split().unwrap();
        Self::INJR_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), child)
    }

    /// Produce a CMR for a take combinator
    pub fn take(ty: &FinalArrow, child: Amr) -> Self {
        let (a, b) = ty.source.split().unwrap();
        let c = &ty.target;
        Self::TAKE_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), child)
    }

    /// Produce a CMR for a drop combinator
    pub fn drop(ty: &FinalArrow, child: Amr) -> Self {
        let (a, b) = ty.source.split().unwrap();
        let c = &ty.target;
        Self::DROP_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), child)
    }

    /// Produce a CMR for a comp combinator
    pub fn comp(ty: &FinalArrow, left_arrow: &FinalArrow, left: Amr, right: Amr) -> Self {
        let a = &ty.source;
        let b = &left_arrow.target;
        let c = &ty.target;
        Self::COMP_IV
            .update_1(a.tmr().into())
            .update(b.tmr().into(), c.tmr().into())
            .update(left, right)
    }

    fn case_helper(iv: Amr, ty: &FinalArrow, left: Amr, right: Amr) -> Self {
        let (sum_a_b, c) = ty.source.split().unwrap();
        let (a, b) = sum_a_b.split().unwrap();
        let d = &ty.target;
        iv.update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), d.tmr().into())
            .update(left, right)
    }

    /// Produce a CMR for a case combinator
    pub fn case(ty: &FinalArrow, left: Amr, right: Amr) -> Self {
        Self::case_helper(Self::CASE_IV, ty, left, right)
    }

    /// Produce a CMR for an assertl combinator
    pub fn assertl(ty: &FinalArrow, left: Amr, right: Amr) -> Self {
        Self::case_helper(Self::ASSERTL_IV, ty, left, right)
    }

    /// Produce a CMR for an assertr combinator
    pub fn assertr(ty: &FinalArrow, left: Amr, right: Amr) -> Self {
        Self::case_helper(Self::ASSERTR_IV, ty, left, right)
    }

    /// Produce a CMR for a pair combinator
    pub fn pair(
        ty: &FinalArrow,
        left_arrow: &FinalArrow,
        right_arrow: &FinalArrow,
        left: Amr,
        right: Amr,
    ) -> Self {
        let a = &ty.source;
        let b = &left_arrow.target;
        let c = &right_arrow.target;
        Self::PAIR_IV
            .update_1(a.tmr().into())
            .update(b.tmr().into(), c.tmr().into())
            .update(left, right)
    }

    /// Produce a CMR for a disconnect combinator
    pub fn disconnect(ty: &FinalArrow, right_arrow: &FinalArrow, left: Amr, right: Amr) -> Self {
        let a = &ty.source;
        let (b, d) = ty.target.split().unwrap();
        let c = &right_arrow.source;
        Self::DISCONNECT_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), d.tmr().into())
            .update(left, right)
    }

    /// Produce a CMR for a witness combinator
    pub fn witness(ty: &FinalArrow, value: &Value) -> Self {
        let a = &ty.source; // will always be unit
        let b = &ty.target;
        Self::WITNESS_IV
            .update_1(a.tmr().into())
            .update(b.tmr().into(), Amr::from_byte_array(compact_value(value)))
    }

    /// Produce an AMR for a fail combinator
    pub fn fail(entropy: FailEntropy) -> Self {
        Self::FAIL_IV.update_fail_entropy(entropy)
    }

    /// Produce a CMR for a jet
    pub fn jet<J: Jet>(jet: J) -> Self {
        Cmr::jet(jet).into()
    }

    /// Compute the CMR of a constant word jet
    ///
    /// This is equal to the IMR of the equivalent scribe, converted to a CMR in
    /// the usual way for jets.
    pub fn const_word(v: &Value) -> Self {
        Cmr::const_word(v).into()
    }

    #[rustfmt::skip]
    const IDEN_IV: Amr = Amr(Midstate([
        0xb1, 0x7c, 0x6f, 0x06, 0x66, 0x61, 0xcf, 0xbb,
        0x68, 0x5d, 0xf8, 0x15, 0x01, 0x86, 0xb5, 0x93,
        0xe4, 0xfc, 0xca, 0xcf, 0x8a, 0x87, 0x96, 0xc4,
        0xcb, 0x15, 0xcc, 0xd4, 0x7d, 0x9f, 0xa4, 0x58,
    ]));

    #[rustfmt::skip]
    const UNIT_IV: Amr = Amr(Midstate([
        0x95, 0x0d, 0x69, 0xa6, 0x60, 0x04, 0xa0, 0xe2,
        0x03, 0xe9, 0x5d, 0xe2, 0x3d, 0x4a, 0xbd, 0x1a,
        0xcb, 0x6a, 0xd1, 0x4f, 0x9a, 0x1f, 0x74, 0xfb,
        0x60, 0xc2, 0x47, 0x59, 0x87, 0xc5, 0x78, 0xb7,
    ]));

    #[rustfmt::skip]
    const INJL_IV: Amr = Amr(Midstate([
        0xc2, 0xa1, 0x2f, 0x43, 0x6e, 0x03, 0x31, 0xe4,
        0x36, 0xc8, 0x1e, 0x87, 0xeb, 0x72, 0xc6, 0xd8,
        0xd3, 0x90, 0xa5, 0xa8, 0x07, 0x95, 0xe3, 0x6c,
        0x6b, 0x8f, 0x91, 0xb4, 0x60, 0xde, 0xee, 0xcd,
    ]));

    #[rustfmt::skip]
    const INJR_IV: Amr = Amr(Midstate([
        0x18, 0xc6, 0xf1, 0x38, 0x18, 0xaf, 0xd4, 0xe2,
        0x48, 0xcd, 0xf2, 0x7a, 0xa6, 0x05, 0x7f, 0x91,
        0xca, 0x39, 0x3d, 0xa5, 0x01, 0xde, 0x69, 0x47,
        0xdc, 0x89, 0x61, 0xab, 0x77, 0x58, 0x2a, 0xd1,
    ]));

    #[rustfmt::skip]
    const TAKE_IV: Amr = Amr(Midstate([
        0xb2, 0xb1, 0xed, 0xe0, 0xed, 0xd5, 0xe7, 0x13,
        0x84, 0xf9, 0x13, 0xc7, 0x14, 0xa6, 0x3f, 0x24,
        0x40, 0x5e, 0xf6, 0x1a, 0x01, 0xef, 0x02, 0xad,
        0x40, 0x9a, 0x84, 0xe9, 0x09, 0x5a, 0x4b, 0x13,
    ]));

    #[rustfmt::skip]
    const DROP_IV: Amr = Amr(Midstate([
        0x09, 0xf5, 0x2a, 0x27, 0xa9, 0xfa, 0xc4, 0x6e,
        0x23, 0x26, 0xbc, 0x62, 0xf8, 0x6b, 0x81, 0x9c,
        0xad, 0x70, 0xf2, 0x60, 0x4c, 0x20, 0x48, 0x58,
        0xa9, 0x14, 0xf9, 0x29, 0x15, 0x9a, 0x7b, 0x5f,
    ]));

    #[rustfmt::skip]
    const COMP_IV: Amr = Amr(Midstate([
        0x61, 0x45, 0x23, 0xee, 0x8f, 0x53, 0x1e, 0x69,
        0x6d, 0x68, 0x83, 0x01, 0x41, 0xcb, 0x85, 0x18,
        0x84, 0x1b, 0x0d, 0xd1, 0x98, 0xb9, 0x7b, 0x54,
        0xe8, 0x08, 0xb8, 0x2c, 0x21, 0x09, 0x91, 0xcc,
    ]));

    #[rustfmt::skip]
    const CASE_IV: Amr = Amr(Midstate([
        0x9e, 0x6e, 0x5a, 0x58, 0x03, 0x00, 0x2b, 0xc6,
        0x19, 0xe2, 0xd0, 0x12, 0xfd, 0x09, 0xea, 0x31,
        0x73, 0xab, 0xd0, 0x1d, 0x94, 0x0c, 0xb5, 0x9b,
        0x19, 0x35, 0x89, 0x4e, 0xc3, 0xfe, 0xdf, 0x04,
    ]));

    #[rustfmt::skip]
    const ASSERTL_IV: Amr = Amr(Midstate([
        0x5c, 0x37, 0x60, 0x38, 0x81, 0xc5, 0xe4, 0xc3,
        0x9d, 0x4a, 0x05, 0x7e, 0xf1, 0x2b, 0x86, 0x0b,
        0xbd, 0xcb, 0xb4, 0xdb, 0x08, 0xb6, 0x35, 0x87,
        0x99, 0xe2, 0x3c, 0x4b, 0x94, 0xa9, 0x74, 0x5b,
    ]));

    #[rustfmt::skip]
    const ASSERTR_IV: Amr = Amr(Midstate([
        0xe7, 0x60, 0x4d, 0xc0, 0xa1, 0xe1, 0x6e, 0x73,
        0x8c, 0x7b, 0x13, 0x78, 0xa3, 0xf1, 0x63, 0x06,
        0x19, 0x47, 0x90, 0x87, 0x95, 0xaa, 0x2c, 0x2a,
        0x51, 0x97, 0x6f, 0xcd, 0x0c, 0xd5, 0xb6, 0x8b,
    ]));

    #[rustfmt::skip]
    const PAIR_IV: Amr = Amr(Midstate([
        0xff, 0xe8, 0x3a, 0xc0, 0x36, 0xa2, 0x0b, 0xa2,
        0x16, 0x01, 0xc5, 0xd1, 0xce, 0xfb, 0xb0, 0x51,
        0x25, 0x58, 0xa0, 0x39, 0xf1, 0x41, 0xb6, 0xa4,
        0xcf, 0xbe, 0x33, 0x86, 0x95, 0x62, 0x47, 0xde,
    ]));

    #[rustfmt::skip]
    const DISCONNECT_IV: Amr = Amr(Midstate([
        0x42, 0x8d, 0xfb, 0xc9, 0x8c, 0xa7, 0xe5, 0x53,
        0x8e, 0xeb, 0x05, 0xa6, 0xd1, 0x67, 0xfb, 0x49,
        0x63, 0xf7, 0xe4, 0x2a, 0x74, 0xa3, 0x74, 0x36,
        0x3c, 0x69, 0x40, 0xac, 0x9a, 0xa1, 0xa3, 0xef,
    ]));

    #[rustfmt::skip]
    const WITNESS_IV: Amr = Amr(Midstate([
        0xed, 0xb2, 0x98, 0x95, 0xbe, 0x35, 0x30, 0xb7,
        0xb6, 0xcc, 0xae, 0x11, 0xff, 0xbd, 0x0b, 0x2c,
        0xa8, 0x4a, 0xb6, 0xca, 0xec, 0xf3, 0xb0, 0xcf,
        0x64, 0x1a, 0xa7, 0x10, 0x39, 0xdc, 0x43, 0x97,
    ]));

    #[rustfmt::skip]
    const FAIL_IV: Amr = Amr(Midstate([
        0x4c, 0x3a, 0x32, 0xc9, 0xa5, 0xe4, 0xf4, 0x88,
        0xb0, 0x0c, 0xd6, 0xa2, 0x1f, 0x03, 0x93, 0x50,
        0x9b, 0xde, 0x3a, 0x42, 0xfa, 0x2a, 0xc3, 0x8e,
        0xc7, 0x47, 0x52, 0xc1, 0x67, 0xdc, 0xbf, 0x59,
    ]));

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
        match *node {
            CommitNodeInner::Iden => Self::iden(ty),
            CommitNodeInner::Unit => Self::unit(ty),
            CommitNodeInner::InjL(..) => Self::injl(ty, left.unwrap().amr),
            CommitNodeInner::InjR(..) => Self::injr(ty, left.unwrap().amr),
            CommitNodeInner::Take(..) => Self::take(ty, left.unwrap().amr),
            CommitNodeInner::Drop(..) => Self::drop(ty, left.unwrap().amr),
            CommitNodeInner::Comp(..) => Self::comp(
                ty,
                &left.as_ref().unwrap().ty,
                left.as_ref().unwrap().amr,
                right.as_ref().unwrap().amr,
            ),
            CommitNodeInner::Case(..) => Self::case(ty, left.unwrap().amr, right.unwrap().amr),
            CommitNodeInner::AssertL(_, r_cmr) => {
                Self::assertl(ty, left.unwrap().amr, r_cmr.into())
            }
            CommitNodeInner::AssertR(l_cmr, _) => {
                Self::assertr(ty, l_cmr.into(), left.unwrap().amr)
            }
            CommitNodeInner::Pair(..) => Self::pair(
                ty,
                &left.as_ref().unwrap().ty,
                &right.as_ref().unwrap().ty,
                left.as_ref().unwrap().amr,
                right.as_ref().unwrap().amr,
            ),
            CommitNodeInner::Disconnect(..) => Self::disconnect(
                ty,
                &right.as_ref().unwrap().ty,
                left.as_ref().unwrap().amr,
                right.as_ref().unwrap().amr,
            ),
            CommitNodeInner::Witness => Self::witness(ty, value.unwrap()),
            CommitNodeInner::Fail(entropy) => Self::fail(entropy),
            CommitNodeInner::Jet(j) => Self::jet(j),
            CommitNodeInner::Word(ref w) => Self::const_word(w),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::bip340_iv;
    use super::*;

    use crate::jet::Core;
    use crate::node::{ConstructNode, JetConstructible};
    use std::sync::Arc;

    #[test]
    fn fixed_amr() {
        let node = Arc::<ConstructNode<_>>::jet(Core::Verify)
            .finalize_types_non_program()
            .unwrap();
        // Checked against C implementation
        #[rustfmt::skip]
        assert_eq!(
            node.amr().unwrap(),
            Amr::from_byte_array([
                0x02, 0x0e, 0x84, 0x01, 0x30, 0x30, 0xec, 0x69,
                0xd9, 0xa9, 0x3f, 0xec, 0x71, 0x10, 0xe7, 0x27,
                0xea, 0xd5, 0x12, 0x88, 0x5f, 0xa3, 0xc5, 0x72,
                0xd8, 0xcf, 0xc3, 0x47, 0x2c, 0xa5, 0xc8, 0xe8, 
            ]),
        );
    }

    #[test]
    fn ivs() {
        fn check_iv(target: Amr, s: &'static str) {
            let name = &s[s.rfind('\x1f').unwrap()..];
            // Uncomment this if the IVs ever change
            /*
            let target = Amr(bip340_iv(s.as_bytes()));
            println!("    #[rustfmt::skip]");
            println!("    const {}_IV: Amr = Amr(Midstate([", name.to_ascii_uppercase());
            print!("       "); for ch in &target.0[0..8] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[8..16] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[16..24] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[24..32] { print!(" 0x{:02x},", ch); }; println!();
            println!("    ]));");
            println!();
            */
            assert_eq!(
                target,
                Amr(bip340_iv(s.as_bytes())),
                "mismatch on IV for {}",
                name
            );
        }

        check_iv(Amr::IDEN_IV, "Simplicity-Draft\x1fAnnotated\x1fiden");
        check_iv(Amr::UNIT_IV, "Simplicity-Draft\x1fAnnotated\x1funit");
        check_iv(Amr::INJL_IV, "Simplicity-Draft\x1fAnnotated\x1finjl");
        check_iv(Amr::INJR_IV, "Simplicity-Draft\x1fAnnotated\x1finjr");
        check_iv(Amr::TAKE_IV, "Simplicity-Draft\x1fAnnotated\x1ftake");
        check_iv(Amr::DROP_IV, "Simplicity-Draft\x1fAnnotated\x1fdrop");
        check_iv(Amr::COMP_IV, "Simplicity-Draft\x1fAnnotated\x1fcomp");
        check_iv(Amr::CASE_IV, "Simplicity-Draft\x1fAnnotated\x1fcase");
        check_iv(Amr::ASSERTL_IV, "Simplicity-Draft\x1fAnnotated\x1fassertl");
        check_iv(Amr::ASSERTR_IV, "Simplicity-Draft\x1fAnnotated\x1fassertr");
        check_iv(Amr::PAIR_IV, "Simplicity-Draft\x1fAnnotated\x1fpair");
        check_iv(
            Amr::DISCONNECT_IV,
            "Simplicity-Draft\x1fAnnotated\x1fdisconnect",
        );
        check_iv(Amr::WITNESS_IV, "Simplicity-Draft\x1fAnnotated\x1fwitness");
        check_iv(Amr::FAIL_IV, "Simplicity-Draft\x1fAnnotated\x1ffail");
    }
}
