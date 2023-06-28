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

use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::types::arrow::FinalArrow;
use crate::{Cmr, Tmr, Value};
use bitcoin_hashes::sha256::Midstate;

use super::{bip340_iv, compact_value, FailEntropy};

/// Identity Merkle root (first pass)
///
/// A Merkle root that commits to a node's combinator, its witness data (if present),
/// and recursively its children. Used as input to the [`Imr`] type which is probably
/// actually what you want.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FirstPassImr(Midstate);

impl_midstate_wrapper!(FirstPassImr);

/// Identity Merkle root
///
/// A Merkle root that commits to a node's combinator, its witness data (if present),
/// its source and target types, and recursively its children.
///
/// Uniquely identifies a program's structure in terms of combinators at redemption time.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Imr(Midstate);

impl_midstate_wrapper!(Imr);

impl From<Cmr> for FirstPassImr {
    fn from(cmr: Cmr) -> Self {
        FirstPassImr::from_byte_array(cmr.to_byte_array())
    }
}

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

impl FirstPassImr {
    /// Produce a CMR for an iden combinator
    pub const fn iden() -> Self {
        Self::IDEN_IV
    }

    /// Produce a CMR for a unit combinator
    pub const fn unit() -> Self {
        Self::UNIT_IV
    }

    /// Produce a CMR for an injl combinator
    pub fn injl(child: FirstPassImr) -> Self {
        Self::INJL_IV.update_1(child)
    }

    /// Produce a CMR for an injr combinator
    pub fn injr(child: FirstPassImr) -> Self {
        Self::INJR_IV.update_1(child)
    }

    /// Produce a CMR for a take combinator
    pub fn take(child: FirstPassImr) -> Self {
        Self::TAKE_IV.update_1(child)
    }

    /// Produce a CMR for a drop combinator
    pub fn drop(child: FirstPassImr) -> Self {
        Self::DROP_IV.update_1(child)
    }

    /// Produce a CMR for a comp combinator
    pub fn comp(left: FirstPassImr, right: FirstPassImr) -> Self {
        Self::COMP_IV.update(left, right)
    }

    /// Produce a CMR for a case combinator
    pub fn case(left: FirstPassImr, right: FirstPassImr) -> Self {
        Self::CASE_IV.update(left, right)
    }

    /// Produce a CMR for a pair combinator
    pub fn pair(left: FirstPassImr, right: FirstPassImr) -> Self {
        Self::PAIR_IV.update(left, right)
    }

    /// Produce a CMR for a disconnect combinator
    pub fn disconnect(left: FirstPassImr, right: FirstPassImr) -> Self {
        Self::DISCONNECT_IV.update(left, right)
    }

    /// Produce a CMR for a witness combinator
    pub fn witness(ty: &FinalArrow, value: &Value) -> Self {
        use bitcoin_hashes::{sha256, HashEngine};

        // 1 Bit-wise hash of `value`
        let value_hash = compact_value(value);
        // 2 Hash of hash of `value` and TMR of `value_type`
        let mut engine = sha256::HashEngine::from_midstate(Self::WITNESS_IV.0, 0);
        engine.input(&value_hash[..]);
        engine.input(ty.target.tmr().as_ref());
        FirstPassImr(engine.midstate())
    }

    /// Produce an IMR for a fail combinator
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
    const IDEN_IV: FirstPassImr = FirstPassImr(Midstate([
        0xdb, 0xfe, 0xfc, 0xfc, 0x77, 0x96, 0xac, 0xfc,
        0x86, 0xb4, 0x35, 0xc1, 0xf8, 0x1e, 0xd8, 0xa1,
        0x65, 0xda, 0xb2, 0x64, 0x9d, 0xc4, 0x8b, 0x0f,
        0x35, 0xf8, 0x32, 0x64, 0x78, 0x68, 0xfb, 0x5e,
    ]));

    #[rustfmt::skip]
    const UNIT_IV: FirstPassImr = FirstPassImr(Midstate([
        0x62, 0x27, 0x4a, 0x89, 0x83, 0x3e, 0xce, 0x8b,
        0xa5, 0xff, 0x57, 0xb2, 0x81, 0x18, 0xc0, 0x06,
        0x3d, 0x3d, 0x4a, 0x85, 0xdd, 0x25, 0xaa, 0xe0,
        0x6f, 0x87, 0x61, 0x76, 0x04, 0x40, 0x27, 0x15,
    ]));

    #[rustfmt::skip]
    const INJL_IV: FirstPassImr = FirstPassImr(Midstate([
        0xd8, 0x07, 0x18, 0xa9, 0xdf, 0x51, 0xd7, 0x64,
        0x36, 0x22, 0x11, 0xab, 0xa4, 0x7d, 0xaf, 0xc9,
        0x3a, 0xf0, 0x70, 0x1b, 0x5d, 0xd5, 0x36, 0xa5,
        0x7b, 0x25, 0xb9, 0x49, 0xdf, 0x49, 0x4f, 0x71,
    ]));

    #[rustfmt::skip]
    const INJR_IV: FirstPassImr = FirstPassImr(Midstate([
        0x3a, 0x3a, 0xe1, 0xa1, 0xc6, 0x2b, 0xcf, 0x45,
        0x6b, 0x5d, 0xa2, 0xc1, 0x1d, 0x60, 0xc8, 0x98,
        0xbc, 0x3b, 0xd0, 0x6f, 0xe9, 0xc6, 0x6d, 0x56,
        0xc7, 0x03, 0x2a, 0x63, 0x20, 0x7a, 0x7b, 0xf6,
    ]));

    #[rustfmt::skip]
    const TAKE_IV: FirstPassImr = FirstPassImr(Midstate([
        0xea, 0xe2, 0xb6, 0x89, 0x82, 0xba, 0x67, 0x8f,
        0x97, 0x6e, 0x6d, 0x96, 0xc7, 0x70, 0x42, 0x07,
        0x86, 0x7b, 0x4f, 0xdb, 0xc7, 0x6d, 0xa8, 0x0d,
        0xb7, 0xb9, 0x59, 0x69, 0x20, 0x3c, 0xbf, 0x17,
    ]));

    #[rustfmt::skip]
    const DROP_IV: FirstPassImr = FirstPassImr(Midstate([
        0xb3, 0x05, 0xbd, 0xaf, 0x53, 0x93, 0x29, 0x89,
        0x12, 0x81, 0xcb, 0x21, 0xa4, 0xa6, 0xe9, 0xcc,
        0x9e, 0x56, 0x85, 0x15, 0x80, 0x79, 0xca, 0xca,
        0xa9, 0x41, 0xfe, 0x65, 0x75, 0x67, 0xdd, 0xb7,
    ]));

    #[rustfmt::skip]
    const COMP_IV: FirstPassImr = FirstPassImr(Midstate([
        0x25, 0x91, 0x90, 0xd1, 0xc4, 0x69, 0x0c, 0x86,
        0x49, 0x74, 0xc5, 0xca, 0x25, 0x75, 0x21, 0x33,
        0x7c, 0x94, 0xed, 0xf5, 0xee, 0xb4, 0x90, 0xae,
        0x56, 0x89, 0x44, 0xbd, 0x85, 0x09, 0xe0, 0x58,
    ]));

    #[rustfmt::skip]
    const CASE_IV: FirstPassImr = FirstPassImr(Midstate([
        0xac, 0xb7, 0x60, 0x6d, 0x3c, 0x55, 0xce, 0xa3,
        0x61, 0xe3, 0xe4, 0xa0, 0x91, 0x42, 0xf5, 0xec,
        0x6b, 0xb9, 0x79, 0xdd, 0x1c, 0xf8, 0xa1, 0x92,
        0x91, 0x24, 0x44, 0x4c, 0xc5, 0xcb, 0xb9, 0x3a,
    ]));

    #[rustfmt::skip]
    const PAIR_IV: FirstPassImr = FirstPassImr(Midstate([
        0x8c, 0x86, 0x65, 0xb4, 0x6b, 0x90, 0x3c, 0x23,
        0x7a, 0x2e, 0x1c, 0x54, 0x77, 0xb6, 0x9a, 0xc3,
        0x28, 0x98, 0x76, 0x61, 0x28, 0x70, 0x92, 0xd3,
        0x6a, 0x3c, 0x99, 0x76, 0x96, 0x85, 0xc6, 0x58,
    ]));

    #[rustfmt::skip]
    const DISCONNECT_IV: FirstPassImr = FirstPassImr(Midstate([
        0xbb, 0xd4, 0x99, 0x6b, 0x6a, 0xb7, 0xcf, 0xc2,
        0xf1, 0x7e, 0x4c, 0xb6, 0xc8, 0xda, 0xdd, 0xe5,
        0xe4, 0x0a, 0xf2, 0x3b, 0xce, 0x2d, 0xa6, 0x3b,
        0x80, 0x91, 0x85, 0x26, 0x5d, 0x3e, 0x53, 0x11,
    ]));

    #[rustfmt::skip]
    const WITNESS_IV: FirstPassImr = FirstPassImr(Midstate([
        0xd7, 0xf8, 0x83, 0x4a, 0x25, 0x71, 0xf9, 0xab,
        0x85, 0xc1, 0x0c, 0xce, 0xb4, 0x56, 0x3b, 0x83,
        0x79, 0x5e, 0x35, 0x8a, 0xbd, 0x14, 0x10, 0xdd,
        0x73, 0x7d, 0xf9, 0x82, 0x91, 0x31, 0x78, 0x25,
    ]));

    #[rustfmt::skip]
    const FAIL_IV: FirstPassImr = FirstPassImr(Midstate([
        0xb1, 0x85, 0xfd, 0x9e, 0x46, 0x60, 0x4c, 0xa0,
        0x02, 0x2d, 0xd1, 0x44, 0x53, 0x99, 0x9f, 0xba,
        0xa4, 0x33, 0x97, 0xae, 0x48, 0x79, 0x6b, 0x0c,
        0x40, 0x23, 0xba, 0xf9, 0x97, 0x1f, 0x15, 0x66,
    ]));
}

impl Imr {
    /// Do the second pass of the IMR computation. This must be called on the result
    /// of first pass.
    pub fn compute_pass2(first_pass: FirstPassImr, ty: &FinalArrow) -> Imr {
        let iv = Imr(bip340_iv(b"Simplicity-Draft\x1fIdentity"));
        iv.update_1(Imr(first_pass.0))
            .update(ty.source.tmr().into(), ty.target.tmr().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip] // wants to split up the check_iv lines below
    fn ivs() {
        fn check_iv(target: FirstPassImr, s: &'static str) {
            let name = &s[s.rfind('\x1f').unwrap()..];
            // Uncomment this if the IVs ever change
            /*
            let target = FirstPassImr(bip340_iv(s.as_bytes()));
            println!("    #[rustfmt::skip]");
            println!("    const {}_IV: FirstPassImr = FirstPassImr(Midstate([", name.to_ascii_uppercase());
            print!("       "); for ch in &target.0[0..8] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[8..16] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[16..24] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[24..32] { print!(" 0x{:02x},", ch); }; println!();
            println!("    ]));");
            println!();
            */
            assert_eq!(target, FirstPassImr(bip340_iv(s.as_bytes())), "mismatch on IV for {}", name);
        }

        // Note that these are the same as those for CMRs **except** for disconnect and witness.
        check_iv(FirstPassImr::IDEN_IV, "Simplicity-Draft\x1fCommitment\x1fiden");
        check_iv(FirstPassImr::UNIT_IV, "Simplicity-Draft\x1fCommitment\x1funit");
        check_iv(FirstPassImr::INJL_IV, "Simplicity-Draft\x1fCommitment\x1finjl");
        check_iv(FirstPassImr::INJR_IV, "Simplicity-Draft\x1fCommitment\x1finjr");
        check_iv(FirstPassImr::TAKE_IV, "Simplicity-Draft\x1fCommitment\x1ftake");
        check_iv(FirstPassImr::DROP_IV, "Simplicity-Draft\x1fCommitment\x1fdrop");
        check_iv(FirstPassImr::COMP_IV, "Simplicity-Draft\x1fCommitment\x1fcomp");
        check_iv(FirstPassImr::CASE_IV, "Simplicity-Draft\x1fCommitment\x1fcase");
        check_iv(FirstPassImr::PAIR_IV, "Simplicity-Draft\x1fCommitment\x1fpair");
        check_iv(FirstPassImr::DISCONNECT_IV, "Simplicity-Draft\x1fIdentity\x1fdisconnect");
        check_iv(FirstPassImr::WITNESS_IV, "Simplicity-Draft\x1fIdentity\x1fwitness");
        check_iv(FirstPassImr::FAIL_IV, "Simplicity-Draft\x1fCommitment\x1ffail");
    }
}
