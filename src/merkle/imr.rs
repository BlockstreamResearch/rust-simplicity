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

//! # Identity Merkle roots
//!
//! Used at time of redemption.
//! In contrast to [`super::cmr`], `witness` data and both `disconnect` branches are included in the hash.
//! The type of `witness` data is included in the hash via [`super::tmr`].

use crate::core::Term;
use crate::impl_midstate_wrapper;
use crate::jet::Application;
use crate::merkle::cmr::Cmr;
use crate::merkle::common::{MerkleRoot, TermMerkleRoot};
use bitcoin_hashes::sha256::Midstate;

/// Identity Merkle root
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Imr(Midstate);

impl_midstate_wrapper!(Imr);

impl TermMerkleRoot for Imr {
    fn get_iv<Witness, App: Application>(term: &Term<Witness, App>) -> Self {
        match term {
            Term::Disconnect(..) => Imr::tag_iv(b"Simplicity-Draft\x1fIdentity\x1fdisconnect"),
            Term::Witness(..) => Imr::tag_iv(b"Simplicity-Draft\x1fIdentity\x1fwitness"),
            _ => Cmr::get_iv(term).into_inner().into(),
        }
    }
}
