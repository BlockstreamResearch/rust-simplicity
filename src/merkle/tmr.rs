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
use crate::merkle::common::MerkleRoot;
use bitcoin_hashes::sha256::Midstate;

/// Type Merkle root
///
/// A Merkle root that commits to a type's primitive (unit, sum, product)
/// and recursively its sub-types.
///
/// Uniquely identifies a type.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tmr(pub(crate) Midstate);

impl_midstate_wrapper!(Tmr);

impl Tmr {
    /// The IV for a TMR of a unit type
    #[rustfmt::skip]
    pub const UNIT_IV: Tmr = Tmr(Midstate([
        0x12, 0xb4, 0xc4, 0xa9, 0xa4, 0xb0, 0xed, 0xf6,
        0x5a, 0x44, 0xf3, 0x0e, 0xa7, 0x62, 0x57, 0x8f,
        0xdd, 0x59, 0xf1, 0x05, 0xf0, 0xe4, 0xd8, 0xf3,
        0x88, 0xcb, 0x9b, 0x6b, 0xd2, 0xc1, 0x3a, 0xdf,
    ]));

    /// The IV for a TMR of a sum type
    #[rustfmt::skip]
    pub const SUM_IV: Tmr = Tmr(Midstate([
        0x05, 0xcc, 0x9d, 0xdd, 0x0e, 0x50, 0xb0, 0xec,
        0x99, 0xfd, 0x5f, 0xad, 0xdc, 0x4d, 0x95, 0x06,
        0xcd, 0x3e, 0x7b, 0xb8, 0xed, 0xeb, 0x40, 0xca,
        0x98, 0x33, 0x86, 0x6e, 0x3a, 0x0a, 0xbc, 0x33,
    ]));

    /// The IV for a TMR of a product type
    #[rustfmt::skip]
    pub const PRODUCT_IV: Tmr = Tmr(Midstate([
        0xc1, 0x71, 0x96, 0x87, 0x4b, 0x51, 0x21, 0xfd,
        0x5d, 0xbe, 0x2f, 0xef, 0x5b, 0xa0, 0xd2, 0xed,
        0xce, 0x23, 0x92, 0xe3, 0x55, 0x15, 0xa2, 0xf2,
        0x06, 0xb2, 0x2b, 0xbe, 0x08, 0x8b, 0xb1, 0xaf,
    ]));

    /// The TMR for the unit type
    pub const fn unit() -> Tmr {
        Self::UNIT_IV
    }

    /// The TMR for the sum of two types, whose TMRs are given
    pub fn sum(tmr1: Tmr, tmr2: Tmr) -> Tmr {
        Self::SUM_IV.update(tmr1, tmr2)
    }

    /// The TMR for the product of two types, whose TMRs are given
    pub fn product(tmr1: Tmr, tmr2: Tmr) -> Tmr {
        Self::PRODUCT_IV.update(tmr1, tmr2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_ivs() {
        assert_eq!(
            Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1funit"),
            Tmr::UNIT_IV,
        );
        assert_eq!(Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1fsum"), Tmr::SUM_IV,);
        assert_eq!(
            Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1fprod"),
            Tmr::PRODUCT_IV,
        );
    }
}
