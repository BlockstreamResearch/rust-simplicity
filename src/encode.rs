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

use std::{io, mem};

use crate::jet::Application;
use crate::Value;
use crate::{Term, UntypedProgram};

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

/// Encode an untyped Simplicity program to bits.
///
/// Returns the number of written bits.
pub fn encode_program_no_witness<W: io::Write, App: Application>(
    program: &UntypedProgram<(), App>,
    w: &mut BitWriter<W>,
) -> io::Result<usize> {
    let start_n = w.n_total_written();

    encode_natural(program.len(), w)?;

    for term in program.iter() {
        encode_node(term, w)?;
    }

    Ok(w.n_total_written() - start_n)
}

/// Encode witness data to bits.
///
/// Returns the number of written bits.
pub fn encode_witness<W: io::Write>(witness: &[Value], w: &mut BitWriter<W>) -> io::Result<usize> {
    let start_n = w.n_total_written();

    if witness.is_empty() {
        w.write_bit(false)?;
    } else {
        w.write_bit(true)?;

        let mut payload = Vec::new();
        let mut payload_w = BitWriter::from(&mut payload);
        let mut bit_len = 0;

        for value in witness {
            bit_len += encode_value(value, &mut payload_w)?;
        }

        encode_natural(bit_len, w)?;

        for value in witness {
            encode_value(value, w)?;
        }
    }

    Ok(w.n_total_written() - start_n)
}

/// Encode a value to bits.
///
/// Returns the number of written bits.
pub fn encode_value<W: io::Write>(value: &Value, w: &mut BitWriter<W>) -> io::Result<usize> {
    let n_start = w.n_total_written();

    match *value {
        Value::Unit => {}
        Value::SumL(ref l) => {
            w.write_bit(false)?;
            encode_value(l, w)?;
        }
        Value::SumR(ref r) => {
            w.write_bit(true)?;
            encode_value(r, w)?;
        }
        Value::Prod(ref l, ref r) => {
            encode_value(l, w)?;
            encode_value(r, w)?;
        }
    }

    Ok(w.n_total_written() - n_start)
}

/// Encode an untyped Simplicity term to bits.
fn encode_node<W: io::Write, Wit, App: Application>(
    node: &Term<Wit, App>,
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
        Term::Fail(..) => unimplemented!(),
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
    use crate::core::types;
    use crate::core::types::TypedProgram;
    use crate::jet::application::Core;
    use crate::{decode, jet};

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
        let program: TypedProgram<(), Core> = types::type_check(UntypedProgram(vec![
            Term::Witness(()),
            Term::Jet(&jet::core::ADD32),
            Term::Comp(2, 1),
        ]))
        .expect("type checking");

        for n in 1..1000 {
            let witness = vec![Value::u64(n)];

            let mut sink = Vec::<u8>::new();
            let mut w = BitWriter::from(&mut sink);
            encode_witness(&witness, &mut w).expect("encoding to vector");
            w.flush_all().expect("flushing");

            let decoded_witness =
                decode::decode_witness(&program, &mut BitIter::from(sink.into_iter()))
                    .expect("decoding from vector");
            assert_eq!(witness, decoded_witness);
        }
    }
}
