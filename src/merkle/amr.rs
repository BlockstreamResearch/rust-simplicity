// SPDX-License-Identifier: CC0-1.0

use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::merkle::compact_value;
use crate::types::arrow::FinalArrow;
use crate::value::Word;
use crate::{Cmr, Tmr, Value};
use hashes::sha256::Midstate;

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
        let (b, c) = ty.target.as_sum().unwrap();
        Self::INJL_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), child)
    }

    /// Produce a CMR for an injr combinator
    pub fn injr(ty: &FinalArrow, child: Amr) -> Self {
        let a = &ty.source;
        let (b, c) = ty.target.as_sum().unwrap();
        Self::INJR_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), child)
    }

    /// Produce a CMR for a take combinator
    pub fn take(ty: &FinalArrow, child: Amr) -> Self {
        let (a, b) = ty.source.as_product().unwrap();
        let c = &ty.target;
        Self::TAKE_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), child)
    }

    /// Produce a CMR for a drop combinator
    pub fn drop(ty: &FinalArrow, child: Amr) -> Self {
        let (a, b) = ty.source.as_product().unwrap();
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
        let (sum_a_b, c) = ty.source.as_product().unwrap();
        let (a, b) = sum_a_b.as_sum().unwrap();
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
        let (b, d) = ty.target.as_product().unwrap();
        let c = &right_arrow.source;
        Self::DISCONNECT_IV
            .update(a.tmr().into(), b.tmr().into())
            .update(c.tmr().into(), d.tmr().into())
            .update(left, right)
    }

    /// Produce a CMR for a witness combinator
    pub fn witness(ty: &FinalArrow, value: &Value) -> Self {
        let a = &ty.source;
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
    /// This is equal to the IHR of the equivalent scribe, converted to a CMR in
    /// the usual way for jets.
    pub fn const_word(word: &Word) -> Self {
        Cmr::const_word(word).into()
    }

    #[rustfmt::skip]
    const IDEN_IV: Amr = Amr(Midstate([
        0x45, 0x1f, 0x68, 0x8c, 0x41, 0x50, 0x99, 0x68,
        0x3a, 0x8d, 0x36, 0xa7, 0x09, 0xba, 0x42, 0x49,
        0xe6, 0xde, 0xf7, 0x1b, 0x35, 0x29, 0xb4, 0xc1,
        0xf2, 0xe4, 0xd5, 0x63, 0x85, 0x54, 0x91, 0xaf,
    ]));

    #[rustfmt::skip]
    const UNIT_IV: Amr = Amr(Midstate([
        0xdb, 0xfe, 0xe7, 0x84, 0x3b, 0x59, 0x1e, 0x8f,
        0x7c, 0xe4, 0x85, 0xdc, 0xb5, 0xd6, 0x6b, 0xcc,
        0x23, 0x98, 0x48, 0x10, 0xbd, 0x6b, 0xe5, 0x2a,
        0xa7, 0x6e, 0xaf, 0x2b, 0x25, 0xcb, 0x44, 0x23,
    ]));

    #[rustfmt::skip]
    const INJL_IV: Amr = Amr(Midstate([
        0xba, 0xcb, 0x63, 0x56, 0x69, 0x80, 0xdc, 0xac,
        0x45, 0x59, 0x1f, 0x5a, 0xef, 0x0d, 0x91, 0xd4,
        0x9e, 0xa4, 0xc1, 0xe0, 0x07, 0x2b, 0x1c, 0xe3,
        0xdd, 0x20, 0x75, 0x21, 0xc9, 0xea, 0xd8, 0x08,
    ]));

    #[rustfmt::skip]
    const INJR_IV: Amr = Amr(Midstate([
        0x7b, 0x11, 0x6e, 0xbc, 0x3f, 0x90, 0x9e, 0xef,
        0x13, 0x87, 0x98, 0x71, 0x99, 0x84, 0x4b, 0x91,
        0xbc, 0xc7, 0x40, 0x2c, 0x24, 0x30, 0xc4, 0x85,
        0x29, 0x75, 0xd4, 0x8f, 0x49, 0x0c, 0x54, 0x7a,
    ]));

    #[rustfmt::skip]
    const TAKE_IV: Amr = Amr(Midstate([
        0x41, 0x75, 0xa1, 0x34, 0xe6, 0xb4, 0xd1, 0x5b,
        0xb4, 0x8a, 0xc5, 0x82, 0xd9, 0xbc, 0x83, 0xa2,
        0x94, 0x84, 0xc7, 0xec, 0x27, 0x00, 0x4d, 0xff,
        0x94, 0xaa, 0xe4, 0xcb, 0x5d, 0x87, 0x6f, 0xb6,
    ]));

    #[rustfmt::skip]
    const DROP_IV: Amr = Amr(Midstate([
        0x3b, 0x32, 0x93, 0xab, 0xbc, 0xc9, 0xb9, 0xdb,
        0xe1, 0xbd, 0x70, 0xa3, 0x31, 0x5f, 0x38, 0xff,
        0x26, 0x05, 0xfb, 0x30, 0xe3, 0x1f, 0x3c, 0xee,
        0x7b, 0x25, 0x1a, 0x88, 0x31, 0xc1, 0x56, 0x89,
    ]));

    #[rustfmt::skip]
    const COMP_IV: Amr = Amr(Midstate([
        0x73, 0x7f, 0xd0, 0x53, 0x08, 0xd6, 0x05, 0x00,
        0x3b, 0xde, 0x8e, 0xb2, 0x0a, 0x3b, 0x93, 0x97,
        0xc1, 0xda, 0xaa, 0xe5, 0x43, 0xd8, 0xa4, 0xe4,
        0xfe, 0xef, 0x48, 0x38, 0x76, 0x85, 0x93, 0x84,
    ]));

    #[rustfmt::skip]
    const CASE_IV: Amr = Amr(Midstate([
        0x59, 0x58, 0x14, 0xf9, 0xe0, 0xc8, 0x89, 0xb4,
        0x84, 0x7a, 0xba, 0x1d, 0x51, 0xe1, 0x4e, 0x04,
        0x8f, 0x95, 0x34, 0x4b, 0x2a, 0x43, 0x20, 0x91,
        0xf8, 0x1b, 0xd3, 0xb8, 0x36, 0xec, 0x35, 0x04,
    ]));

    #[rustfmt::skip]
    const ASSERTL_IV: Amr = Amr(Midstate([
        0xd5, 0xdd, 0xd7, 0x41, 0xd7, 0x27, 0xe2, 0x20,
        0x35, 0x56, 0xb0, 0xf8, 0xba, 0x4f, 0x7f, 0xd0,
        0xb4, 0x8f, 0x8d, 0xbf, 0x7a, 0x85, 0x84, 0x87,
        0x9b, 0xab, 0xa8, 0xd6, 0xec, 0xaa, 0x43, 0xef,
    ]));

    #[rustfmt::skip]
    const ASSERTR_IV: Amr = Amr(Midstate([
        0xc5, 0xfe, 0xcd, 0x9c, 0xad, 0xc1, 0x6a, 0x7a,
        0x08, 0xcf, 0x21, 0x5a, 0xe7, 0x58, 0x40, 0xf6,
        0xd2, 0x05, 0xe7, 0xaa, 0x27, 0xf6, 0xff, 0xf2,
        0x6b, 0xfa, 0x1e, 0x6b, 0x3a, 0x0f, 0x76, 0xfa,
    ]));

    #[rustfmt::skip]
    const PAIR_IV: Amr = Amr(Midstate([
        0x79, 0x96, 0xb3, 0xbf, 0xa6, 0x03, 0x36, 0x73,
        0x25, 0x15, 0xa1, 0x24, 0x3f, 0x6b, 0x3c, 0x23,
        0x6b, 0xff, 0x0e, 0xb0, 0x66, 0x2f, 0x8a, 0xdd,
        0x54, 0xaf, 0x0c, 0x98, 0xd1, 0xfd, 0xdd, 0xa7,
    ]));

    #[rustfmt::skip]
    const DISCONNECT_IV: Amr = Amr(Midstate([
        0x27, 0xeb, 0xbc, 0xaa, 0x5a, 0x53, 0x7c, 0x25,
        0xd8, 0xbb, 0x43, 0x40, 0x37, 0xbb, 0x2f, 0xff,
        0x6b, 0x88, 0x15, 0x35, 0x43, 0x5e, 0x6d, 0x60,
        0xa5, 0xf8, 0x5c, 0xd0, 0xf0, 0x5a, 0x61, 0x47,
    ]));

    #[rustfmt::skip]
    const WITNESS_IV: Amr = Amr(Midstate([
        0x6b, 0xc6, 0x20, 0xcb, 0x52, 0xf9, 0x0c, 0x63,
        0x0e, 0xf8, 0xc8, 0x0d, 0x5f, 0x69, 0x0d, 0xb4,
        0xe9, 0x93, 0x0a, 0xd0, 0xfa, 0x70, 0x71, 0x36,
        0x5b, 0x5c, 0x93, 0xe0, 0x39, 0x1b, 0xd7, 0x96,
    ]));

    #[rustfmt::skip]
    const FAIL_IV: Amr = Amr(Midstate([
        0x36, 0x9c, 0xdd, 0x29, 0x3f, 0xd5, 0x8a, 0x85,
        0xa5, 0x90, 0x38, 0x0f, 0x12, 0x20, 0x7e, 0x18,
        0x10, 0x02, 0x0d, 0x39, 0x1c, 0x46, 0xd1, 0xab,
        0xbe, 0xb2, 0x12, 0x2e, 0xa2, 0xfd, 0x55, 0xec,
    ]));
}

#[cfg(test)]
mod tests {
    use super::super::bip340_iv;
    use super::*;

    use crate::jet::Core;
    use crate::node::{ConstructNode, JetConstructible};
    use crate::types;
    use std::sync::Arc;

    #[test]
    fn fixed_amr() {
        types::Context::with_context(|ctx| {
            let node = Arc::<ConstructNode<_>>::jet(&ctx, Core::Verify)
                .finalize_types_non_program()
                .unwrap();
            // Checked against C implementation
            #[rustfmt::skip]
            assert_eq!(
                &node.amr().unwrap().to_string(),
                "cdca2a05e52cefa59dc7a5b0dae22098fb896e3913bfdd446b594e1f9250783e"
            );
        });
    }

    #[test]
    fn ivs() {
        fn check_iv(target: Amr, s: &'static str) {
            let name = s.trim_start_matches("Simplicity\x1fAnnotated\x1f");
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

        check_iv(Amr::IDEN_IV, "Simplicity\x1fAnnotated\x1fiden");
        check_iv(Amr::UNIT_IV, "Simplicity\x1fAnnotated\x1funit");
        check_iv(Amr::INJL_IV, "Simplicity\x1fAnnotated\x1finjl");
        check_iv(Amr::INJR_IV, "Simplicity\x1fAnnotated\x1finjr");
        check_iv(Amr::TAKE_IV, "Simplicity\x1fAnnotated\x1ftake");
        check_iv(Amr::DROP_IV, "Simplicity\x1fAnnotated\x1fdrop");
        check_iv(Amr::COMP_IV, "Simplicity\x1fAnnotated\x1fcomp");
        check_iv(Amr::CASE_IV, "Simplicity\x1fAnnotated\x1fcase");
        check_iv(Amr::ASSERTL_IV, "Simplicity\x1fAnnotated\x1fassertl");
        check_iv(Amr::ASSERTR_IV, "Simplicity\x1fAnnotated\x1fassertr");
        check_iv(Amr::PAIR_IV, "Simplicity\x1fAnnotated\x1fpair");
        check_iv(Amr::DISCONNECT_IV, "Simplicity\x1fAnnotated\x1fdisconnect");
        check_iv(Amr::WITNESS_IV, "Simplicity\x1fAnnotated\x1fwitness");
        check_iv(Amr::FAIL_IV, "Simplicity\x1fAnnotated\x1ffail");
    }
}
