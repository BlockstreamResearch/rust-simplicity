// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
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

//! # Commitment Merkle Roots
//!
//! Tagged SHA256 hashes used for computing CMRs
//!

use bitcoin_hashes::{sha256, Hash, HashEngine};
use std::fmt;

/// Commitment Merkle Root
#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Cmr(sha256::Midstate);

impl From<[u8; 32]> for Cmr {
    fn from(data: [u8; 32]) -> Cmr {
        Cmr(sha256::Midstate::from_inner(data))
    }
}

impl Into<[u8; 32]> for Cmr {
    fn into(self) -> [u8; 32] {
        self.0.into_inner()
    }
}

impl Cmr {
    /// Construct a new sha2 midstate by hashing a length-postfixed
    /// string. Will panic if `data` exceeds 56 bytes in length
    pub fn new(data: &[u8]) -> Cmr {
        debug_assert!(data.len() < 56);

        let mut engine = sha256::Hash::engine();
        engine.input(data);
        engine.input(&[0x80]);
        for _ in 0..64 - 2 - data.len() - 1 {
            engine.input(&[0x00]);
        }
        engine.input(&[(data.len() as u8) >> 5]);
        engine.input(&[(data.len() as u8) << 3]);
        Cmr(engine.midstate())
    }

    /// Using a CMR as a sha2 midstate, hash 64 more bytes with one run of the
    /// sha2 compression function
    pub fn update(self, left: Cmr, right: Cmr) -> Cmr {
        let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
        engine.input(&left.0[..]);
        engine.input(&right.0[..]);
        Cmr(engine.midstate())
    }

    /// Using a CMR as a sha2 midstate, hash 32 0's followed by 32 more bytes with
    /// one run of the sha2 compression function
    pub fn update_1(self, x: Cmr) -> Cmr {
        let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
        engine.input(&[0; 32]);
        engine.input(&x.0[..]);
        Cmr(engine.midstate())
    }

    /// Helper function to convert a `Cmr` to a byte array
    pub fn into_inner(self) -> [u8; 32] {
        self.into()
    }
}

impl fmt::Display for Cmr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.into_inner().iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// CMR tags for the nodes in pure Simplicity
pub mod tag {
    use super::Cmr;

    /// Tagged hash used by `iden`
    pub fn iden() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fiden")
    }

    /// Tagged hash used by `comp`
    pub fn comp() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fcomp")
    }

    /// Tagged hash used by `unit`
    pub fn unit() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1funit")
    }

    /// Tagged hash used by `injl`
    pub fn injl() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1finjl")
    }

    /// Tagged hash used by `injr`
    pub fn injr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1finjr")
    }

    /// Tagged hash used by `case`
    pub fn case() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fcase")
    }

    /// Tagged hash used by `pair`
    pub fn pair() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fpair")
    }

    /// Tagged hash used by `take`
    pub fn take() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1ftake")
    }

    /// Tagged hash used by `drop`
    pub fn drop() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fdrop")
    }

    /// Tagged hash used by `witness`
    pub fn witness() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fwitness")
    }
}

