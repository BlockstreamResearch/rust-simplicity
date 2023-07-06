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

use crate::dag::{Dag, DagLike, PostOrderIterItem, SharingTracker};
use crate::jet::Jet;
use crate::node::{self, Disconnectable};
use crate::{BitWriter, Cmr, Value};

use std::collections::{hash_map::Entry, HashMap};
use std::sync::Arc;
use std::{hash, io, mem};

#[derive(Copy, Clone)]
enum EncodeNode<'n, N: node::Marker> {
    Node(&'n node::Node<N>),
    Hidden(Cmr),
}

impl<'n, N: node::Marker> Disconnectable<EncodeNode<'n, N>> for EncodeNode<'n, N> {
    fn disconnect_dag_arc(self, other: Arc<EncodeNode<'n, N>>) -> Dag<Arc<EncodeNode<'n, N>>> {
        Dag::Binary(other, Arc::new(self))
    }

    fn disconnect_dag_ref<'s>(
        &'s self,
        other: &'s EncodeNode<'n, N>,
    ) -> Dag<&'s EncodeNode<'n, N>> {
        Dag::Binary(other, self)
    }
}

impl<'n, N: node::Marker> DagLike for EncodeNode<'n, N> {
    type Node = Self;
    fn data(&self) -> &Self {
        self
    }

    fn as_dag_node(&self) -> Dag<Self> {
        let node = match *self {
            EncodeNode::Node(node) => node,
            EncodeNode::Hidden(..) => return Dag::Nullary,
        };
        match node.inner() {
            node::Inner::Unit
            | node::Inner::Iden
            | node::Inner::Fail(..)
            | node::Inner::Jet(..)
            | node::Inner::Word(..) => Dag::Nullary,
            node::Inner::InjL(sub)
            | node::Inner::InjR(sub)
            | node::Inner::Take(sub)
            | node::Inner::Drop(sub) => Dag::Unary(EncodeNode::Node(sub)),
            node::Inner::Comp(left, right)
            | node::Inner::Case(left, right)
            | node::Inner::Pair(left, right) => {
                Dag::Binary(EncodeNode::Node(left), EncodeNode::Node(right))
            }
            node::Inner::Disconnect(left, right) => {
                right.disconnect_dag_ref(left).map(EncodeNode::Node)
            }
            node::Inner::AssertL(left, rcmr) => {
                Dag::Binary(EncodeNode::Node(left), EncodeNode::Hidden(*rcmr))
            }
            node::Inner::AssertR(lcmr, right) => {
                Dag::Binary(EncodeNode::Hidden(*lcmr), EncodeNode::Node(right))
            }
            node::Inner::Witness(..) => Dag::Nullary,
        }
    }
}

#[derive(Clone)]
enum EncodeId<N: node::Marker> {
    Node(N::SharingId),
    Hidden(Cmr),
}

// Have to implement these manually because Rust sucks.
impl<N: node::Marker> PartialEq for EncodeId<N> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EncodeId::Node(left), EncodeId::Node(right)) => left == right,
            (EncodeId::Hidden(left), EncodeId::Hidden(right)) => left == right,
            _ => false,
        }
    }
}

impl<N: node::Marker> Eq for EncodeId<N> {}

impl<N: node::Marker> hash::Hash for EncodeId<N> {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        match self {
            EncodeId::Node(id) => {
                hash::Hash::hash(&false, hasher);
                hash::Hash::hash(id, hasher);
            }
            EncodeId::Hidden(cmr) => {
                hash::Hash::hash(&true, hasher);
                hash::Hash::hash(cmr, hasher);
            }
        }
    }
}

/// Shares nodes based on IMR, *except* for Hidden nodes, which are identified
/// solely by the hash they contain
#[derive(Clone)]
pub struct EncodeSharing<N: node::Marker> {
    map: HashMap<EncodeId<N>, usize>,
}

// Annoyingly we have to implement Default by hand
impl<N: node::Marker> Default for EncodeSharing<N> {
    fn default() -> Self {
        EncodeSharing {
            map: HashMap::default(),
        }
    }
}

impl<'n, N: node::Marker> SharingTracker<EncodeNode<'n, N>> for EncodeSharing<N> {
    fn record(&mut self, d: &EncodeNode<N>, index: usize) -> Option<usize> {
        let id = match d {
            EncodeNode::Node(n) => EncodeId::Node(n.sharing_id()?),
            EncodeNode::Hidden(cmr) => EncodeId::Hidden(*cmr),
        };

        match self.map.entry(id) {
            Entry::Occupied(occ) => Some(*occ.get()),
            Entry::Vacant(vac) => {
                vac.insert(index);
                None
            }
        }
    }

    fn seen_before(&self, d: &EncodeNode<N>) -> Option<usize> {
        let id = match d {
            EncodeNode::Node(n) => EncodeId::Node(n.sharing_id()?),
            EncodeNode::Hidden(cmr) => EncodeId::Hidden(*cmr),
        };

        self.map.get(&id).copied()
    }
}

/// Encode a Simplicity program to bits, without witness data.
///
/// Returns the number of written bits.
pub fn encode_program<W: io::Write, N: node::Marker>(
    program: &node::Node<N>,
    w: &mut BitWriter<W>,
) -> io::Result<usize> {
    let iter = EncodeNode::Node(program).post_order_iter::<EncodeSharing<N>>();

    let len = iter.clone().count();
    let start_n = w.n_total_written();
    encode_natural(len, w)?;

    for node in iter {
        encode_node(node, w)?;
    }

    Ok(w.n_total_written() - start_n)
}

/// Encode a node to bits.
fn encode_node<W: io::Write, N: node::Marker>(
    data: PostOrderIterItem<EncodeNode<N>>,
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

            match node.inner() {
                node::Inner::Comp(_, _) => {
                    w.write_bits_be(0x00000, 5)?;
                }
                node::Inner::Case(_, _)
                | node::Inner::AssertL(_, _)
                | node::Inner::AssertR(_, _) => {
                    w.write_bits_be(0b00001, 5)?;
                }
                node::Inner::Pair(_, _) => {
                    w.write_bits_be(0b00010, 5)?;
                }
                node::Inner::Disconnect(_, _) => {
                    w.write_bits_be(0b00011, 5)?;
                }
                _ => unreachable!(),
            }

            encode_natural(i, w)?;
            encode_natural(j, w)?;
        } else {
            match node.inner() {
                node::Inner::InjL(_) => {
                    w.write_bits_be(0b00100, 5)?;
                }
                node::Inner::InjR(_) => {
                    w.write_bits_be(0b00101, 5)?;
                }
                node::Inner::Take(_) => {
                    w.write_bits_be(0b00110, 5)?;
                }
                node::Inner::Drop(_) => {
                    w.write_bits_be(0b00111, 5)?;
                }
                _ => unreachable!(),
            };

            encode_natural(i, w)?;
        }
    } else {
        match node.inner() {
            node::Inner::Iden => {
                w.write_bits_be(0b01000, 5)?;
            }
            node::Inner::Unit => {
                w.write_bits_be(0b01001, 5)?;
            }
            node::Inner::Fail(entropy) => {
                w.write_bits_be(0b01010, 5)?;
                encode_hash(entropy.as_ref(), w)?;
            }
            node::Inner::Witness(_) => {
                w.write_bits_be(0b0111, 4)?;
            }
            node::Inner::Jet(jet) => {
                w.write_bit(true)?; // jet or word
                w.write_bit(true)?; // jet
                jet.encode(w)?;
            }
            node::Inner::Word(val) => {
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
        bit_len += value.len();
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
        w.write_bits_be(u64::from(*byte), 8)?;
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

    use crate::decode;
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
}
