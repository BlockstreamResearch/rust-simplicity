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

use crate::core::types::TypeInner;
use crate::impl_midstate_wrapper;
use crate::merkle::common::{MerkleRoot, TypeMerkleRoot};
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

impl TypeMerkleRoot for Tmr {
    fn get_iv(ty: &TypeInner) -> Self {
        match ty {
            TypeInner::Unit => Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1funit"),
            TypeInner::Sum(..) => Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1fsum"),
            TypeInner::Product(..) => Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1fprod"),
        }
    }
}

impl Tmr {
    /// The TMR for the unit type
    pub fn unit() -> Tmr {
        Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1funit")
    }

    /// The TMR for the sum of two types, whose TMRs are given
    pub fn sum(tmr1: Tmr, tmr2: Tmr) -> Tmr {
        Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1fsum").update(tmr1, tmr2)
    }

    /// The TMR for the product of two types, whose TMRs are given
    pub fn product(tmr1: Tmr, tmr2: Tmr) -> Tmr {
        Tmr::tag_iv(b"Simplicity-Draft\x1fType\x1fprod").update(tmr1, tmr2)
    }
}
