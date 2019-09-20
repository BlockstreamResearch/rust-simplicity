
use std::mem;

// Generic node
use Node as GenNode;

/// De/serialization error
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Number exceeded 32 bits
    NaturalOverflow,
    /// Non-'case' nodes may not have hidden children
    NonCaseHiddenChild,
    /// 'case' nodes may have at most one hidden child
    CaseMultipleHiddenChildren,
    /// Bitstream ended early
    EndOfStream,
    /// Unrecognized node
    ParseError,
}

/// A node with no witness data which refers to other nodes by
/// relative index
pub type Node = GenNode<usize, ()>;

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
pub fn decode_node_no_witness<BitStream: Iterator<Item = bool>>(
    mut iter: BitStream,
) -> Result<Node, Error> {
    let b1 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    let b2 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    let b3 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    let b4 = match iter.next() {
        Some(bit) => bit,
        None => return Err(Error::EndOfStream),
    };
    match (b1, b2, b3, b4) {
        (false, true, true, false) => {
            let mut h = [0; 32];
            for i in 0..32 {
                for b in 0..8 {
                    match iter.next() {
                        Some(true) => h[i] |= 1 << (7 - b),
                        Some(false) => {},
                        None => return Err(Error::EndOfStream),
                    };
                }
            }
            Ok(GenNode::Hidden(h))
        },
        (false, true, true, true) => {
            Ok(GenNode::Witness(()))
        },
        _ => {
            let b5 = match iter.next() {
                Some(bit) => bit,
                None => return Err(Error::EndOfStream),
            };
            match (b1, b2, b3, b4, b5) {
                (false, false, false, false, false) => {
                    Ok(GenNode::Comp(
                        decode_natural(&mut iter)?,
                        decode_natural(&mut iter)?,
                    ))
                },
                (false, false, false, false, true) => {
                    Ok(GenNode::Case(
                        decode_natural(&mut iter)?,
                        decode_natural(&mut iter)?,
                    ))
                },
                (false, false, false, true, false) => {
                    Ok(GenNode::Pair(
                        decode_natural(&mut iter)?,
                        decode_natural(&mut iter)?,
                    ))
                },
                (false, false, false, true, true) => {
                    Ok(GenNode::Disconnect(
                        decode_natural(&mut iter)?,
                        decode_natural(&mut iter)?,
                    ))
                },
                (false, false, true, false, false)
                    => Ok(GenNode::InjL(decode_natural(&mut iter)?)),
                (false, false, true, false, true)
                    => Ok(GenNode::InjR(decode_natural(&mut iter)?)),
                (false, false, true, true, false)
                    => Ok(GenNode::Take(decode_natural(&mut iter)?)),
                (false, false, true, true, true)
                    => Ok(GenNode::Drop(decode_natural(&mut iter)?)),
                (false, true, false, false, false) => Ok(GenNode::Iden),
                (false, true, false, false, true) => Ok(GenNode::Unit),
                (false, true, false, true, false) => {
                    let mut h1 = [0; 32];
                    let mut h2 = [0; 32];
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h1[i] |= 1 << (7 - b),
                                Some(false) => {},
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h2[i] |= 1 << (7 - b),
                                Some(false) => {},
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    Ok(GenNode::Fail(h1, h2))
                },
                (false, true, false, true, true) => Err(Error::ParseError),
                (true, _, _, _, _) => Err(Error::ParseError),
                (false, true, true, _, _) => unreachable!(),
            }
        },
    }
}

pub fn decode_program_no_witness<BitStream: Iterator<Item = bool>>(
    mut iter: BitStream,
) -> Result<Vec<Node>, Error> {
    let prog_len = decode_natural(&mut iter)?;

    let mut program = Vec::with_capacity(prog_len);
    for _ in 0..prog_len {
        program.push(decode_node_no_witness(&mut iter)?);
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
            let encode = encode_natural(target);
            assert_eq!(encode, vec);
            let decode = decode_natural(vec.into_iter()).unwrap();
            assert_eq!(target, decode);
        }
    }
}
