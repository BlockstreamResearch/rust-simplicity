// SPDX-License-Identifier: CC0-1.0

use honggfuzz::fuzz;

use simplicity::jet::Core;
use simplicity::{BitIter, BitWriter, RedeemNode};

fn do_test(data: &[u8]) {
    let prog_iter = BitIter::new(data.iter().cloned());
    let wit_iter = BitIter::new(core::iter::repeat(0));
    if let Ok(program) = RedeemNode::<Core>::decode(prog_iter, wit_iter) {
        let mut prog_reser = Vec::<u8>::new();
        let mut wit_reser = std::io::sink();

        let mut prog_w = BitWriter::from(&mut prog_reser);
        let mut wit_w = BitWriter::from(&mut wit_reser);
        program
            .encode(&mut prog_w, &mut wit_w)
            .expect("encoding to vector");
        prog_w.flush_all().expect("flushing");
        wit_w.flush_all().expect("flushing");

        assert_eq!(prog_reser, &data[0..prog_reser.len()]);
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
