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

extern crate simplicity;

use simplicity::bititer::BitIter;
use simplicity::encode::{self, BitWrite};

fn do_test(data: &[u8]) {
    let mut read_iter = BitIter::new(data.iter().cloned());
    if let Ok(nat) = encode::decode_natural(&mut read_iter) {
        let len = read_iter.n_total_read();

        let bit_vec: Vec<bool> = BitIter::new(data.iter().cloned()).take(len).collect();
        let mut enc_vec = Vec::<bool>::new();
        encode::encode_natural(nat, &mut enc_vec)
            .expect("encoding natural");
        assert_eq!(bit_vec, enc_vec);

        let mut w = encode::BitWriter::new(Vec::<u8>::new());
        let write_len = encode::encode_natural(nat, &mut w)
            .expect("encoding natural");
        w.flush_all().expect("flushing");
        let mut output = w.into_inner();
        if write_len % 8 != 0 {
            let mask = !(1u8 << (8 - (write_len % 8)));
            let idx = output.len() - 1;
            output[idx] |= data[idx] & mask;
        }
        assert_eq!(output, &data[0..output.len()]);
    }
}

#[cfg(feature = "afl")]
#[macro_use] extern crate afl;
#[cfg(feature = "afl")]
fn main() {
    fuzz!(|data| {
        do_test(&data);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}

#[cfg(test)]
mod tests {
    fn extend_vec_from_hex(hex: &str, out: &mut Vec<u8>) {
        let mut b = 0;
        for (idx, c) in hex.as_bytes().iter().enumerate() {
            b <<= 4;
            match *c {
                b'A'..=b'F' => b |= c - b'A' + 10,
                b'a'..=b'f' => b |= c - b'a' + 10,
                b'0'..=b'9' => b |= c - b'0',
                _ => panic!("Bad hex"),
            }
            if (idx & 1) == 1 {
                out.push(b);
                b = 0;
            }
        }
    }

    #[test]
    fn duplicate_crash() {
        let mut a = Vec::new();
        extend_vec_from_hex("00", &mut a);
        super::do_test(&a);
    }
}
