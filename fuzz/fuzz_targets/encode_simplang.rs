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
use simplang::parse::parse;
use simplicity::bititer::BitIter;
use simplicity::jet::Elements;
use simplicity::Tmr;

fn do_test(data: &[u8]) {
    let mut iter = BitIter::new(data.iter().cloned());

    if let Ok(program) = simplicity::decode_program_fresh_witness::<_, Elements>(&mut iter) {
        // Manually check that this is a 1-1 program
        match program.arrow().source.finalize() {
            Ok(source) if source.tmr() == Tmr::unit() => {},
            _ => return,
        }
        // Manually check that this is a 1-1 program
        match program.arrow().target.finalize() {
            Ok(target) if target.tmr() == Tmr::unit() => {},
            _ => return,
        }

        let reserialize= simplang::decompile::to_human_readable(&program);
        println!("{reserialize}");
        let round_trip = parse::<Elements>(&reserialize).unwrap();
        assert_eq!(program, round_trip);
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
        extend_vec_from_hex("286969010000000000003e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e3e7aadd33e4d4e62e17a7666313433333333333303abaaaaaaaaaa5502c0000000006574732bb035", &mut a);
        super::do_test(&a);
    }
}
