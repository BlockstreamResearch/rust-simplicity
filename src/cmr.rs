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

/// Annotated Merkle Root
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Amr(sha256::Midstate);

/// Type Merkle Root
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tmr(sha256::Midstate);

macro_rules! impl_midstate_wrapper {
    ($cmr:ident) => {
        impl From<[u8; 32]> for $cmr {
            fn from(data: [u8; 32]) -> $cmr {
                $cmr(sha256::Midstate::from_inner(data))
            }
        }

        impl From<$cmr> for [u8; 32] {
            fn from(cmr: $cmr) -> [u8; 32] {
                cmr.0.into_inner()
            }
        }

        impl ops::Deref for $cmr {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $cmr {
            /// Construct a new sha2 midstate by hashing a length-postfixed
            /// string. Will panic if `data` exceeds 56 bytes in length
            pub fn new(data: &[u8]) -> $cmr {
                debug_assert!(data.len() < 56);

                let mut engine = sha256::Hash::engine();
                engine.input(data);
                let h = sha256::Hash::from_engine(engine);
                // concat with itself
                let block = [h.into_inner(), h.into_inner()].concat();

                let mut engine = sha256::Hash::engine();
                engine.input(&block);
                $cmr(engine.midstate())
            }

            /// Using a CMR as a sha2 midstate, hash 64 more bytes with one run of the
            /// sha2 compression function
            pub fn update(self, left: $cmr, right: $cmr) -> $cmr {
                let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
                engine.input(&left.0[..]);
                engine.input(&right.0[..]);
                $cmr(engine.midstate())
            }

            /// Using a CMR as a sha2 midstate, hash 32 0's followed by 32 more bytes with
            /// one run of the sha2 compression function
            pub fn update_1(self, x: $cmr) -> $cmr {
                let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
                engine.input(&[0; 32]);
                engine.input(&x.0[..]);
                $cmr(engine.midstate())
            }

            /// Helper function to convert a `$cmr` to a byte array
            pub fn into_inner(self) -> [u8; 32] {
                self.into()
            }
        }

        impl fmt::Debug for $cmr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for byte in self.0.into_inner().iter() {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }
        }

        impl fmt::Display for $cmr {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for byte in self.0.into_inner().iter() {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }
        }
    };
}

impl_midstate_wrapper!(Cmr);
impl_midstate_wrapper!(Amr);
impl_midstate_wrapper!(Tmr);

impl From<Tmr> for Amr {
    /// Re-interpret a TMR as an AMR
    fn from(tmr: Tmr) -> Amr {
        Amr::from(tmr.into_inner())
    }
}

/// CMR tags for the nodes in pure Simplicity
pub mod tag {
    use super::{Amr, Cmr, Tmr};

    /// Tagged cmr hash used by `iden`
    pub fn iden_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1fiden")
    }

    /// Tagged cmr hash used by `comp`
    pub fn comp_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1fcomp")
    }

    /// Tagged cmr hash used by `unit`
    pub fn unit_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1funit")
    }

    /// Tagged cmr hash used by `injl`
    pub fn injl_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1finjl")
    }

    /// Tagged cmr hash used by `injr`
    pub fn injr_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1finjr")
    }

    /// Tagged cmr hash used by `case`
    pub fn case_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1fcase")
    }

    /// Tagged cmr hash used by `pair`
    pub fn pair_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1fpair")
    }

    /// Tagged cmr hash used by `take`
    pub fn take_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1ftake")
    }

    /// Tagged cmr hash used by `drop`
    pub fn drop_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1fdrop")
    }

    /// Tagged cmr hash used by `witness`
    pub fn witness_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1fwitness")
    }

    /// Tagged cmr hash used by `disconnect`
    pub fn disconnect_cmr() -> Cmr {
        Cmr::new(b"Simplicity-Draft\x1fCommitment\x1fdisconnect")
    }

    /// Tagged amr hash used by `iden`
    pub fn iden_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fiden")
    }

    /// Tagged amr hash used by `comp`
    pub fn comp_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fcomp")
    }

    /// Tagged amr hash used by `unit`
    pub fn unit_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1funit")
    }

    /// Tagged amr hash used by `injl`
    pub fn injl_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1finjl")
    }

    /// Tagged amr hash used by `injr`
    pub fn injr_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1finjr")
    }

    /// Tagged amr hash used by `case`
    pub fn case_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fcase")
    }

    /// Tagged amr hash used by `assertl`
    pub fn assertl_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fassertl")
    }

    /// Tagged amr hash used by `assertr`
    pub fn assertr_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fassertr")
    }

    /// Tagged amr hash used by `pair`
    pub fn pair_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fpair")
    }

    /// Tagged amr hash used by `take`
    pub fn take_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1ftake")
    }

    /// Tagged amr hash used by `drop`
    pub fn drop_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fdrop")
    }

    /// Tagged amr hash used by `witness`
    pub fn witness_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fwitness")
    }

    /// Tagged amr hash used by `disconnect`
    pub fn disconnect_amr() -> Amr {
        Amr::new(b"Simplicity-Draft\x1fAnnotated\x1fdisconnect")
    }

    /// Tagged term type mr IV for unit type
    pub fn unit_type_tmr() -> Tmr {
        Tmr::new(b"Simplicity-Draft\x1fType\x1funit")
    }

    /// Tagged term type rm IV for unit type
    pub fn sum_type_tmr() -> Tmr {
        Tmr::new(b"Simplicity-Draft\x1fType\x1fsum")
    }

    /// Tagged term type rm IV for prod type
    pub fn prod_type_tmr() -> Tmr {
        Tmr::new(b"Simplicity-Draft\x1fType\x1fprod")
    }
}
