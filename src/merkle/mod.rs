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

//! # Merkle roots
//!
//! Tools for creating Merkle roots.
//! There exist different Merkle roots for commitment and for redemption.

pub mod amr;
pub mod cmr;
pub mod imr;
pub mod tmr;

use crate::core::commit::CommitNodeInner;
use crate::jet::Jet;
use crate::types;
use crate::util::u64_to_array_be;
use crate::Value;
use bitcoin_hashes::sha256::Midstate;
use bitcoin_hashes::{sha256, Hash, HashEngine};
use std::{fmt, str};

/// Helper function to compute the "compact value", i.e. the sha256 hash
/// of the bits of a given value, which is used in some IMRs and AMRs.
fn compact_value(value: &Value) -> [u8; 32] {
    let (mut bytes, bit_length) = value.to_bytes_len();

    // TODO: Automate hashing once `bitcoin_hashes` supports bit-wise hashing
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
    let bit_length_bytes = u64_to_array_be(bit_length as u64);
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
    engine.midstate().into_inner()
}

/// [Tagged SHA256 hash](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki).
///
/// A tag is hashed and used as initial value (256-bit midstate).
/// Subsequent data extends this hash (updated 256-bit midstate)
/// and the resulting hash is returned
pub trait MerkleRoot: From<[u8; 32]> + Into<[u8; 32]> + fmt::Display + str::FromStr {
    /// Create a tagged hash from the given tag `data`.
    ///
    /// The `data` is hashed,
    /// the resulting 256-bit value is duplicated to create a 512-bit block,
    /// and the compression function is run once
    fn tag_iv(data: &[u8]) -> Self {
        let tag_hash = sha256::Hash::hash(data);
        let block = [tag_hash.into_inner(), tag_hash.into_inner()].concat();
        let mut engine = sha256::Hash::engine();
        engine.input(&block);

        Self::from(engine.midstate().into_inner())
    }

    /// Extend the given tagged hash by the given `left` and `right` hashes.
    ///
    /// The hash `self` is taken as initial value,
    /// `left` and `right` hash are combined to create a 512-bit block,
    /// and the compression function is run once
    fn update(self, left: Self, right: Self) -> Self {
        let mut engine = sha256::HashEngine::from_midstate(Midstate::from_inner(self.into()), 0);
        engine.input(&left.into()[..]);
        engine.input(&right.into()[..]);
        Self::from(engine.midstate().into_inner())
    }

    /// Extend the given tagged hash by 256 bits of zeroes and the `right` hash.
    ///
    /// The hash `self` is taken as initial value,
    /// 256 bits of zeroes and `right` hash are combined to create a 512-bit block,
    /// and the compression function is run once
    fn update_1(self, right: Self) -> Self {
        let mut engine = sha256::HashEngine::from_midstate(Midstate::from_inner(self.into()), 0);
        engine.input(&[0; 32]);
        engine.input(&right.into()[..]);
        Self::from(engine.midstate().into_inner())
    }

    /// Extend the given tagged hash by the hash of the given `value` and the TMR of its `value_type`.
    ///
    /// The hash `self` is taken as initial value,
    /// the hash of `value` and the TMR of `value_type` are combined to create a 512-bit block,
    /// and the compression is run once
    fn update_value(self, value: &Value, value_type: &types::Final) -> Self {
        // 1 Bit-wise hash of `value`
        let value_hash = compact_value(value);
        // 2 Hash of hash of `value` and TMR of `value_type`
        let mut engine = sha256::HashEngine::from_midstate(Midstate::from_inner(self.into()), 0);
        engine.input(&value_hash[..]);
        engine.input(value_type.tmr().as_ref());
        Self::from(engine.midstate().into_inner())
    }

    /// Converts the given tagged hash into a byte array
    fn into_inner(self) -> [u8; 32] {
        Into::<[u8; 32]>::into(self)
    }
}

/// Tagged SHA256 hash used for [`crate::core::CommitNode`]
pub trait CommitMerkleRoot: MerkleRoot {
    /// Return the initial value for the given `node`.
    ///
    /// Each [`CommitNodeInner`] corresponds to some tag that is hashed
    /// and returned as initial value
    fn get_iv<J: Jet>(node: &CommitNodeInner<J>) -> Self;
}

/// Convenience macro for wrappers of `Midstate`.
///
/// Implements `From` to and from `[u8; 32]`,
/// `MerkleRoot`, `AsRef<[u8]>`, `Debug` and `Display`
#[macro_export]
macro_rules! impl_midstate_wrapper {
    ($wrapper:ident) => {
        impl From<[u8; 32]> for $wrapper {
            fn from(data: [u8; 32]) -> Self {
                $wrapper(Midstate::from_inner(data))
            }
        }

        impl From<$wrapper> for [u8; 32] {
            fn from(x: $wrapper) -> Self {
                x.0.into_inner()
            }
        }

        impl MerkleRoot for $wrapper {}

        impl AsRef<[u8]> for $wrapper {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl std::fmt::Debug for $wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                bitcoin_hashes::hex::format_hex(&self.0.as_ref(), f)
            }
        }

        impl std::fmt::Display for $wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                bitcoin_hashes::hex::format_hex(&self.0.as_ref(), f)
            }
        }

        impl std::str::FromStr for $wrapper {
            type Err = bitcoin_hashes::hex::Error;

            fn from_str(s: &str) -> Result<Self, bitcoin_hashes::hex::Error> {
                let x: [u8; 32] = bitcoin_hashes::hex::FromHex::from_hex(s)?;
                Ok($wrapper(Midstate::from_inner(x)))
            }
        }
    };
}
