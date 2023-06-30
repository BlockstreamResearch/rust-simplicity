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

use honggfuzz::fuzz;

use simplicity::jet::Core;
use simplicity::{BitIter, BitWriter, RedeemNode};

fn do_test(data: &[u8]) {
    let mut iter = BitIter::new(data.iter().cloned());

    if let Ok(program) = RedeemNode::<Core>::decode(&mut iter) {
        let bit_len = iter.n_total_read();

        let mut sink = Vec::<u8>::new();
        let mut w = BitWriter::from(&mut sink);
        program.encode(&mut w).expect("encoding to vector");
        w.flush_all().expect("flushing");
        assert_eq!(w.n_total_written(), bit_len);

        // RedeemNode::<Value, Core>::decode() may stop reading `data` mid-byte:
        // copy trailing bits from `data` to `sink`
        if bit_len % 8 != 0 {
            let mask = !(1u8 << (8 - (bit_len % 8)));
            let idx = sink.len() - 1;
            sink[idx] |= data[idx] & mask;
        }
        assert_eq!(sink, &data[0..sink.len()]);
    }
}

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
        extend_vec_from_hex("00000", &mut a);
        super::do_test(&a);
    }
}
