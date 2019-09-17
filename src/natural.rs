
use std::mem;

use Error;

/// Encode a natural number according to section 7.2.1
/// of the Simplicity tech report
pub fn encode(n: usize) -> Vec<bool> {
    assert_ne!(n, 0); // Cannot encode zero
    let len = 8 * mem::size_of::<usize>() - n.leading_zeros() as usize - 1;

    if len == 0 {
        vec![false]
    } else {
        let mut ret = vec![true];
        ret.extend(encode(len));
        let idx = ret.len();
        let mut n = n - (1 << len as u32);
        for _ in 0..len {
            ret.push(n % 2 == 1);
            n /= 2;
        }
        ret[idx..].reverse();
        ret
    }
}

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
pub fn decode<BitStream: Iterator<Item = bool>>(
    mut iter: BitStream,
) -> Result<usize, Error> {
    let mut recurse_depth = 0;
    loop {
        match iter.next() {
            Some(true) => recurse_depth += 1,
            Some(false) => break,
            None => return Err(Error::EndOfStream),
        }
    }

    let mut len = 0;
    loop {
        let mut n = 1;
        for _ in 0..len {
            let bit = match iter.next() {
                Some(false) => 0,
                Some(true) => 1,
                None => return Err(Error::EndOfStream),
            };
            n = 2 * n + bit;
        }

        if recurse_depth == 0 {
            return Ok(n)
        } else {
            len = n;
            if len > 31 {
                return Err(Error::NaturalOverflow);
            }
            recurse_depth -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_fixed() {
        let tries = vec![
            (1, vec![false]),
            (2, vec![true, false, false]),
            (3, vec![true, false, true]),
            (4, vec![true, true, false, false, false, false]),
            (5, vec![true, true, false, false, false, true]),
            (6, vec![true, true, false, false, true, false]),
            (7, vec![true, true, false, false, true, true]),
            (8, vec![true, true, false, true, false, false, false]),
            (15, vec![true, true, false, true, true, true, true]),
            (16, vec![
                true, true, true,
                false, // len: 1
                false, // len: 2
                false, false, // len: 4
                false, false, false, false,
            ]),
            // 31
            (31, vec![
                true, true, true,
                false, // len: 1
                false, // len: 2
                false, false, // len: 4
                true, true, true, true,
            ]),
            // 32
            (32, vec![
                true, true, true,
                false, // len: 1
                false, // len: 2
                false, true, // len: 5
                false, false, false, false, false
            ]),
            // 2^15
            (32768, vec![
                true, true, true,
                false, // len: 1
                true, // len: 3
                true, true, true, // len: 15
                false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false,
            ]),
            (65535, vec![
                true, true, true,
                false, // len: 1
                true, // len: 3
                true, true, true, // len: 15
                true, true, true, true, true, true, true, true,
                true, true, true, true, true, true, true,
            ]),
            (65536, vec![
                true, true, true, true,
                false, // len: 1
                false, // len: 2
                false, false, // len: 4
                false, false, false, false, // len: 16
                false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false,
            ]),
        ];

        for (target, vec) in tries {
            let truncated = vec[0..vec.len() - 1].to_vec();
            assert_eq!(
                decode(truncated.into_iter()),
                Err(Error::EndOfStream),
            );
            let encode = encode(target);
            assert_eq!(encode, vec);
            let decode = decode(vec.into_iter()).unwrap();
            assert_eq!(target, decode);
        }
    }
}
