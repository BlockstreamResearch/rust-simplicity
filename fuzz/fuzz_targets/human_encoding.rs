// SPDX-License-Identifier: CC0-1.0

use honggfuzz::fuzz;

use simplicity::human_encoding::Forest;
use simplicity::jet::Elements;

fn do_test(data: &[u8]) {
    let str = match std::str::from_utf8(data) {
        Ok(str) => str,
        Err(_) => return,
    };
    let _forest = match Forest::<Elements>::parse(str) {
        Ok(forest) => forest,
        Err(_) => return,
    };
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
