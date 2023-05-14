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

use simplicity::bititer::BitIter;
use simplicity::bitwriter::BitWriter;
use simplicity::core::iter::WitnessIterator;
use simplicity::core::types::Type;
use simplicity::{WitnessDecoder, encode_witness};

fn do_test(data: &[u8]) {
    let mut iter = BitIter::new(data.iter().cloned());

    if let Ok(mut decoder) = WitnessDecoder::new(&mut iter) {
        let witness_bit_len = decoder.max_n - decoder.bits.n_total_read();
        // Prevent huge allocations
        if witness_bit_len > 1_000_000 {
            return;
        }

        let mut witness = Vec::with_capacity(witness_bit_len);
        let two_0 = Type::unit();
        let two_1 = Type::sum(two_0.clone(), two_0);

        for _ in 0..witness_bit_len {
            match decoder.next(&two_1) {
                Ok(value) => witness.push(value),
                Err(_) => return,
            }
        }

        assert!(decoder.finish().is_ok());
        assert_eq!(witness.len(), witness_bit_len);
        let bit_len = iter.n_total_read();

        let mut sink = Vec::<u8>::new();
        let mut w = BitWriter::from(&mut sink);
        encode_witness(witness.iter(), &mut w).expect("encoding to vector");
        w.flush_all().expect("flushing");
        assert_eq!(w.n_total_written(), bit_len);

        // WitnessDecoder may stop reading `data` mid-byte:
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
