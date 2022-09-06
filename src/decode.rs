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
use crate::core::commit::{CommitNodeInner, RefWrapper};
use crate::core::iter::{DagIterable, WitnessIterator};
use crate::core::types::{Type, TypeInner};
use crate::core::{CommitNode, Value};
use crate::jet::Application;
use crate::merkle::cmr::Cmr;
use crate::Error;
use std::collections::HashMap;
use std::rc::Rc;

/// Decode a Simplicity program from bits where witness data follows.
///
/// The number of decoded witness nodes corresponds exactly to the witness data.
pub fn decode_program_exact_witness<I: Iterator<Item = u8>, App: Application>(
    bits: &mut BitIter<I>,
) -> Result<Rc<CommitNode<App>>, Error> {
    decode_program(bits, false)
}

/// Decode a Simplicity program from bits where there is no witness data.
///
/// Witness nodes are made fresh, i.e., each witness node has at most one parent.
/// This increases the number of witness nodes and hence the length of the required witness data.
pub fn decode_program_fresh_witness<I: Iterator<Item = u8>, App: Application>(
    bits: &mut BitIter<I>,
) -> Result<Rc<CommitNode<App>>, Error> {
    decode_program(bits, true)
}

/// Decode a Simplicity program from bits, without the witness data.
fn decode_program<I: Iterator<Item = u8>, App: Application>(
    bits: &mut BitIter<I>,
    fresh_witness: bool,
) -> Result<Rc<CommitNode<App>>, Error> {
    let len = decode_natural(bits, None)?;

    if len == 0 {
        return Err(Error::EmptyProgram);
    }
    // FIXME: check maximum length of DAG that is allowed by consensus
    if len > 1_000_000 {
        return Err(Error::TooManyNodes(len));
    }

    let mut new_index = 0;
    let mut index_to_node = HashMap::new();
    let mut new_index_to_node = HashMap::new();

    for index in 0..len {
        new_index = decode_node(
            bits,
            index,
            new_index,
            &mut index_to_node,
            &mut new_index_to_node,
            fresh_witness,
        )?;
    }

    let new_root_index = new_index - 1;
    let root = new_index_to_node.get(&new_root_index).unwrap().clone();
    let mut it = RefWrapper(&root).iter_post_order();

    // new_len ≥ RefWrapper(&root).iter_post_order().count()
    for new_index in 0..=new_root_index {
        let connected_node = it.next().ok_or(Error::NotInCanonicalOrder)?;
        let indexed_node = RefWrapper(new_index_to_node.get(&new_index).unwrap());

        if connected_node != indexed_node {
            return Err(Error::NotInCanonicalOrder);
        }
    }

    Ok(root)
}

/// Decode a single Simplicity node from bits and
/// insert it into a hash map at its index for future reference by ancestor nodes.
///
/// If witness nodes are made fresh, then the program grows and its indices change.
/// This is why `index` is the node's index in the serialized program,
/// while `new_index` is its index in the deserialized program.
///
/// Returns the next `new_index`
/// because fresh witness nodes may be added to the deserialized program.
fn decode_node<I: Iterator<Item = u8>, App: Application>(
    bits: &mut BitIter<I>,
    index: usize,
    mut new_index: usize,
    index_to_node: &mut HashMap<usize, Rc<CommitNode<App>>>,
    new_index_to_node: &mut HashMap<usize, Rc<CommitNode<App>>>,
    fresh_witness: bool,
) -> Result<usize, Error> {
    match bits.next() {
        None => return Err(Error::EndOfStream),
        Some(true) => {
            let node = CommitNode::jet(App::decode_jet(bits)?);

            debug_assert!(!index_to_node.contains_key(&index));
            debug_assert!(!new_index_to_node.contains_key(&new_index));
            index_to_node.insert(index, node.clone());
            new_index_to_node.insert(new_index, node);

            return Ok(new_index + 1);
        }
        Some(false) => {}
    };

    let code = match bits.read_bits_be(2) {
        Some(n) => n,
        None => return Err(Error::EndOfStream),
    };
    let subcode = match bits.read_bits_be(if code < 3 { 2 } else { 1 }) {
        Some(n) => n,
        None => return Err(Error::EndOfStream),
    };
    let node = if code <= 1 {
        let i_abs = index - decode_natural(bits, Some(index))?;
        let left = get_child_from_index(i_abs, &mut new_index, index_to_node, new_index_to_node);

        if code == 0 {
            let j_abs = index - decode_natural(bits, Some(index))?;
            let right =
                get_child_from_index(j_abs, &mut new_index, index_to_node, new_index_to_node);

            match subcode {
                0 => CommitNode::comp(left, right),
                1 => {
                    if let CommitNodeInner::Hidden(..) = left.inner {
                        if let CommitNodeInner::Hidden(..) = right.inner {
                            return Err(Error::CaseMultipleHiddenChildren);
                        }
                    }

                    if let CommitNodeInner::Hidden(..) = right.inner {
                        CommitNode::assertl(left, right)
                    } else if let CommitNodeInner::Hidden(..) = left.inner {
                        CommitNode::assertr(left, right)
                    } else {
                        CommitNode::case(left, right)
                    }
                }
                2 => CommitNode::pair(left, right),
                3 => CommitNode::disconnect(left, right),
                // TODO: convert into crate::Error::ParseError
                _ => unimplemented!(),
            }
        } else {
            match subcode {
                0 => CommitNode::injl(left),
                1 => CommitNode::injr(left),
                2 => CommitNode::take(left),
                3 => CommitNode::drop(left),
                _ => unimplemented!(),
            }
        }
    } else if code == 2 {
        match subcode {
            0 => CommitNode::iden(),
            1 => CommitNode::unit(),
            2 => CommitNode::fail(Cmr::from(decode_hash(bits)?), Cmr::from(decode_hash(bits)?)),
            3 => return Err(Error::ParseError("01011 (stop code)")),
            _ => unimplemented!(),
        }
    } else if code == 3 {
        match subcode {
            0 => CommitNode::hidden(Cmr::from(decode_hash(bits)?)),
            1 if fresh_witness => return Ok(index),
            1 => CommitNode::witness(),
            _ => unimplemented!(),
        }
    } else {
        unimplemented!()
    };

    debug_assert!(!index_to_node.contains_key(&index));
    debug_assert!(!new_index_to_node.contains_key(&new_index));
    index_to_node.insert(index, node.clone());
    new_index_to_node.insert(new_index, node);

    Ok(new_index + 1)
}

/// Return the child node at the given index from a hash map.
///
/// A fresh witness node is returned as child and inserted into the program
/// if witness nodes should be made fresh.
/// The parent index and index hash maps are updated accordingly.
fn get_child_from_index<App: Application>(
    child_index: usize,
    new_parent_index: &mut usize,
    index_to_node: &HashMap<usize, Rc<CommitNode<App>>>,
    new_index_to_node: &mut HashMap<usize, Rc<CommitNode<App>>>,
) -> Rc<CommitNode<App>> {
    match index_to_node.get(&child_index) {
        Some(child) => child.clone(),
        // Absence of child means that child is a skipped witness node
        None => {
            let child = CommitNode::witness();
            debug_assert!(!new_index_to_node.contains_key(new_parent_index));
            new_index_to_node.insert(*new_parent_index, child.clone());
            *new_parent_index += 1;
            child
        }
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
