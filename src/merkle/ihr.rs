// SPDX-License-Identifier: CC0-1.0

use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::types::arrow::FinalArrow;
use crate::value::Word;
use crate::{Cmr, Tmr, Value};
use hashes::sha256::Midstate;

use super::{bip340_iv, compact_value, FailEntropy};

/// Identity Merkle root (first pass)
///
/// A Merkle root that commits to a node's combinator, its witness data (if present),
/// and recursively its children. Used as input to the [`Ihr`] type which is probably
/// actually what you want.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FirstPassIhr(Midstate);

impl_midstate_wrapper!(FirstPassIhr);

/// Identity Merkle root
///
/// A Merkle root that commits to a node's combinator, its witness data (if present),
/// its source and target types, and recursively its children.
///
/// Uniquely identifies a program's structure in terms of combinators at redemption time.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ihr(Midstate);

impl_midstate_wrapper!(Ihr);

impl From<Cmr> for FirstPassIhr {
    fn from(cmr: Cmr) -> Self {
        FirstPassIhr::from_byte_array(cmr.to_byte_array())
    }
}

impl From<Cmr> for Ihr {
    fn from(cmr: Cmr) -> Self {
        Ihr::from_byte_array(cmr.to_byte_array())
    }
}

impl From<Tmr> for Ihr {
    fn from(tmr: Tmr) -> Self {
        Ihr::from_byte_array(tmr.to_byte_array())
    }
}

impl FirstPassIhr {
    /// Produce a CMR for an iden combinator
    pub const fn iden() -> Self {
        Self::IDEN_IV
    }

    /// Produce a CMR for a unit combinator
    pub const fn unit() -> Self {
        Self::UNIT_IV
    }

    /// Produce a CMR for an injl combinator
    pub fn injl(child: FirstPassIhr) -> Self {
        Self::INJL_IV.update_1(child)
    }

    /// Produce a CMR for an injr combinator
    pub fn injr(child: FirstPassIhr) -> Self {
        Self::INJR_IV.update_1(child)
    }

    /// Produce a CMR for a take combinator
    pub fn take(child: FirstPassIhr) -> Self {
        Self::TAKE_IV.update_1(child)
    }

    /// Produce a CMR for a drop combinator
    pub fn drop(child: FirstPassIhr) -> Self {
        Self::DROP_IV.update_1(child)
    }

    /// Produce a CMR for a comp combinator
    pub fn comp(left: FirstPassIhr, right: FirstPassIhr) -> Self {
        Self::COMP_IV.update(left, right)
    }

    /// Produce a CMR for a case combinator
    pub fn case(left: FirstPassIhr, right: FirstPassIhr) -> Self {
        Self::CASE_IV.update(left, right)
    }

    /// Produce a CMR for a pair combinator
    pub fn pair(left: FirstPassIhr, right: FirstPassIhr) -> Self {
        Self::PAIR_IV.update(left, right)
    }

    /// Produce a CMR for a disconnect combinator
    pub fn disconnect(left: FirstPassIhr, right: FirstPassIhr) -> Self {
        Self::DISCONNECT_IV.update(left, right)
    }

    /// Produce a CMR for a witness combinator
    pub fn witness(ty: &FinalArrow, value: &Value) -> Self {
        use hashes::{sha256, HashEngine};

        // 1 Bit-wise hash of `value`
        let value_hash = compact_value(value);
        // 2 Hash of hash of `value` and TMR of `value_type`
        let mut engine = sha256::HashEngine::from_midstate(Self::WITNESS_IV.0, 0);
        engine.input(&value_hash[..]);
        engine.input(ty.target.tmr().as_ref());
        FirstPassIhr(engine.midstate())
    }

    /// Produce an IHR for a fail combinator
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
    const IDEN_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x54, 0x1a, 0x1a, 0x69, 0xbd, 0x4b, 0xcb, 0xda,
        0x7f, 0x34, 0x31, 0x0e, 0x30, 0x78, 0xf7, 0x26,
        0x44, 0x31, 0x22, 0xfb, 0xcc, 0x1c, 0xb5, 0x36,
        0x0c, 0x78, 0x64, 0xec, 0x0d, 0x32, 0x3a, 0xc0,
    ]));

    #[rustfmt::skip]
    const UNIT_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0xc4, 0x0a, 0x10, 0x26, 0x3f, 0x74, 0x36, 0xb4,
        0x16, 0x0a, 0xcb, 0xef, 0x1c, 0x36, 0xfb, 0xa4,
        0xbe, 0x4d, 0x95, 0xdf, 0x18, 0x1a, 0x96, 0x8a,
        0xfe, 0xab, 0x5e, 0xac, 0x24, 0x7a, 0xdf, 0xf7,
    ]));

    #[rustfmt::skip]
    const INJL_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x54, 0xe9, 0x1d, 0x18, 0xd8, 0xf8, 0x1f, 0x6d,
        0x29, 0x86, 0xbb, 0x58, 0x47, 0x9a, 0x54, 0xeb,
        0x63, 0x0e, 0x95, 0x23, 0xb6, 0x9e, 0xe8, 0x53,
        0x29, 0x80, 0xd0, 0x55, 0x58, 0x19, 0x4f, 0x15,
    ]));

    #[rustfmt::skip]
    const INJR_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0xd7, 0x0f, 0xfd, 0xce, 0x97, 0x77, 0x7b, 0x4d,
        0xfe, 0x31, 0xfd, 0x9f, 0xf5, 0xd0, 0x17, 0xa6,
        0x30, 0x5d, 0x7e, 0xc6, 0x0d, 0xf3, 0xb1, 0xbf,
        0x6d, 0x25, 0xe8, 0x16, 0x33, 0xde, 0xd4, 0xbf,
    ]));

    #[rustfmt::skip]
    const TAKE_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x50, 0x5f, 0xc0, 0x81, 0xb5, 0xba, 0x2a, 0xcd,
        0x09, 0x50, 0x67, 0xc3, 0xdf, 0xb8, 0xea, 0x12,
        0x6f, 0xa1, 0x5d, 0x55, 0xcb, 0x21, 0x1e, 0x6a,
        0xed, 0x34, 0xe8, 0xd1, 0xe3, 0x7a, 0xf0, 0xfa,
    ]));

    #[rustfmt::skip]
    const DROP_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x8a, 0x30, 0x8d, 0x38, 0xa1, 0x13, 0xa2, 0x60,
        0xb4, 0xc7, 0x14, 0x5a, 0xbd, 0xc5, 0x22, 0x4d,
        0xeb, 0x70, 0x13, 0x79, 0x59, 0x0e, 0x0c, 0x8c,
        0x38, 0x86, 0x0b, 0xab, 0x12, 0x71, 0xa8, 0xa8,
    ]));

    #[rustfmt::skip]
    const COMP_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x57, 0xec, 0x23, 0xa2, 0xa4, 0x77, 0x8e, 0x01,
        0x58, 0xa6, 0x21, 0x7a, 0xea, 0x3e, 0xf7, 0x42,
        0x8b, 0xa0, 0x90, 0x92, 0x73, 0xb9, 0x73, 0xfa,
        0x14, 0x32, 0xa9, 0x27, 0x84, 0x3e, 0x92, 0x7a,
    ]));

    #[rustfmt::skip]
    const CASE_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x29, 0x5e, 0x2a, 0x6d, 0xc8, 0xc5, 0xce, 0x59,
        0xe4, 0xed, 0xcf, 0xe9, 0xb4, 0xd8, 0xf7, 0x64,
        0x13, 0x3a, 0xa5, 0x51, 0x4b, 0xd3, 0xee, 0x8b,
        0x4b, 0x75, 0xec, 0x8f, 0x4d, 0xeb, 0x08, 0xbe,
    ]));

    #[rustfmt::skip]
    const PAIR_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x7d, 0x5e, 0x6d, 0xac, 0x15, 0xb1, 0x42, 0x8a,
        0x0d, 0x26, 0x0c, 0x94, 0x29, 0xdb, 0xe8, 0x89,
        0x65, 0x93, 0xf3, 0x1f, 0x70, 0x86, 0x27, 0xee,
        0x75, 0xb2, 0x7e, 0xee, 0xfd, 0xd0, 0x50, 0x05,
    ]));

    #[rustfmt::skip]
    const DISCONNECT_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x4e, 0xb7, 0x99, 0x5f, 0xb5, 0xdd, 0xe5, 0xd0,
        0x85, 0xf4, 0x70, 0x85, 0xcd, 0x95, 0x3d, 0x16,
        0x84, 0x54, 0x11, 0xed, 0xc6, 0x89, 0xe2, 0x7a,
        0xf9, 0xc3, 0xde, 0xa2, 0xfb, 0x12, 0x25, 0xd5,
    ]));

    #[rustfmt::skip]
    const WITNESS_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0xcb, 0x37, 0xff, 0x70, 0x01, 0xc6, 0x2d, 0x94,
        0x42, 0x4f, 0x98, 0x7f, 0x30, 0x23, 0xb3, 0x5e,
        0x30, 0xd2, 0x17, 0x23, 0x96, 0x27, 0x6f, 0x89,
        0xd0, 0x9f, 0x07, 0xaa, 0x67, 0xb6, 0x21, 0x96,
    ]));

    #[rustfmt::skip]
    const FAIL_IV: FirstPassIhr = FirstPassIhr(Midstate([
        0x22, 0x83, 0xc1, 0x81, 0x9e, 0x69, 0x2f, 0x96,
        0x85, 0xfe, 0x95, 0x40, 0x76, 0xc5, 0x16, 0x7c,
        0x03, 0xbd, 0xe7, 0xcc, 0xda, 0xab, 0x00, 0x5e,
        0x55, 0x36, 0x12, 0x2e, 0x18, 0xf7, 0x23, 0x7a,
    ]));
}

impl Ihr {
    /// Do the second pass of the IHR computation. This must be called on the result
    /// of first pass.
    pub fn compute_pass2(first_pass: FirstPassIhr, ty: &FinalArrow) -> Ihr {
        let iv = Ihr(bip340_iv(b"Simplicity\x1fIdentity"));
        iv.update_1(Ihr(first_pass.0))
            .update(ty.source.tmr().into(), ty.target.tmr().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip] // wants to split up the check_iv lines below
    fn ivs() {
        fn check_iv(target: FirstPassIhr, s: &'static str) {
            let name = s
                .trim_start_matches("Simplicity\x1f")
                .trim_start_matches("Commitment\x1f")
                .trim_start_matches("Identity\x1f");
            // Uncomment this if the IVs ever change
            /*
            let target = FirstPassIhr(bip340_iv(s.as_bytes()));
            println!("    #[rustfmt::skip]");
            println!("    const {}_IV: FirstPassIhr = FirstPassIhr(Midstate([", name.to_ascii_uppercase());
            print!("       "); for ch in &target.0[0..8] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[8..16] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[16..24] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[24..32] { print!(" 0x{:02x},", ch); }; println!();
            println!("    ]));");
            println!();
            */
            assert_eq!(target, FirstPassIhr(bip340_iv(s.as_bytes())), "mismatch on IV for {}", name);
        }

        // Note that these are the same as those for CMRs **except** for disconnect and witness.
        check_iv(FirstPassIhr::IDEN_IV, "Simplicity\x1fCommitment\x1fiden");
        check_iv(FirstPassIhr::UNIT_IV, "Simplicity\x1fCommitment\x1funit");
        check_iv(FirstPassIhr::INJL_IV, "Simplicity\x1fCommitment\x1finjl");
        check_iv(FirstPassIhr::INJR_IV, "Simplicity\x1fCommitment\x1finjr");
        check_iv(FirstPassIhr::TAKE_IV, "Simplicity\x1fCommitment\x1ftake");
        check_iv(FirstPassIhr::DROP_IV, "Simplicity\x1fCommitment\x1fdrop");
        check_iv(FirstPassIhr::COMP_IV, "Simplicity\x1fCommitment\x1fcomp");
        check_iv(FirstPassIhr::CASE_IV, "Simplicity\x1fCommitment\x1fcase");
        check_iv(FirstPassIhr::PAIR_IV, "Simplicity\x1fCommitment\x1fpair");
        check_iv(FirstPassIhr::DISCONNECT_IV, "Simplicity\x1fIdentity\x1fdisconnect");
        check_iv(FirstPassIhr::WITNESS_IV, "Simplicity\x1fIdentity\x1fwitness");
        check_iv(FirstPassIhr::FAIL_IV, "Simplicity\x1fCommitment\x1ffail");
    }
}
