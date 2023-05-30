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
use crate::core::iter::{DagIterable, IndexDag, IndexNode, WitnessIterator};
use crate::core::types::{Type, TypeInner};
use crate::core::{CommitNode, Context, Value};
use crate::jet::Jet;
use crate::merkle::cmr::Cmr;
use crate::Error;
use std::collections::HashMap;
use std::rc::Rc;

/// Decode a Simplicity program from bits where witness data follows.
///
/// The number of decoded witness nodes corresponds exactly to the witness data.
pub fn decode_program_exact_witness<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
) -> Result<Rc<CommitNode<J>>, Error> {
    decode_program(bits, false)
}

/// Decode a Simplicity program from bits where there is no witness data.
///
/// Witness nodes are made fresh, i.e., each witness node has at most one parent.
/// This increases the number of witness nodes and hence the length of the required witness data.
pub fn decode_program_fresh_witness<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
) -> Result<Rc<CommitNode<J>>, Error> {
    decode_program(bits, true)
}

/// Decode a Simplicity program from bits, without the witness data.
fn decode_program<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
    fresh_witness: bool,
) -> Result<Rc<CommitNode<J>>, Error> {
    let len = decode_natural(bits, None)?;

    if len == 0 {
        return Err(Error::EmptyProgram);
    }
    // FIXME: check maximum length of DAG that is allowed by consensus
    if len > 1_000_000 {
        return Err(Error::TooManyNodes(len));
    }

    let mut index_to_node = HashMap::new();
    let mut index_dag = Vec::new();
    let mut context = Context::new();

    for index in 0..len {
        decode_node(
            bits,
            &mut context,
            index,
            &mut index_to_node,
            &mut index_dag,
            fresh_witness,
        )?;
    }

    // If witnesses are made fresh, then the deserialized program grows and its indices change.
    // We must check the canonical order of the serialized program
    let node = IndexNode(usize::default(), &index_dag);
    let connected_it = node.iter_post_order();
    let indexed_it = (0..index_dag.len()).map(|index| IndexNode(index, &index_dag));

    if !Iterator::eq(connected_it, indexed_it) {
        return Err(Error::NotInCanonicalOrder);
    }

    let root = index_to_node.get(&(len - 1)).unwrap();
    Ok(root.clone())
}

/// Decode a single Simplicity node from bits and
/// insert it into a hash map at its index for future reference by ancestor nodes.
fn decode_node<I: Iterator<Item = u8>, J: Jet>(
    bits: &mut BitIter<I>,
    context: &mut Context<J>,
    index: usize,
    index_to_node: &mut HashMap<usize, Rc<CommitNode<J>>>,
    index_dag: &mut IndexDag,
    fresh_witness: bool,
) -> Result<(), Error> {
    debug_assert!(index == index_dag.len());

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
            let left = get_child_from_index(context, i_abs, index_to_node);

            match code {
                0 => {
                    let j_abs = index - decode_natural(bits, Some(index))?;
                    let right = get_child_from_index(context, j_abs, index_to_node);
                    index_dag.push((Some(i_abs), Some(j_abs)));

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
                1 => {
                    index_dag.push((Some(i_abs), None));

                    Ok(match subcode {
                        0 => CommitNode::injl(context, left),
                        1 => CommitNode::injr(context, left),
                        2 => CommitNode::take(context, left),
                        3 => CommitNode::drop(context, left),
                        _ => unreachable!("2-bit subcode"),
                    })
                }
                _ => unreachable!("code < 2"),
            }
        }
        _ => {
            index_dag.push((None, None));

            match maybe_code {
                None => match bits.next() {
                    None => return Err(Error::EndOfStream),
                    Some(true) => Ok(CommitNode::jet(context, J::decode(bits)?)),
                    Some(false) => {
                        let depth = decode_natural(bits, Some(32))?;
                        let word = decode_value(&context.nth_power_of_2_type(depth - 1), bits)?;
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
                    1 if fresh_witness => return Ok(()),
                    1 => CommitNode::witness(context),
                    _ => unreachable!("1-bit subcode"),
                }),
                Some(_) => unreachable!("2-bit code"),
            }
        }
    }?;

    debug_assert!(!index_to_node.contains_key(&index));
    index_to_node.insert(index, node);

    Ok(())
}

/// Return the child node at the given index from a hash map.
///
/// A fresh witness node is returned as child if witness nodes should be made fresh.
fn get_child_from_index<J: Jet>(
    context: &mut Context<J>,
    index: usize,
    index_to_node: &HashMap<usize, Rc<CommitNode<J>>>,
) -> Rc<CommitNode<J>> {
    match index_to_node.get(&index) {
        Some(child) => child.clone(),
        // Absence of child means that child is a skipped witness node
        None => CommitNode::witness(context),
    }
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
    fn next(&mut self, ty: &Type) -> Result<Value, Error> {
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
pub fn decode_value<I: Iterator<Item = bool>>(ty: &Type, iter: &mut I) -> Result<Value, Error> {
    let value = match ty.inner {
        TypeInner::Unit => Value::Unit,
        TypeInner::Sum(ref l, ref r) => match iter.next() {
            Some(false) => Value::SumL(Box::new(decode_value(l, iter)?)),
            Some(true) => Value::SumR(Box::new(decode_value(r, iter)?)),
            None => return Err(Error::EndOfStream),
        },
        TypeInner::Product(ref l, ref r) => Value::Prod(
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

    #[test]
    #[cfg(feature = "elements")]
    fn decode_schnorr() {
        #[rustfmt::skip]
        let mut schnorr0 = vec![
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
        let prog =
            decode_program_exact_witness::<_, crate::jet::Elements>(&mut schnorr0[..].into())
                .expect("can't decode schnorr0");

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
