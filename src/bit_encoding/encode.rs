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

//! # Encoding
//!
//! Functionality to encode Simplicity programs.
//! These programs are encoded bitwise rather than bytewise,
//! so given a hex dump of a program it is not generally possible
//! to read it visually the way you can with Bitcoin Script.

use crate::core::redeem::RedeemNodeInner;
use crate::core::Value;
use crate::dag::{Dag, DagLike, PostOrderIterItem, SharingTracker};
use crate::jet::Jet;
use crate::{BitWriter, Cmr, Imr, RedeemNode};
use std::collections::HashMap;
use std::{io, mem};

#[derive(Copy, Clone)]
enum EncodeNode<'n, J: Jet> {
    Node(&'n RedeemNode<J>),
    Hidden(Cmr),
}

impl<'n, J: Jet> DagLike for EncodeNode<'n, J> {
    type Node = Self;
    fn data(&self) -> &Self {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        let node = match *self {
            EncodeNode::Node(node) => node,
            EncodeNode::Hidden(..) => return Dag::Hidden,
        };
        match &node.inner {
            RedeemNodeInner::Unit => Dag::Unit,
            RedeemNodeInner::Iden => Dag::Iden,
            RedeemNodeInner::InjL(sub) => Dag::InjL(EncodeNode::Node(sub)),
            RedeemNodeInner::InjR(sub) => Dag::InjR(EncodeNode::Node(sub)),
            RedeemNodeInner::Take(sub) => Dag::Take(EncodeNode::Node(sub)),
            RedeemNodeInner::Drop(sub) => Dag::Drop(EncodeNode::Node(sub)),
            RedeemNodeInner::Comp(left, right) => {
                Dag::Comp(EncodeNode::Node(left), EncodeNode::Node(right))
            }
            RedeemNodeInner::Case(left, right) => {
                Dag::Case(EncodeNode::Node(left), EncodeNode::Node(right))
            }
            RedeemNodeInner::AssertL(left, rcmr) => {
                Dag::Case(EncodeNode::Node(left), EncodeNode::Hidden(*rcmr))
            }
            RedeemNodeInner::AssertR(lcmr, right) => {
                Dag::Case(EncodeNode::Hidden(*lcmr), EncodeNode::Node(right))
            }
            RedeemNodeInner::Pair(left, right) => {
                Dag::Pair(EncodeNode::Node(left), EncodeNode::Node(right))
            }
            RedeemNodeInner::Disconnect(left, right) => {
                Dag::Disconnect(EncodeNode::Node(left), EncodeNode::Node(right))
            }
            RedeemNodeInner::Witness(..) => Dag::Witness,
            RedeemNodeInner::Fail(..) => Dag::Fail,
            RedeemNodeInner::Jet(..) => Dag::Jet,
            RedeemNodeInner::Word(..) => Dag::Word,
        }
    }
}

/// Shares nodes based on IMR, *except* for Hidden nodes, which are identified
/// solely by the hash they contain
#[derive(Clone, Debug, Default)]
pub struct EncodeSharing {
    map: HashMap<Imr, usize>,
}

impl<'n, J: Jet> SharingTracker<EncodeNode<'n, J>> for EncodeSharing {
    fn record(&mut self, d: &EncodeNode<J>, index: usize, _: Option<usize>, _: Option<usize>) {
        let imr = match d {
            EncodeNode::Node(n) => n.imr,
            EncodeNode::Hidden(cmr) => (*cmr).into(),
        };
        self.map.insert(imr, index);
    }
    fn seen_before(&self, d: &EncodeNode<J>) -> Option<usize> {
        let imr = match d {
            EncodeNode::Node(n) => n.imr,
            EncodeNode::Hidden(cmr) => (*cmr).into(),
        };
        self.map.get(&imr).copied()
    }
}

/// Encode a Simplicity program to bits, without witness data.
///
/// Returns the number of written bits.
pub fn encode_program<W: io::Write, J: Jet>(
    program: &RedeemNode<J>,
    w: &mut BitWriter<W>,
) -> io::Result<usize> {
    let iter = EncodeNode::Node(program).post_order_iter::<EncodeSharing>();

    let len = iter.clone().count();
    let start_n = w.n_total_written();
    encode_natural(len, w)?;

    for node in iter {
        encode_node(node, w)?;
    }

    Ok(w.n_total_written() - start_n)
}

/// Encode a node to bits.
fn encode_node<W: io::Write, J: Jet>(
    data: PostOrderIterItem<EncodeNode<J>>,
    w: &mut BitWriter<W>,
) -> io::Result<()> {
    // Handle Hidden nodes specially
    let node = match data.node {
        EncodeNode::Node(node) => node,
        EncodeNode::Hidden(cmr) => {
            w.write_bits_be(0b0110, 4)?;
            encode_hash(cmr.as_ref(), w)?;
            return Ok(());
        }
    };

    if let Some(i_abs) = data.left_index {
        debug_assert!(i_abs < data.index);
        let i = data.index - i_abs;

        if let Some(j_abs) = data.right_index {
            debug_assert!(j_abs < data.index);
            let j = data.index - j_abs;

            match &node.inner {
                RedeemNodeInner::Comp(_, _) => {
                    w.write_bits_be(0x00000, 5)?;
                }
                RedeemNodeInner::Case(_, _)
                | RedeemNodeInner::AssertL(_, _)
                | RedeemNodeInner::AssertR(_, _) => {
                    w.write_bits_be(0b00001, 5)?;
                }
                RedeemNodeInner::Pair(_, _) => {
                    w.write_bits_be(0b00010, 5)?;
                }
                RedeemNodeInner::Disconnect(_, _) => {
                    w.write_bits_be(0b00011, 5)?;
                }
                _ => unreachable!(),
            }

            encode_natural(i, w)?;
            encode_natural(j, w)?;
        } else {
            match &node.inner {
                RedeemNodeInner::InjL(_) => {
                    w.write_bits_be(0b00100, 5)?;
                }
                RedeemNodeInner::InjR(_) => {
                    w.write_bits_be(0b00101, 5)?;
                }
                RedeemNodeInner::Take(_) => {
                    w.write_bits_be(0b00110, 5)?;
                }
                RedeemNodeInner::Drop(_) => {
                    w.write_bits_be(0b00111, 5)?;
                }
                _ => unreachable!(),
            };

            encode_natural(i, w)?;
        }
    } else {
        match &node.inner {
            RedeemNodeInner::Iden => {
                w.write_bits_be(0b01000, 5)?;
            }
            RedeemNodeInner::Unit => {
                w.write_bits_be(0b01001, 5)?;
            }
            RedeemNodeInner::Fail(hl, hr) => {
                w.write_bits_be(0b01010, 5)?;
                encode_hash(hl.as_ref(), w)?;
                encode_hash(hr.as_ref(), w)?;
            }
            RedeemNodeInner::Witness(_) => {
                w.write_bits_be(0b0111, 4)?;
            }
            RedeemNodeInner::Jet(jet) => {
                w.write_bit(true)?; // jet or word
                w.write_bit(true)?; // jet
                jet.encode(w)?;
            }
            RedeemNodeInner::Word(val) => {
                w.write_bit(true)?; // jet or word
                w.write_bit(false)?; // word
                assert_eq!(val.len().count_ones(), 1);
                let depth = val.len().trailing_zeros();
                encode_natural(1 + depth as usize, w)?;
                encode_value(val, w)?;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/// Encode witness data to bits.
///
/// Returns the number of written bits.
pub fn encode_witness<'a, W: io::Write, I>(witness: I, w: &mut BitWriter<W>) -> io::Result<usize>
where
    I: Iterator<Item = &'a Value> + Clone,
{
    let mut bit_len = 0;
    let start_n = w.n_total_written();

    for value in witness.clone() {
        bit_len += get_bit_len(value);
    }

    if bit_len == 0 {
        w.write_bit(false)?;
    } else {
        w.write_bit(true)?;
        encode_natural(bit_len, w)?;

        for value in witness {
            encode_value(value, w)?;
        }
    }

    Ok(w.n_total_written() - start_n)
}

/// Return the bit length of the given `value` when encoded.
pub fn get_bit_len(value: &Value) -> usize {
    match value {
        Value::Unit => 0,
        Value::SumL(left) => 1 + get_bit_len(left),
        Value::SumR(right) => 1 + get_bit_len(right),
        Value::Prod(left, right) => get_bit_len(left) + get_bit_len(right),
    }
}

/// Encode a value to bits.
pub fn encode_value<W: io::Write>(value: &Value, w: &mut BitWriter<W>) -> io::Result<()> {
    match value {
        Value::Unit => {}
        Value::SumL(left) => {
            w.write_bit(false)?;
            encode_value(left, w)?;
        }
        Value::SumR(right) => {
            w.write_bit(true)?;
            encode_value(right, w)?;
        }
        Value::Prod(left, right) => {
            encode_value(left, w)?;
            encode_value(right, w)?;
        }
    }

    Ok(())
}

/// Encode a hash to bits.
pub fn encode_hash<W: io::Write>(h: &[u8], w: &mut BitWriter<W>) -> io::Result<()> {
    for byte in h {
        w.write_bits_be(*byte as u64, 8)?;
    }

    Ok(())
}

/// Encode a natural number to bits.
pub fn encode_natural<W: io::Write>(n: usize, w: &mut BitWriter<W>) -> io::Result<()> {
    assert_ne!(n, 0); // Cannot encode zero
    let len = 8 * mem::size_of::<usize>() - n.leading_zeros() as usize - 1;

    if len == 0 {
        w.write_bit(false)?;
    } else {
        w.write_bit(true)?;
        encode_natural(len, w)?;
        w.write_bits_be(n as u64, len)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::iter::WitnessIterator;
    use crate::decode;
    use crate::decode::WitnessDecoder;
    use crate::BitIter;

    #[test]
    fn encode_decode_natural() {
        for n in 1..1000 {
            let mut sink = Vec::<u8>::new();
            let mut w = BitWriter::from(&mut sink);
            encode_natural(n, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");
            let m = decode::decode_natural(&mut BitIter::from(sink.into_iter()), None)
                .expect("decoding from vector");
            assert_eq!(n, m);
        }
    }

    #[test]
    fn encode_decode_witness() {
        let context = crate::Context::<crate::jet::Core>::new();

        for n in 1..1000 {
            let witness = vec![Value::u64(n)];
            let it = witness.iter();

            let mut sink = Vec::<u8>::new();
            let mut w = BitWriter::from(&mut sink);
            encode_witness(it, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");

            let mut bits = BitIter::from(sink.into_iter());
            let mut decoder = WitnessDecoder::new(&mut bits).expect("decoding from vector");
            assert_eq!(
                witness[0],
                decoder.next(&context.nth_power_of_2_final(6)).unwrap()
            );
        }
    }
}
