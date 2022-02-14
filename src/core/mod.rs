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

use crate::{
    util::{bitvec_to_bytevec, u64_to_array_be},
    Value,
};
/// Core Module for simplicity
pub mod term;
pub mod types;

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
// FIXME: Consider taking a reference and inplementing to_bits() instead
// At first glance, maybe we are still reading the entire value so it
// makes sense to consume it.
pub(crate) fn sha256_value(v: Value) -> [u8; 32] {
    let mut bits = v.into_bits();
    let len = bits.len();
    // Now that we have stored the len modify the bitvec for the sha2
    // specification
    // append a single '1' bit
    bits.push(true);
    // append K '0' bits, where K is the minimum number >= 0 such that L + 1 + K + 64 is a multiple of 512
    let k;
    if len % 512 >= 448 {
        // 448 from the next byte.
        k = 448 + (512 - (len % 512) - 1);
    } else {
        k = 448 - 1 - (len % 512);
    }
    let sz = bits.len();
    bits.resize(sz + k, false);

    debug_assert!(bits.len() % 512 == 448);
    // append L as a 64-bit big-endian integer, making the total post-processed length a multiple of 512 bits
    let mut block_bytes = bitvec_to_bytevec(&bits);
    let len_bytes = u64_to_array_be(len as u64);
    // IntoIter not implemented for [u8; N]. Cheap to allocate agian for 2 bytes
    block_bytes.extend(&len_bytes);
    debug_assert!(block_bytes.len() % 16 == 0);

    // Now compute the hash as normal since data is multiple of 512 bits
    let mut consumed = 0;
    let mut engine = sha256::HashEngine::default();
    while consumed < block_bytes.len() {
        engine.input(&block_bytes[consumed..(consumed + 16)]);
        consumed += 16;
    }
    debug_assert!(consumed == block_bytes.len());
    engine.midstate().into_inner()
}
