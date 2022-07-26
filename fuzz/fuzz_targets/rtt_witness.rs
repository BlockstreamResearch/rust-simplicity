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

extern crate simplicity;

use simplicity::bititer::BitIter;
use simplicity::core::TermDag;
use simplicity::encode::BitWriter;
use simplicity::jet;
use simplicity::{decode, encode};

fn do_test(data: &[u8]) {
    let mut iter = BitIter::new(data.iter().cloned());
    let typed = TermDag::comp(TermDag::witness(()), TermDag::jet(&jet::core::ADD32))
        .to_linear()
        .type_check()
        .expect("type check");

    if let Ok(witness) = decode::decode_witness(&typed, &mut iter) {
        // println!("{:?}", witness);
        let bit_len = iter.n_total_read();

        let mut sink = Vec::<u8>::new();
        let mut w = BitWriter::from(&mut sink);
        encode::encode_witness(witness.iter(), &mut w).expect("encoding to vector");
        w.flush_all().expect("flushing");
        assert_eq!(w.n_total_written(), bit_len);

        // decode_witness() may stop reading `data` mid-byte:
        // copy trailing bits from `data` to `sink`
        if bit_len % 8 != 0 {
            let mask = !(1u8 << (8 - (bit_len % 8)));
            let idx = sink.len() - 1;
            sink[idx] |= data[idx] & mask;
        }
        assert_eq!(sink, &data[0..sink.len()]);
    }
}

#[cfg(feature = "afl")]
#[macro_use]
extern crate afl;
#[cfg(feature = "afl")]
fn main() {
    fuzz!(|data| {
        do_test(&data);
    });
}

#[cfg(feature = "honggfuzz")]
#[macro_use]
extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
    loop {
        fuzz!(|data| {
            do_test(data);
        });
    }
}
