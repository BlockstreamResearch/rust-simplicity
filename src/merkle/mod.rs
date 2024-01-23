// SPDX-License-Identifier: CC0-1.0

//! # Merkle roots
//!
//! Tools for creating Merkle roots.
//! There exist different Merkle roots for commitment and for redemption.

pub mod amr;
pub mod cmr;
pub mod imr;
pub mod tmr;

use crate::Value;
use hashes::{sha256, Hash, HashEngine};
use std::fmt;

/// 512-bit opaque blob of data used to seed `Fail` nodes
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct FailEntropy([u8; 64]);

impl FailEntropy {
    /// The all-zeroes entropy
    pub const ZERO: Self = FailEntropy([0; 64]);

    /// Construct a [`FailEntropy`] from raw data
    pub fn from_byte_array(data: [u8; 64]) -> Self {
        FailEntropy(data)
    }
}

impl fmt::Display for FailEntropy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&hex::DisplayHex::as_hex(&self.0), f)
    }
}

impl AsRef<[u8]> for FailEntropy {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// Helper function to compute the "compact value", i.e. the sha256 hash
/// of the bits of a given value, which is used in some IMRs and AMRs.
fn compact_value(value: &Value) -> [u8; 32] {
    let (mut bytes, bit_length) = value.to_bytes_len();

    // TODO: Automate hashing once `hashes` supports bit-wise hashing
    // 1.1 Append single '1' bit
    if bit_length % 8 == 0 {
        bytes.push(0x80);
    } else {
        let delimiter_index = bit_length % 8;
        *bytes.last_mut().unwrap() |= 1 << (7 - delimiter_index);
    }

    // 1.2 Append k '0x00' bytes, where k is minimum number >= 0 such that bytes.len() + k + 8 is multiple of 64
    let k = if bytes.len() % 64 > 56 {
        // Not enough space for 64-bit integer
        // Pad with zeroes until next block is 64 bits short of completion
        56 + (64 - (bytes.len() % 64))
    } else {
        // Pad with zeroes until current block is 64 bits short of completion
        56 - (bytes.len() % 64)
    };
    bytes.resize(bytes.len() + k, 0x00);
    debug_assert!(bytes.len() % 64 == 56);

    // 1.3 Append bit_length as 64-bit bit-endian integer
    let bit_length_bytes = (bit_length as u64).to_be_bytes();
    bytes.extend(bit_length_bytes.iter());
    debug_assert!(bytes.len() % 16 == 0);

    // 1.4 Compute hash of `value` normally since bytes.len() is multiple of 64
    let mut consumed = 0;
    let mut engine = sha256::HashEngine::default();
    while consumed < bytes.len() {
        engine.input(&bytes[consumed..(consumed + 16)]);
        consumed += 16;
    }
    debug_assert!(consumed == bytes.len());
    engine.midstate().to_byte_array()
}

fn bip340_iv(tag: &[u8]) -> sha256::Midstate {
    let tag_hash = sha256::Hash::hash(tag);
    let mut engine = sha256::Hash::engine();
    engine.input(tag_hash.as_ref());
    engine.input(tag_hash.as_ref());
    engine.midstate()
}

/// Convenience macro for wrappers of `Midstate`.
///
/// Implements `From` to and from `[u8; 32]`,
/// `MerkleRoot`, `AsRef<[u8]>`, `Debug` and `Display`
#[macro_export]
macro_rules! impl_midstate_wrapper {
    ($wrapper:ident) => {
        impl AsRef<[u8]> for $wrapper {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl std::fmt::Debug for $wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&hex::DisplayHex::as_hex(self.as_ref()), f)
            }
        }

        impl std::fmt::Display for $wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }

        impl std::str::FromStr for $wrapper {
            type Err = hashes::hex::HexToArrayError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let x: [u8; 32] = hashes::hex::FromHex::from_hex(s)?;
                Ok($wrapper(Midstate::from_byte_array(x)))
            }
        }

        impl $wrapper {
            /// Extend the given tagged hash by the given `left` and `right` hashes.
            ///
            /// The hash `self` is taken as initial value,
            /// `left` and `right` hash are combined to create a 512-bit block,
            /// and the compression function is run once
            pub fn update(self, left: Self, right: Self) -> Self {
                use $crate::hashes::{sha256, HashEngine};

                let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
                engine.input(left.as_ref());
                engine.input(right.as_ref());
                $wrapper(engine.midstate())
            }

            /// Extend the given tagged hash by 256 bits of zeroes and the `right` hash.
            ///
            /// The hash `self` is taken as initial value,
            /// 256 bits of zeroes and `right` hash are combined to create a 512-bit block,
            /// and the compression function is run once
            pub fn update_1(self, right: Self) -> Self {
                use $crate::hashes::{sha256, HashEngine};

                let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
                engine.input(&[0; 32]);
                engine.input(&right.as_ref());
                $wrapper(engine.midstate())
            }

            /// Updates the given tagged hash with given `left` cost and `right` hash.
            ///
            /// The cost is serialized as the last 64 bits in the left block
            pub fn update_with_weight(self, left_weight: u64, right: Self) -> Self {
                use $crate::hashes::{sha256, HashEngine};

                let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
                let mut left_blk = [0; 32];
                left_blk[24..].copy_from_slice(&left_weight.to_be_bytes());
                engine.input(&left_blk);
                engine.input(right.as_ref());
                $wrapper(engine.midstate())
            }

            pub fn update_fail_entropy(self, entropy: $crate::FailEntropy) -> Self {
                use $crate::hashes::{sha256, HashEngine};

                let mut engine = sha256::HashEngine::from_midstate(self.0, 0);
                engine.input(entropy.as_ref());
                $wrapper(engine.midstate())
            }

            /// Converts the given tagged hash into a byte array
            pub fn from_byte_array(data: [u8; 32]) -> Self {
                $wrapper(Midstate::from_byte_array(data))
            }

            /// Converts the given tagged hash into a byte array
            pub fn to_byte_array(self) -> [u8; 32] {
                self.0.to_byte_array()
            }
        }

        impl_serde_string!($wrapper);
    };
}
