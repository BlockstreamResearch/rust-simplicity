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

/// Core Module for simplicity
pub mod term;
pub mod types;

// handy function for converting bit vector to vec[u8]
// # PANIC:
// panics when bitvec length is not a multiple of 8.
#[allow(dead_code)]
pub(crate) fn bitvec_to_bytevec(bitvec: Vec<bool>) -> Vec<u8> {
    let mut ret = vec![];
    assert!(bitvec.len() % 8 == 0, "Bitvec len must be multiple of 8");
    let mut start = 0;
    while start < bitvec.len() {
        //read a byte
        let mut byte: u8 = 0;
        for i in 0..8 {
            byte += (bitvec[start + i] as u8) * (1u8 << (7 - i));
        }
        ret.push(byte);
        start += 8;
    }
    ret
}
