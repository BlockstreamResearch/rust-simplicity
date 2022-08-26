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

use crate::core::Value;
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

/*
/// Encode an untyped Simplicity program to bits.
///
/// Returns the number of written bits.
pub fn encode_program_no_witness<'a, W: io::Write, Witness, App: Application, I>(
    program: I,
    w: &mut BitWriter<W>,
) -> io::Result<usize>
where
    Witness: 'a,
    I: ExactSizeIterator<Item = &'a Term<Witness, App>>,
{
    let start_n = w.n_total_written();

    encode_natural(program.len(), w)?;

    for term in program {
        encode_node(term, w)?;
    }

    Ok(w.n_total_written() - start_n)
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
*/

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

/*
/// Encode an untyped Simplicity term to bits.
fn encode_node<W: io::Write, Witness, App: Application>(
    node: &Term<Witness, App>,
    w: &mut BitWriter<W>,
) -> io::Result<()> {
    match *node {
        Term::Comp(i, j) => {
            w.write_bits_be(0, 5)?;
            encode_natural(i, w)?;
            encode_natural(j, w)?;
        }
        Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) => {
            w.write_bits_be(1, 5)?;
            encode_natural(i, w)?;
            encode_natural(j, w)?;
        }
        Term::Pair(i, j) => {
            w.write_bits_be(2, 5)?;
            encode_natural(i, w)?;
            encode_natural(j, w)?;
        }
        Term::Disconnect(i, j) => {
            w.write_bits_be(3, 5)?;
            encode_natural(i, w)?;
            encode_natural(j, w)?;
        }
        Term::InjL(i) => {
            w.write_bits_be(4, 5)?;
            encode_natural(i, w)?;
        }
        Term::InjR(i) => {
            w.write_bits_be(5, 5)?;
            encode_natural(i, w)?;
        }
        Term::Take(i) => {
            w.write_bits_be(6, 5)?;
            encode_natural(i, w)?;
        }
        Term::Drop(i) => {
            w.write_bits_be(7, 5)?;
            encode_natural(i, w)?;
        }
        Term::Iden => {
            w.write_bits_be(8, 5)?;
        }
        Term::Unit => {
            w.write_bits_be(9, 5)?;
        }
        Term::Fail(left, right) => {
            w.write_bits_be(10, 5)?;
            encode_hash(left.as_ref(), w)?;
            encode_hash(right.as_ref(), w)?;
        }
        Term::Hidden(cmr) => {
            w.write_bits_be(6, 4)?;
            encode_hash(cmr.as_ref(), w)?;
        }
        Term::Witness(..) => {
            w.write_bits_be(7, 4)?;
        }
        Term::Jet(jet) => {
            App::encode_jet(jet, w)?;
        }
    };

    Ok(())
}
*/

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
    use crate::decode;

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

    /*
    #[test]
    fn encode_decode_witness() {
        let program: TypedProgram<(), Core> = UntypedProgram(vec![
            Term::Witness(()),
            Term::Jet(&jet::core::ADD32),
            Term::Comp(2, 1),
        ])
        .type_check()
        .expect("type checking");

        for n in 1..1000 {
            let witness = vec![Value::u64(n)];
            let it = witness.iter();

            let mut sink = Vec::<u8>::new();
            let mut w = BitWriter::from(&mut sink);
            encode_witness(it, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");

            let decoded_witness =
                decode::decode_witness(&program, &mut BitIter::from(sink.into_iter()))
                    .expect("decoding from vector");
            assert_eq!(witness, decoded_witness);
        }
    }
    */
}
