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

//! # Decoding
//!
//! Functionality to decode Simplicity programs.
//! Refer to [`crate::encode`] for information on the encoding.

use crate::bititer::BitIter;
use crate::core::commit::CommitNodeInner;
use crate::core::iter::WitnessIterator;
use crate::core::{CommitNode, Context, Value};
use crate::dag::{DagLike, InternalSharing};
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::types;
use crate::Error;
use std::rc::Rc;

/// Decode a Simplicity program from bits, without the witness data.
///
/// If witness data is present, it should be encoded after the program, and the
/// user must deserialize it separately.
pub fn decode_program<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
) -> Result<Rc<CommitNode<J>>, Error> {
    let root = decode_program_arbitrary_type(bits)?;
    let unit_ty = crate::types::Type::unit();
    root.arrow()
        .source
        .unify(&unit_ty, "setting root source to unit")?;
    root.arrow()
        .target
        .unify(&unit_ty, "setting root source to unit")?;
    Ok(root)
}

pub fn decode_program_arbitrary_type<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
) -> Result<Rc<CommitNode<J>>, Error> {
    let len = decode_natural(bits, None)?;

    if len == 0 {
        return Err(Error::EmptyProgram);
    }
    // FIXME: check maximum length of DAG that is allowed by consensus
    if len > 1_000_000 {
        return Err(Error::TooManyNodes(len));
    }

    let mut context = Context::new();

    let mut nodes = vec![];
    for _ in 0..len {
        let new_node = decode_node(bits, &mut context, &nodes[..])?;
        nodes.push(new_node);
    }

    // We must check the canonical order of the serialized program
    for data in nodes[nodes.len() - 1]
        .as_ref()
        .post_order_iter::<InternalSharing>()
    {
        if data.index >= nodes.len() || data.node != nodes[data.index].as_ref() {
            return Err(Error::NotInCanonicalOrder);
        }
    }

    Ok(Rc::clone(&nodes[nodes.len() - 1]))
}

/// Decode a single Simplicity node from bits and
/// insert it into a hash map at its index for future reference by ancestor nodes.
fn decode_node<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
    context: &mut Context<J>,
    nodes: &[Rc<CommitNode<J>>],
) -> Result<Rc<CommitNode<J>>, Error> {
    let index = nodes.len();

    let (maybe_code, subcode) = match bits.next() {
        None => return Err(Error::EndOfStream),
        Some(true) => (None, u64::default()),
        Some(false) => {
            let code = bits.read_bits_be(2).ok_or(Error::EndOfStream)?;
            let subcode = bits
                .read_bits_be(if code < 3 { 2 } else { 1 })
                .ok_or(Error::EndOfStream)?;
            (Some(code), subcode)
        }
    };

    let node = match maybe_code {
        Some(code) if code < 2 => {
            let i_abs = index - decode_natural(bits, Some(index))?;
            let left = Rc::clone(&nodes[i_abs]);

            match code {
                0 => {
                    let j_abs = index - decode_natural(bits, Some(index))?;
                    let right = Rc::clone(&nodes[j_abs]);

                    match subcode {
                        0 => CommitNode::comp(context, left, right),
                        1 => {
                            if let CommitNodeInner::Hidden(..) = left.inner() {
                                if let CommitNodeInner::Hidden(..) = right.inner() {
                                    return Err(Error::BothChildrenHidden);
                                }
                            }

                            if let CommitNodeInner::Hidden(right_hash) = right.inner() {
                                CommitNode::assertl(context, left, *right_hash)
                            } else if let CommitNodeInner::Hidden(left_hash) = left.inner() {
                                CommitNode::assertr(context, *left_hash, right)
                            } else {
                                CommitNode::case(context, left, right)
                            }
                        }
                        2 => CommitNode::pair(context, left, right),
                        3 => CommitNode::disconnect(context, left, right),
                        _ => unreachable!("2-bit subcode"),
                    }
                }
                1 => Ok(match subcode {
                    0 => CommitNode::injl(context, left),
                    1 => CommitNode::injr(context, left),
                    2 => CommitNode::take(context, left),
                    3 => CommitNode::drop(context, left),
                    _ => unreachable!("2-bit subcode"),
                }),
                _ => unreachable!("code < 2"),
            }
        }
        _ => match maybe_code {
            None => match bits.next() {
                None => return Err(Error::EndOfStream),
                Some(true) => Ok(CommitNode::jet(context, J::decode(bits)?)),
                Some(false) => {
                    let depth = decode_natural(bits, Some(32))?;
                    let word = decode_value(&context.nth_power_of_2_final(depth - 1), bits)?;
                    Ok(CommitNode::const_word(context, word))
                }
            },
            Some(2) => Ok(match subcode {
                0 => CommitNode::iden(context),
                1 => CommitNode::unit(context),
                2 => CommitNode::fail(
                    context,
                    Cmr::from(decode_hash(bits)?),
                    Cmr::from(decode_hash(bits)?),
                ),
                3 => return Err(Error::ParseError("01011 (stop code)")),
                _ => unreachable!("2-bit subcode"),
            }),
            Some(3) => Ok(match subcode {
                0 => CommitNode::hidden(context, Cmr::from(decode_hash(bits)?)),
                1 => CommitNode::witness(context),
                _ => unreachable!("1-bit subcode"),
            }),
            Some(_) => unreachable!("2-bit code"),
        },
    }?;

    Ok(node)
}

/// Implementation of [`WitnessIterator`] for an underlying [`BitIter`].
#[derive(Debug)]
pub struct WitnessDecoder<'a, I: Iterator<Item = u8>> {
    pub bits: &'a mut BitIter<I>,
    pub max_n: usize,
}

impl<'a, I: Iterator<Item = u8>> WitnessDecoder<'a, I> {
    /// Create a new witness decoder for the given bit iterator.
    ///
    /// # Usage
    ///
    /// This method must be used **after** the program serialization has been read by the iterator.
    pub fn new(bits: &'a mut BitIter<I>) -> Result<Self, Error> {
        let bit_len = match bits.next() {
            Some(false) => 0,
            Some(true) => decode_natural(bits, None)?,
            None => return Err(Error::EndOfStream),
        };
        let n_start = bits.n_total_read();

        Ok(Self {
            bits,
            max_n: n_start + bit_len,
        })
    }
}

impl<'a, I: Iterator<Item = u8>> WitnessIterator for WitnessDecoder<'a, I> {
    fn next(&mut self, ty: &types::Final) -> Result<Value, Error> {
        decode_value(ty, self.bits)
    }

    fn finish(self) -> Result<(), Error> {
        if self.bits.n_total_read() != self.max_n {
            Err(Error::InconsistentWitnessLength)
        } else {
            Ok(())
        }
    }
}

/// Decode a value from bits, based on the given type.
pub fn decode_value<I: Iterator<Item = bool>>(
    ty: &types::Final,
    iter: &mut I,
) -> Result<Value, Error> {
    let value = match ty.bound() {
        types::CompleteBound::Unit => Value::Unit,
        types::CompleteBound::Sum(ref l, ref r) => match iter.next() {
            Some(false) => Value::SumL(Box::new(decode_value(l, iter)?)),
            Some(true) => Value::SumR(Box::new(decode_value(r, iter)?)),
            None => return Err(Error::EndOfStream),
        },
        types::CompleteBound::Product(ref l, ref r) => Value::Prod(
            Box::new(decode_value(l, iter)?),
            Box::new(decode_value(r, iter)?),
        ),
    };

    Ok(value)
}

/// Decode a 256-bit hash from bits.
pub fn decode_hash<I: Iterator<Item = u8>>(iter: &mut BitIter<I>) -> Result<[u8; 32], Error> {
    let mut h = [0; 32];

    for b in &mut h {
        match iter.read_bits_be(8) {
            Some(n) => *b = n as u8,
            None => return Err(Error::EndOfStream),
        }
    }

    Ok(h)
}

/// Decode a natural number from bits.
/// If a bound is specified, then the decoding terminates before trying to decode a larger number.
pub fn decode_natural<I: Iterator<Item = bool>>(
    iter: &mut I,
    bound: Option<usize>,
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
            if let Some(bound) = bound {
                if n > bound {
                    return Err(Error::BadIndex);
                }
            }
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
    use crate::bitwriter::BitWriter;
    use crate::encode;
    use crate::exec::BitMachine;

    #[test]
    fn canonical_order() {
        // "main = comp unit iden", but with the iden serialized before the unit
        // To obtain this test vector I temporarily swapped `get_left` and `get_right`
        // in the implementation of `PostOrderIter`
        let justwit = vec![0xa8, 0x48, 0x10];
        let mut iter = BitIter::from(&justwit[..]);
        if let Err(Error::NotInCanonicalOrder) = decode_program::<_, crate::jet::Core>(&mut iter) {
            // ok
        } else {
            panic!("accepted program with non-canonical order")
        }
    }

    #[test]
    fn shared_witnesses() {
        // main = witness :: A -> B
        let justwit = vec![0x38];
        let mut iter = BitIter::from(&justwit[..]);
        decode_program::<_, crate::jet::Core>(&mut iter).unwrap();

        // # Program which demands two 32-bit witnesses, the first one == the second + 1
        // wit1 = witness :: 1 -> 2^32
        // wit2 = witness :: 1 -> 2^32
        //
        // wit_diff = comp (comp (pair wit1 wit2) jet_subtract_32) (drop iden) :: 1 -> 2^32
        // diff_is_one = comp (pair wit_diff jet_one_32) jet_eq_32             :: 1 -> 2
        // main = comp diff_is_one jet_verify                                  :: 1 -> 1
        #[rustfmt::skip]
        let diff1 = vec![
            0xda, 0xe2, 0x39, 0xa3, 0x10, 0x42, 0x0e, 0x05,
            0x71, 0x88, 0xa3, 0x6d, 0xc4, 0x11, 0x80, 0x80
        ];
        let mut iter = BitIter::from(&diff1[..]);
        let diff1_prog = decode_program::<_, crate::jet::Core>(&mut iter).unwrap();

        // Attempt to finalize, providing 32-bit witnesses 0, 1, ..., and then
        // counting how many were consumed afterward.
        let mut counter = 0..100;
        let mut witness_iter = (&mut counter).rev().map(Value::u32);
        let diff1_final = diff1_prog.finalize(&mut witness_iter, true).unwrap();
        assert_eq!(counter, 0..98);

        // Execute the program to confirm that it worked
        let mut mac = BitMachine::for_program(&diff1_final);
        mac.exec(&diff1_final, &()).unwrap();
    }

    #[test]
    fn root_unit_to_unit() {
        // main = jet_eq_32 :: 2^64 -> 2 # 7387d279
        let justjet = vec![0x6d, 0xb8, 0x80];
        // Should be able to decode this as a CommitNode...
        let mut iter = BitIter::from(&justjet[..]);
        CommitNode::<crate::jet::Core>::decode::<_>(&mut iter).unwrap();
        // ...but not as a program
        let mut iter = BitIter::from(&justjet[..]);
        decode_program::<_, crate::jet::Core>(&mut iter).unwrap_err();
    }

    #[test]
    #[cfg(feature = "elements")]
    fn decode_schnorr() {
        #[rustfmt::skip]
        let schnorr0 = vec![
            0xc6, 0xd5, 0xf2, 0x61, 0x14, 0x03, 0x24, 0xb1, 0x86, 0x20, 0x92, 0x68, 0x9f, 0x0b, 0xf1, 0x3a,
            0xa4, 0x53, 0x6a, 0x63, 0x90, 0x8b, 0x06, 0xdf, 0x33, 0x61, 0x0c, 0x03, 0xe2, 0x27, 0x79, 0xc0,
            0x6d, 0xf2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xe2, 0x8d, 0x8c, 0x04, 0x7a, 0x40, 0x1d, 0x20, 0xf0, 0x63, 0xf0, 0x10, 0x91, 0xa2,
            0x0d, 0x34, 0xa6, 0xe3, 0x68, 0x04, 0x82, 0x06, 0xc9, 0x7b, 0xe3, 0x8b, 0xf0, 0x60, 0xf6, 0x01,
            0x09, 0x8a, 0xbe, 0x39, 0xc5, 0xb9, 0x50, 0x42, 0xa4, 0xbe, 0xcd, 0x49, 0x50, 0xbd, 0x51, 0x6e,
            0x3c, 0x90, 0x54, 0xe9, 0xe7, 0x05, 0xa5, 0x9c, 0xbd, 0x7d, 0xdd, 0x1f, 0xb6, 0x42, 0xe5, 0xe8,
            0xef, 0xbe, 0x92, 0x01, 0xa6, 0x20, 0xa6, 0xd8, 0x00
        ];
        let mut iter = BitIter::from(&schnorr0[..]);
        let prog =
            decode_program::<_, crate::jet::Elements>(&mut iter).expect("can't decode schnorr0");

        // Matches C source code
        #[rustfmt::skip]
        assert_eq!(
            prog.cmr(),
            [
                0x7b, 0xc5, 0x6c, 0xb1, 0x6d, 0x84, 0x99, 0x9b,
                0x97, 0x7b, 0x58, 0xe1, 0xbc, 0x71, 0xdb, 0xe9,
                0xed, 0xcc, 0x33, 0x65, 0x0a, 0xfc, 0x8a, 0x6e,
                0xe0, 0x5c, 0xfe, 0xf8, 0xd6, 0x08, 0x13, 0x2b, 
            ].into(),
        );
    }

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

        for (natural, bitvec) in tries {
            let truncated = bitvec[0..bitvec.len() - 1].to_vec();
            assert_matches!(
                decode_natural(&mut truncated.into_iter(), None),
                Err(Error::EndOfStream)
            );

            let mut sink = Vec::<u8>::new();

            let mut w = BitWriter::from(&mut sink);
            encode::encode_natural(natural, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");
            assert_eq!(w.n_total_written(), bitvec.len());

            let decoded_natural = decode_natural(&mut BitIter::from(sink.into_iter()), None)
                .expect("decoding from vector");
            assert_eq!(natural, decoded_natural);
        }
    }
}
