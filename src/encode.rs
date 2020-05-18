use std::mem;

use bititer::BitIter;
use extension::bitcoin;
use {Error, Node};

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
pub fn decode_node_no_witness<I: Iterator<Item = u8>>(
    idx: usize,
    iter: &mut BitIter<I>,
) -> Result<Node<()>, Error> {
    match iter.next() {
        None => Err(Error::EndOfStream),
        Some(true) => {
            match iter.next() {
                None => Err(Error::EndOfStream),
                Some(false) => Ok(Node::Bitcoin(bitcoin::decode_node_no_witness(iter)?)),
                Some(true) => Err(Error::ParseError("invalid parse 11")),
            }
        },
        Some(false) => {
            let code = match iter.read_bits_be(2) {
                Some(n) => n,
                None => return Err(Error::EndOfStream),
            };
            let subcode = match iter.read_bits_be(if code < 3 { 2 } else { 1 }) {
                Some(n) => n,
                None => return Err(Error::EndOfStream),
            };
            match (code, subcode) {
                (0, 0) => Ok(Node::Comp(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                // FIXME `Case` should check for asserts and reject if both children are hidden
                (0, 1) => Ok(Node::Case(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (0, 2) => Ok(Node::Pair(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (0, 3) => Ok(Node::Disconnect(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 0) => Ok(Node::InjL(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 1) => Ok(Node::InjR(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 2) => Ok(Node::Take(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 3) => Ok(Node::Drop(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (2, 0) => Ok(Node::Iden),
                (2, 1) => Ok(Node::Unit),
                // FIXME Russell's code just rejects `Fail`
                (2, 2) => {
                    let mut h1 = [0; 32];
                    let mut h2 = [0; 32];
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h1[i] |= 1 << (7 - b),
                                Some(false) => {}
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h2[i] |= 1 << (7 - b),
                                Some(false) => {}
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    Ok(Node::Fail(h1, h2))
                },
                (2, 3) => Err(Error::ParseError("01011 (stop code)")),
                (3, 0) => {
                    let mut h = [0; 32];
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h[i] |= 1 << (7 - b),
                                Some(false) => {}
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    Ok(Node::Hidden(h))
                },
                (3, 1) => Ok(Node::Witness(())),
                (_, _) => unreachable!("we read only so many bits"),
            }
        }
    }
}

pub fn decode_program_no_witness<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<Vec<Node<()>>, Error> {
    let prog_len = decode_natural(&mut *iter)?;

    let mut program = Vec::with_capacity(prog_len);
    for i in 0..prog_len {
        program.push(decode_node_no_witness(i, iter)?);
    }

    Ok(program)
}

/// Encode a natural number according to section 7.2.1
/// of the Simplicity tech report
pub fn encode_natural(n: usize) -> Vec<bool> {
    assert_ne!(n, 0); // Cannot encode zero
    let len = 8 * mem::size_of::<usize>() - n.leading_zeros() as usize - 1;

    if len == 0 {
        vec![false]
    } else {
        let mut ret = vec![true];
        ret.extend(encode_natural(len));
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
pub fn decode_natural<BitStream: Iterator<Item = bool>>(
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
            return Ok(n);
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
            (
                16,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    false, false, false, false,
                ],
            ),
            // 31
            (
                31,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    true, true, true, true,
                ],
            ),
            // 32
            (
                32,
                vec![
                    true, true, true, false, // len: 1
                    false, // len: 2
                    false, true, // len: 5
                    false, false, false, false, false,
                ],
            ),
            // 2^15
            (
                32768,
                vec![
                    true, true, true, false, // len: 1
                    true,  // len: 3
                    true, true, true, // len: 15
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ),
            (
                65535,
                vec![
                    true, true, true, false, // len: 1
                    true,  // len: 3
                    true, true, true, // len: 15
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true,
                ],
            ),
            (
                65536,
                vec![
                    true, true, true, true, false, // len: 1
                    false, // len: 2
                    false, false, // len: 4
                    false, false, false, false, // len: 16
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ],
            ),
        ];

        for (target, vec) in tries {
            let truncated = vec[0..vec.len() - 1].to_vec();
            assert_eq!(
                decode_natural(truncated.into_iter()),
                Err(Error::EndOfStream),
            );
            let encode = encode_natural(target);
            assert_eq!(encode, vec);
            let decode = decode_natural(vec.into_iter()).unwrap();
            assert_eq!(target, decode);
        }
    }
}

