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

use crate::core::iter::{DagIterable, PostOrderIter};
use crate::core::node::{NodeInner, RefWrapper};
use crate::core::Value;
use crate::jet::Application;
use crate::sharing;
use std::collections::HashMap;
use std::{io, mem};

/// Bitwise writer formed by wrapping a bytewise [`io::Write`].
/// Bits are written in big-endian order.
/// Bytes are filled with zeroes for padding.
pub struct BitWriter<W: io::Write> {
    /// Byte writer
    w: W,
    /// Current byte that contains current bits, yet to be written out
    cache: u8,
    /// Number of current bits
    cache_len: usize,
    /// Total number of written bits
    total_written: usize,
}

impl<W: io::Write> From<W> for BitWriter<W> {
    fn from(w: W) -> Self {
        BitWriter {
            w,
            cache: 0,
            cache_len: 0,
            total_written: 0,
        }
    }
}

impl<W: io::Write> io::Write for BitWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.w.write(buf)
    }

    /// Does **not** write out cached bits
    /// (i.e. bits written since the last byte boundary).
    /// To do this you must call [`BitWriter::flush_all()`].
    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }
}

impl<W: io::Write> BitWriter<W> {
    /// Create a bitwise writer from a bytewise one.
    /// Equivalent to using [`From`].
    pub fn new(w: W) -> BitWriter<W> {
        BitWriter::from(w)
    }

    /// Write a single bit.
    pub fn write_bit(&mut self, b: bool) -> io::Result<()> {
        if self.cache_len < 8 {
            self.cache_len += 1;
            self.total_written += 1;
            if b {
                self.cache |= 1 << (8 - self.cache_len);
            }
            Ok(())
        } else {
            self.w.write_all(&[self.cache])?;
            self.cache_len = 0;
            self.cache = 0;
            self.write_bit(b)
        }
    }

    /// Write out all cached bits.
    /// This may write up to two bytes and flushes the underlying [`io::Write`].
    pub fn flush_all(&mut self) -> io::Result<()> {
        self.w.write_all(&[self.cache])?;
        self.cache_len = 0;
        self.cache = 0;

        io::Write::flush(&mut self.w)
    }

    /// Return total number of written bits.
    pub fn n_total_written(&self) -> usize {
        self.total_written
    }

    /// Write up to 64 bits in big-endian order.
    /// The first `len` many _least significant_ bits from `n` are written.
    ///
    /// Returns the number of written bits.
    pub fn write_bits_be(&mut self, n: u64, len: usize) -> io::Result<usize> {
        for i in 0..len {
            self.write_bit(n & (1 << (len - i - 1)) != 0)?;
        }
        Ok(len)
    }
}

/// Encode a Simplicity program to bits, without witness data.
///
/// Returns the number of written bits.
pub fn encode_program<W: io::Write, Witness, App: Application>(
    program: PostOrderIter<RefWrapper<Witness, App>>,
    w: &mut BitWriter<W>,
) -> io::Result<usize> {
    let (node_to_index, len) = sharing::compute_maximal_sharing(program.clone());

    let start_n = w.n_total_written();
    encode_natural(len, w)?;

    let mut index = 0;
    for node in program {
        if node_to_index.get(&node).unwrap() != &index {
            continue;
        }

        encode_node(node, index, &node_to_index, w)?;
        index += 1;
    }

    Ok(w.n_total_written() - start_n)
}

/// Encode a node to bits.
fn encode_node<W: io::Write, Witness, App: Application>(
    node: RefWrapper<Witness, App>,
    index: usize,
    node_to_index: &HashMap<RefWrapper<Witness, App>, usize>,
    w: &mut BitWriter<W>,
) -> io::Result<()> {
    if let Some(left) = node.get_left() {
        let i_abs = *node_to_index.get(&left).unwrap();
        debug_assert!(i_abs < index);
        let i = index - i_abs;

        if let Some(right) = node.get_right() {
            let j_abs = *node_to_index.get(&right).unwrap();
            debug_assert!(j_abs < index);
            let j = index - j_abs;

            match &node.0.inner {
                NodeInner::Comp(_, _) => {
                    w.write_bits_be(0, 5)?;
                }
                NodeInner::Case(_, _) | NodeInner::AssertL(_, _) | NodeInner::AssertR(_, _) => {
                    w.write_bits_be(1, 5)?;
                }
                NodeInner::Pair(_, _) => {
                    w.write_bits_be(2, 5)?;
                }
                NodeInner::Disconnect(_, _) => {
                    w.write_bits_be(3, 5)?;
                }
                _ => unreachable!(),
            }

            encode_natural(i, w)?;
            encode_natural(j, w)?;
        } else {
            match &node.0.inner {
                NodeInner::InjL(_) => {
                    w.write_bits_be(4, 5)?;
                }
                NodeInner::InjR(_) => {
                    w.write_bits_be(5, 5)?;
                }
                NodeInner::Take(_) => {
                    w.write_bits_be(6, 5)?;
                }
                NodeInner::Drop(_) => {
                    w.write_bits_be(7, 5)?;
                }
                _ => unreachable!(),
            };

            encode_natural(i, w)?;
        }
    } else {
        match &node.0.inner {
            NodeInner::Iden => {
                w.write_bits_be(8, 5)?;
            }
            NodeInner::Unit => {
                w.write_bits_be(9, 5)?;
            }
            NodeInner::Fail(hl, hr) => {
                w.write_bits_be(10, 5)?;
                encode_hash(hl.as_ref(), w)?;
                encode_hash(hr.as_ref(), w)?;
            }
            NodeInner::Hidden(h) => {
                w.write_bits_be(6, 4)?;
                encode_hash(h.as_ref(), w)?;
            }
            NodeInner::Witness(_) => {
                w.write_bits_be(7, 4)?;
            }
            NodeInner::Jet(jet) => {
                App::encode_jet(jet, w)?;
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
fn get_bit_len(value: &Value) -> usize {
    match value {
        Value::Unit => 0,
        Value::SumL(left) => 1 + get_bit_len(left),
        Value::SumR(right) => 1 + get_bit_len(right),
        Value::Prod(left, right) => get_bit_len(left) + get_bit_len(right),
    }
}

/// Encode a value to bits.
///
/// Returns the number of written bits.
pub fn encode_value<W: io::Write>(value: &Value, w: &mut BitWriter<W>) -> io::Result<usize> {
    let n_start = w.n_total_written();

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

    Ok(w.n_total_written() - n_start)
}

/// Encode a hash to bits.
fn encode_hash<W: io::Write>(h: &[u8], w: &mut BitWriter<W>) -> io::Result<()> {
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
    use crate::bititer::BitIter;
    use crate::core::types::Type;
    use crate::decode;
    use crate::decode::{WitnessDecoder, WitnessIterator};

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
        let pow2s = Type::powers_of_two();

        for n in 1..1000 {
            let witness = vec![Value::u64(n)];
            let it = witness.iter();

            let mut sink = Vec::<u8>::new();
            let mut w = BitWriter::from(&mut sink);
            encode_witness(it, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");

            let mut bits = BitIter::from(sink.into_iter());
            let mut decoder = WitnessDecoder::new(&mut bits).expect("decoding from vector");
            assert_eq!(witness[0], decoder.next(&pow2s[6]).unwrap());
        }
    }
}
