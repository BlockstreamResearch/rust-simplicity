// SPDX-License-Identifier: CC0-1.0

use std::sync::Arc;

use crate::impl_midstate_wrapper;
use crate::jet::Jet;
use crate::node::{
    CoreConstructible, DisconnectConstructible, JetConstructible, WitnessConstructible,
};
use crate::types::{self, Error};
use crate::{FailEntropy, Tmr, Value};
use hashes::sha256::Midstate;

use super::bip340_iv;

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

impl From<Tmr> for Cmr {
    fn from(tmr: Tmr) -> Self {
        Cmr::from_byte_array(tmr.to_byte_array())
    }
}

impl Cmr {
    /// Produce a CMR for an iden combinator
    pub const fn iden() -> Self {
        Self::IDEN_IV
    }

    /// Produce a CMR for a unit combinator
    pub const fn unit() -> Self {
        Self::UNIT_IV
    }

    /// Produce a CMR for an injl combinator
    pub fn injl(child: Cmr) -> Self {
        Self::INJL_IV.update_1(child)
    }

    /// Produce a CMR for an injr combinator
    pub fn injr(child: Cmr) -> Self {
        Self::INJR_IV.update_1(child)
    }

    /// Produce a CMR for a take combinator
    pub fn take(child: Cmr) -> Self {
        Self::TAKE_IV.update_1(child)
    }

    /// Produce a CMR for a drop combinator
    pub fn drop(child: Cmr) -> Self {
        Self::DROP_IV.update_1(child)
    }

    /// Produce a CMR for a comp combinator
    pub fn comp(left: Cmr, right: Cmr) -> Self {
        Self::COMP_IV.update(left, right)
    }

    /// Produce a CMR for a case combinator
    pub fn case(left: Cmr, right: Cmr) -> Self {
        Self::CASE_IV.update(left, right)
    }

    /// Produce a CMR for a pair combinator
    pub fn pair(left: Cmr, right: Cmr) -> Self {
        Self::PAIR_IV.update(left, right)
    }

    /// Produce a CMR for a disconnect combinator
    pub fn disconnect(left: Cmr) -> Self {
        Self::DISCONNECT_IV.update_1(left)
    }

    /// Produce a CMR for a witness combinator
    pub fn witness() -> Self {
        Self::WITNESS_IV
    }

    /// Produce a CMR for a fail combinator
    pub fn fail(entropy: FailEntropy) -> Self {
        Self::FAIL_IV.update_fail_entropy(entropy)
    }

    /// Produce a CMR for a jet
    pub fn jet<J: Jet>(jet: J) -> Self {
        jet.cmr()
    }

    /// Compute the CMR of a constant word jet
    ///
    /// This is equal to the IMR of the equivalent scribe, converted to a CMR in
    /// the usual way for jets.
    pub fn const_word(v: &Value) -> Self {
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

        let imr_iv = Self::CONST_WORD_IV;
        let imr_pass1 = imr_iv.update_1(cmr_stack[0]);
        // 2. Add TMRs to get the pass-two IMR
        let imr_pass2 = imr_pass1.update(Tmr::unit().into(), Tmr::POWERS_OF_TWO[w - 1].into());
        // 3. Convert to a jet CMR
        Cmr(bip340_iv(b"Simplicity-Draft\x1fJet")).update_with_weight(v.len() as u64, imr_pass2)
    }

    #[rustfmt::skip]
    const UNIT_IV: Cmr = Cmr(Midstate([
        0x62, 0x27, 0x4a, 0x89, 0x83, 0x3e, 0xce, 0x8b,
        0xa5, 0xff, 0x57, 0xb2, 0x81, 0x18, 0xc0, 0x06,
        0x3d, 0x3d, 0x4a, 0x85, 0xdd, 0x25, 0xaa, 0xe0,
        0x6f, 0x87, 0x61, 0x76, 0x04, 0x40, 0x27, 0x15,
    ]));

    #[rustfmt::skip]
    const IDEN_IV: Cmr = Cmr(Midstate([
        0xdb, 0xfe, 0xfc, 0xfc, 0x77, 0x96, 0xac, 0xfc,
        0x86, 0xb4, 0x35, 0xc1, 0xf8, 0x1e, 0xd8, 0xa1,
        0x65, 0xda, 0xb2, 0x64, 0x9d, 0xc4, 0x8b, 0x0f,
        0x35, 0xf8, 0x32, 0x64, 0x78, 0x68, 0xfb, 0x5e,
    ]));

    #[rustfmt::skip]
    const INJL_IV: Cmr = Cmr(Midstate([
        0xd8, 0x07, 0x18, 0xa9, 0xdf, 0x51, 0xd7, 0x64,
        0x36, 0x22, 0x11, 0xab, 0xa4, 0x7d, 0xaf, 0xc9,
        0x3a, 0xf0, 0x70, 0x1b, 0x5d, 0xd5, 0x36, 0xa5,
        0x7b, 0x25, 0xb9, 0x49, 0xdf, 0x49, 0x4f, 0x71,
    ]));

    #[rustfmt::skip]
    const INJR_IV: Cmr = Cmr(Midstate([
        0x3a, 0x3a, 0xe1, 0xa1, 0xc6, 0x2b, 0xcf, 0x45,
        0x6b, 0x5d, 0xa2, 0xc1, 0x1d, 0x60, 0xc8, 0x98,
        0xbc, 0x3b, 0xd0, 0x6f, 0xe9, 0xc6, 0x6d, 0x56,
        0xc7, 0x03, 0x2a, 0x63, 0x20, 0x7a, 0x7b, 0xf6,
    ]));

    #[rustfmt::skip]
    const TAKE_IV: Cmr = Cmr(Midstate([
        0xea, 0xe2, 0xb6, 0x89, 0x82, 0xba, 0x67, 0x8f,
        0x97, 0x6e, 0x6d, 0x96, 0xc7, 0x70, 0x42, 0x07,
        0x86, 0x7b, 0x4f, 0xdb, 0xc7, 0x6d, 0xa8, 0x0d,
        0xb7, 0xb9, 0x59, 0x69, 0x20, 0x3c, 0xbf, 0x17,
    ]));

    #[rustfmt::skip]
    const DROP_IV: Cmr = Cmr(Midstate([
        0xb3, 0x05, 0xbd, 0xaf, 0x53, 0x93, 0x29, 0x89,
        0x12, 0x81, 0xcb, 0x21, 0xa4, 0xa6, 0xe9, 0xcc,
        0x9e, 0x56, 0x85, 0x15, 0x80, 0x79, 0xca, 0xca,
        0xa9, 0x41, 0xfe, 0x65, 0x75, 0x67, 0xdd, 0xb7,
    ]));

    #[rustfmt::skip]
    const COMP_IV: Cmr = Cmr(Midstate([
        0x25, 0x91, 0x90, 0xd1, 0xc4, 0x69, 0x0c, 0x86,
        0x49, 0x74, 0xc5, 0xca, 0x25, 0x75, 0x21, 0x33,
        0x7c, 0x94, 0xed, 0xf5, 0xee, 0xb4, 0x90, 0xae,
        0x56, 0x89, 0x44, 0xbd, 0x85, 0x09, 0xe0, 0x58,
    ]));

    #[rustfmt::skip]
    const CASE_IV: Cmr = Cmr(Midstate([
        0xac, 0xb7, 0x60, 0x6d, 0x3c, 0x55, 0xce, 0xa3,
        0x61, 0xe3, 0xe4, 0xa0, 0x91, 0x42, 0xf5, 0xec,
        0x6b, 0xb9, 0x79, 0xdd, 0x1c, 0xf8, 0xa1, 0x92,
        0x91, 0x24, 0x44, 0x4c, 0xc5, 0xcb, 0xb9, 0x3a,
    ]));

    #[rustfmt::skip]
    const PAIR_IV: Cmr = Cmr(Midstate([
        0x8c, 0x86, 0x65, 0xb4, 0x6b, 0x90, 0x3c, 0x23,
        0x7a, 0x2e, 0x1c, 0x54, 0x77, 0xb6, 0x9a, 0xc3,
        0x28, 0x98, 0x76, 0x61, 0x28, 0x70, 0x92, 0xd3,
        0x6a, 0x3c, 0x99, 0x76, 0x96, 0x85, 0xc6, 0x58,
    ]));

    #[rustfmt::skip]
    const DISCONNECT_IV: Cmr = Cmr(Midstate([
        0x36, 0x13, 0x48, 0x81, 0x7e, 0x0b, 0x72, 0xd2,
        0xfa, 0x28, 0x38, 0x22, 0x5d, 0x0e, 0xca, 0x86,
        0xe8, 0xcc, 0x75, 0xda, 0x29, 0xef, 0xd1, 0x4a,
        0x0b, 0xcb, 0x0d, 0x2e, 0x1b, 0x3b, 0x0d, 0x1e,
    ]));

    #[rustfmt::skip]
    const WITNESS_IV: Cmr = Cmr(Midstate([
        0xbf, 0x12, 0x68, 0x1a, 0x76, 0xfc, 0x7c, 0x00,
        0xc6, 0x3e, 0x58, 0x3c, 0x25, 0xcc, 0x97, 0x23,
        0x73, 0x37, 0xd6, 0xac, 0xa3, 0x0d, 0x3f, 0x4a,
        0x66, 0x40, 0x75, 0x44, 0x53, 0x85, 0xc6, 0x48,
    ]));

    #[rustfmt::skip]
    const FAIL_IV: Cmr = Cmr(Midstate([
        0xb1, 0x85, 0xfd, 0x9e, 0x46, 0x60, 0x4c, 0xa0,
        0x02, 0x2d, 0xd1, 0x44, 0x53, 0x99, 0x9f, 0xba,
        0xa4, 0x33, 0x97, 0xae, 0x48, 0x79, 0x6b, 0x0c,
        0x40, 0x23, 0xba, 0xf9, 0x97, 0x1f, 0x15, 0x66,
    ]));

    #[rustfmt::skip]
    const CONST_WORD_IV: Cmr = Cmr(Midstate([
        0x84, 0xd0, 0xd2, 0xc2, 0x28, 0x90, 0x93, 0xd6,
        0x4b, 0x06, 0x85, 0x85, 0x04, 0xcf, 0x5f, 0xb3,
        0x36, 0xdc, 0x40, 0x43, 0x71, 0xb2, 0x88, 0x64,
        0x81, 0xee, 0x67, 0x83, 0xa4, 0xfb, 0x00, 0x10,
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
}

/// Wrapper around a CMR which allows it to be constructed with the
/// `*Constructible*` traits, allowing CMRs to be computed using the
/// same generic construction code that nodes are.
pub struct ConstructibleCmr {
    pub cmr: Cmr,
    pub inference_context: types::Context,
}

impl CoreConstructible for ConstructibleCmr {
    fn iden(inference_context: &types::Context) -> Self {
        ConstructibleCmr {
            cmr: Cmr::iden(),
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn unit(inference_context: &types::Context) -> Self {
        ConstructibleCmr {
            cmr: Cmr::unit(),
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn injl(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::injl(child.cmr),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn injr(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::injl(child.cmr),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn take(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::take(child.cmr),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn drop_(child: &Self) -> Self {
        ConstructibleCmr {
            cmr: Cmr::drop(child.cmr),
            inference_context: child.inference_context.shallow_clone(),
        }
    }

    fn comp(left: &Self, right: &Self) -> Result<Self, Error> {
        left.inference_context.check_eq(&right.inference_context)?;
        Ok(ConstructibleCmr {
            cmr: Cmr::comp(left.cmr, right.cmr),
            inference_context: left.inference_context.shallow_clone(),
        })
    }

    fn case(left: &Self, right: &Self) -> Result<Self, Error> {
        left.inference_context.check_eq(&right.inference_context)?;
        Ok(ConstructibleCmr {
            cmr: Cmr::case(left.cmr, right.cmr),
            inference_context: left.inference_context.shallow_clone(),
        })
    }

    fn assertl(left: &Self, right: Cmr) -> Result<Self, Error> {
        Ok(ConstructibleCmr {
            cmr: Cmr::case(left.cmr, right),
            inference_context: left.inference_context.shallow_clone(),
        })
    }

    fn assertr(left: Cmr, right: &Self) -> Result<Self, Error> {
        Ok(ConstructibleCmr {
            cmr: Cmr::case(left, right.cmr),
            inference_context: right.inference_context.shallow_clone(),
        })
    }

    fn pair(left: &Self, right: &Self) -> Result<Self, Error> {
        left.inference_context.check_eq(&right.inference_context)?;
        Ok(ConstructibleCmr {
            cmr: Cmr::pair(left.cmr, right.cmr),
            inference_context: left.inference_context.shallow_clone(),
        })
    }

    fn fail(inference_context: &types::Context, entropy: FailEntropy) -> Self {
        ConstructibleCmr {
            cmr: Cmr::fail(entropy),
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn const_word(inference_context: &types::Context, word: Arc<Value>) -> Self {
        ConstructibleCmr {
            cmr: Cmr::const_word(&word),
            inference_context: inference_context.shallow_clone(),
        }
    }

    fn inference_context(&self) -> &types::Context {
        &self.inference_context
    }
}

impl<X> DisconnectConstructible<X> for ConstructibleCmr {
    // Specifically with disconnect we don't check for consistency between the
    // type inference context of the disconnected node, if any, and that of
    // the left node. The idea is, from the point of view of (Constructible)Cmr,
    // the right child of disconnect doesn't even exist.
    fn disconnect(left: &Self, _right: &X) -> Result<Self, Error> {
        Ok(ConstructibleCmr {
            cmr: Cmr::disconnect(left.cmr),
            inference_context: left.inference_context.shallow_clone(),
        })
    }
}

impl<W> WitnessConstructible<W> for ConstructibleCmr {
    fn witness(inference_context: &types::Context, _witness: W) -> Self {
        ConstructibleCmr {
            cmr: Cmr::witness(),
            inference_context: inference_context.shallow_clone(),
        }
    }
}

impl<J: Jet> JetConstructible<J> for ConstructibleCmr {
    fn jet(inference_context: &types::Context, jet: J) -> Self {
        ConstructibleCmr {
            cmr: jet.cmr(),
            inference_context: inference_context.shallow_clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::jet::Core;
    use crate::node::{ConstructNode, CoreConstructible};

    use std::str::FromStr;
    use std::sync::Arc;

    #[test]
    fn cmr_display_unit() {
        let ctx = types::Context::new();
        let c = Arc::<ConstructNode<Core>>::unit(&ctx);

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
        let bit0 = Value::sum_l(Value::unit());
        #[rustfmt::skip]
        assert_eq!(
            Cmr::const_word(&bit0),
            Cmr::from_str("d96f51cb162ddd0e317d711aec53a8bdd15ff71367518ea21787ca978559021b").unwrap(),
        );
    }

    #[test]
    fn bit_cmr() {
        let ctx = types::Context::new();
        let unit = Arc::<ConstructNode<Core>>::unit(&ctx);
        let bit0 = Arc::<ConstructNode<Core>>::injl(&unit);
        assert_eq!(bit0.cmr(), Cmr::BITS[0]);

        let bit1 = Arc::<ConstructNode<_>>::injr(&unit);
        assert_eq!(bit1.cmr(), Cmr::BITS[1]);
    }

    #[test]
    fn ivs() {
        fn check_iv(target: Cmr, s: &'static str) {
            let name = &s[s.rfind('\x1f').unwrap()..];
            // Uncomment this if the IVs ever change
            /*
            let target = Cmr(bip340_iv(s.as_bytes()));
            println!("    #[rustfmt::skip]");
            println!("    const {}_IV: Cmr = Cmr(Midstate([", name.to_ascii_uppercase());
            print!("       "); for ch in &target.0[0..8] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[8..16] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[16..24] { print!(" 0x{:02x},", ch); }; println!();
            print!("       "); for ch in &target.0[24..32] { print!(" 0x{:02x},", ch); }; println!();
            println!("    ]));");
            println!();
            */
            assert_eq!(
                target,
                Cmr(bip340_iv(s.as_bytes())),
                "mismatch on IV for {}",
                name
            );
        }

        check_iv(Cmr::UNIT_IV, "Simplicity-Draft\x1fCommitment\x1funit");
        check_iv(Cmr::IDEN_IV, "Simplicity-Draft\x1fCommitment\x1fiden");
        check_iv(Cmr::INJL_IV, "Simplicity-Draft\x1fCommitment\x1finjl");
        check_iv(Cmr::INJR_IV, "Simplicity-Draft\x1fCommitment\x1finjr");
        check_iv(Cmr::TAKE_IV, "Simplicity-Draft\x1fCommitment\x1ftake");
        check_iv(Cmr::DROP_IV, "Simplicity-Draft\x1fCommitment\x1fdrop");
        check_iv(Cmr::COMP_IV, "Simplicity-Draft\x1fCommitment\x1fcomp");
        check_iv(Cmr::CASE_IV, "Simplicity-Draft\x1fCommitment\x1fcase");
        check_iv(Cmr::PAIR_IV, "Simplicity-Draft\x1fCommitment\x1fpair");
        check_iv(
            Cmr::DISCONNECT_IV,
            "Simplicity-Draft\x1fCommitment\x1fdisconnect",
        );
        check_iv(Cmr::WITNESS_IV, "Simplicity-Draft\x1fCommitment\x1fwitness");
        check_iv(Cmr::FAIL_IV, "Simplicity-Draft\x1fCommitment\x1ffail");
        check_iv(Cmr::CONST_WORD_IV, "Simplicity-Draft\x1fIdentity");
    }
}
