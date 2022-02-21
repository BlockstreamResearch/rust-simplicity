// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

use bitcoin_hashes::{sha256, HashEngine};

use crate::{util::u64_to_array_be, Value};
/// Core Module for simplicity
pub mod term;
pub mod term_dag;
pub mod types;
pub mod value;

/// Simplicity has a different logic for computing the transaction input and output
/// digest. This trait defines the method for computation of such digests.
pub(crate) trait SimplicityHash {
    /// Add the hash of current tx component
    fn simplicity_hash(&self, eng: &mut sha256::HashEngine);
}

/// This is basically reimplementing the SHA256 algorithm completely
/// but since the bitcoin_hashes does not support input message when
/// it is not a multiple of 8 bits. We have to reimplement that here.
/// Consumes the value as we are reading it anyways.
pub(crate) fn sha256_value(v: &Value) -> [u8; 32] {
    let (mut bytes, bit_length) = v.to_bytes_len();

    // Append single '1' bit
    if bit_length % 8 == 0 {
        bytes.push(0x80);
    } else {
        let delimiter_index = bit_length % 8;
        *bytes.last_mut().unwrap() |= 1 << (7 - delimiter_index);
    }

    // Append k '0x00' bytes, where k is minimum number >= 0 such that bytes.len() + k + 8 is multiple of 64
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

    // Append bit_length as 64-bit bit-endian integer
    let bit_length_bytes = u64_to_array_be(bit_length as u64);
    bytes.extend(&bit_length_bytes);
    debug_assert!(bytes.len() % 16 == 0);

    // Compute hash normally since bytes.len() is multiple of 64
    let mut consumed = 0;
    let mut engine = sha256::HashEngine::default();
    while consumed < bytes.len() {
        engine.input(&bytes[consumed..(consumed + 16)]);
        consumed += 16;
    }
    debug_assert!(consumed == bytes.len());
    engine.midstate().into_inner()
}
