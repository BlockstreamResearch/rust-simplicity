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
use std::{fmt, ops};

/// Commitment Merkle Root
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cmr(sha256::Midstate);

impl From<[u8; 32]> for Cmr {
    fn from(data: [u8; 32]) -> Cmr {
        Cmr(sha256::Midstate::from_inner(data))
    }
}

impl From<Cmr> for [u8; 32] {
    fn from(cmr: Cmr) -> [u8; 32] {
        cmr.0.into_inner()
    }
}

impl ops::Deref for Cmr {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Cmr {
    /// Construct a new sha2 midstate by hashing a length-postfixed
    /// string. Will panic if `data` exceeds 56 bytes in length
    pub fn new(data: &[u8]) -> Cmr {
        debug_assert!(data.len() < 56);

        let mut engine = sha256::Hash::engine();
        engine.input(data);
        let h = sha256::Hash::from_engine(engine);
        // concat with itself
        let block = [h.into_inner(), h.into_inner()].concat();

        let mut engine = sha256::Hash::engine();
        engine.input(&block);
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

impl fmt::Debug for Cmr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.into_inner().iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
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

    /// Tagged cmr hash used by `iden`
    pub fn iden_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fiden")
    }

    /// Tagged cmr hash used by `comp`
    pub fn comp_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fcomp")
    }

    /// Tagged cmr hash used by `unit`
    pub fn unit_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1funit")
    }

    /// Tagged cmr hash used by `injl`
    pub fn injl_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1finjl")
    }

    /// Tagged cmr hash used by `injr`
    pub fn injr_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1finjr")
    }

    /// Tagged cmr hash used by `case`
    pub fn case_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fcase")
    }

    /// Tagged cmr hash used by `pair`
    pub fn pair_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fpair")
    }

    /// Tagged cmr hash used by `take`
    pub fn take_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1ftake")
    }

    /// Tagged cmr hash used by `drop`
    pub fn drop_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fdrop")
    }

    /// Tagged cmr hash used by `witness`
    pub fn witness_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fwitness")
    }

    /// Tagged cmr hash used by `disconnect`
    pub fn disconnect_cmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fCommitment\x1fdisconnect")
    }

    /// Tagged wmr hash used by `iden`
    pub fn iden_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fiden")
    }

    /// Tagged wmr hash used by `comp`
    pub fn comp_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fcomp")
    }

    /// Tagged wmr hash used by `unit`
    pub fn unit_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1funit")
    }

    /// Tagged wmr hash used by `injl`
    pub fn injl_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1finjl")
    }

    /// Tagged wmr hash used by `injr`
    pub fn injr_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1finjr")
    }

    /// Tagged wmr hash used by `case`
    pub fn case_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fcase")
    }

    /// Tagged wmr hash used by `assertl`
    pub fn assertl_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fassertl")
    }

    /// Tagged wmr hash used by `assertr`
    pub fn assertr_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fassertr")
    }

    /// Tagged wmr hash used by `pair`
    pub fn pair_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fpair")
    }

    /// Tagged wmr hash used by `take`
    pub fn take_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1ftake")
    }

    /// Tagged wmr hash used by `drop`
    pub fn drop_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fdrop")
    }

    /// Tagged wmr hash used by `witness`
    pub fn witness_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fwitness")
    }

    /// Tagged wmr hash used by `disconnect`
    pub fn disconnect_wmr() -> Cmr {
        Cmr::new(b"Simplicity\x1fWitness\x1fdisconnect")
    }
}
