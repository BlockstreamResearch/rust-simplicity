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

//! # Encoding/Decoding
//!
//! Functionality to encode or decode Simplicity programs. These programs
//! are encoded bitwise rather than bytewise, so given a hex dump of a
//! program it is not generally possible to read it visually the way you
//! can with Bitcoin Script.
//!

use std::{io, mem};

use crate::bititer::BitIter;
use crate::cmr;

use crate::extension;
use crate::extension::Jet as ExtNode;
use crate::{Error, Term};

use crate::core::term::UnTypedProg;

/// Trait for writing individual bits to some sink
pub trait BitWrite {
    /// Write a single bit to the writer
    fn write_bit(&mut self, b: bool) -> io::Result<()>;

    /// Write out all cached bits, including those written since the last
    /// byte boundary. (0s will be written after the actual data to pad out
    /// to the next byte boundary). Then flushes the underlying `io::Write`
    /// if one exists.
    fn flush_all(&mut self) -> io::Result<()>;

    /// Total number of bits written to this object (not necessarily the number
    /// of bits written to the underlying byte-oriented sink, which in general
    /// will be less)
    fn n_written(&self) -> usize;

    /// Write several bytes at once, up to 8. If `len` is less than 8, the
    /// data is read in big-endian order from the least significant bits.
    /// On success, returns `len`.
    fn write_u8(&mut self, n: u8, len: usize) -> io::Result<usize> {
        for i in 0..len {
            self.write_bit(n & (1 << (len - i - 1)) != 0)?;
        }
        Ok(len)
    }
}

/// Wrapper around `io::Write` to enable writing individual bits to a bytestream
pub struct BitWriter<W: io::Write> {
    w: W,
    cache: u8,
    cache_len: usize,
    total_written: usize,
}

impl<W: io::Write> BitWriter<W> {
    /// Create a new `BitWriter` from an underlying `Write`
    pub fn new(w: W) -> BitWriter<W> {
        BitWriter {
            w: w,
            cache: 0,
            cache_len: 0,
            total_written: 0,
        }
    }

    /// Recover the underlying `Write`
    pub fn into_inner(self) -> W {
        self.w
    }
}

impl<W: io::Write> io::Write for BitWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.w.write(buf)
    }

    /// Flush the underlying `io::Write` -- does **not** write out cached
    /// bits (i.e. bits written since the last byte boundary). To do this
    /// you must call `flush_all`.
    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }
}

impl<W: io::Write> BitWrite for BitWriter<W> {
    fn write_bit(&mut self, b: bool) -> io::Result<()> {
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

    /// Write out all cached bits, including those written since the last
    /// byte boundary. (0s will be written after the actual data to pad out
    /// to the next byte boundary). Then flushes the underlying `io::Write`.
    fn flush_all(&mut self) -> io::Result<()> {
        self.w.write_all(&[self.cache])?;
        self.cache_len = 0;

        io::Write::flush(&mut self.w)
    }

    /// Total number of bits written to this iterator (not necessarily the number
    /// of bits written to the underlying iterator, which in general will be less)
    fn n_written(&self) -> usize {
        self.total_written
    }
}

impl BitWrite for Vec<bool> {
    fn write_bit(&mut self, b: bool) -> io::Result<()> {
        self.push(b);
        Ok(())
    }

    fn flush_all(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn n_written(&self) -> usize {
        self.len()
    }
}

impl<'a, B: BitWrite> BitWrite for &'a mut B {
    fn write_bit(&mut self, b: bool) -> io::Result<()> {
        (**self).write_bit(b)
    }

    fn flush_all(&mut self) -> io::Result<()> {
        (**self).flush_all()
    }

    fn n_written(&self) -> usize {
        (**self).n_written()
    }
}

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
pub fn decode_node_no_witness<I: Iterator<Item = u8>, Ext: extension::Jet>(
    idx: usize,
    iter: &mut BitIter<I>,
) -> Result<Term<(), Ext>, Error> {
    match iter.next() {
        None => Err(Error::EndOfStream),
        Some(true) => match iter.next() {
            None => Err(Error::EndOfStream),
            Some(false) => Ok(Term::Ext(extension::Jet::decode(iter)?)),
            Some(true) => Ok(Term::Jet(extension::Jet::decode(iter)?)),
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
                (0, 0) => Ok(Term::Comp(
                    decode_natural(&mut *iter, Some(idx))?,
                    decode_natural(iter, Some(idx))?,
                )),
                // FIXME `Case` should check for asserts and reject if both children are hidden
                (0, 1) => Ok(Term::Case(
                    decode_natural(&mut *iter, Some(idx))?,
                    decode_natural(iter, Some(idx))?,
                )),
                (0, 2) => Ok(Term::Pair(
                    decode_natural(&mut *iter, Some(idx))?,
                    decode_natural(iter, Some(idx))?,
                )),
                (0, 3) => Ok(Term::Disconnect(
                    decode_natural(&mut *iter, Some(idx))?,
                    decode_natural(iter, Some(idx))?,
                )),
                (1, 0) => Ok(Term::InjL(decode_natural(iter, Some(idx))?)),
                (1, 1) => Ok(Term::InjR(decode_natural(iter, Some(idx))?)),
                (1, 2) => Ok(Term::Take(decode_natural(iter, Some(idx))?)),
                (1, 3) => Ok(Term::Drop(decode_natural(iter, Some(idx))?)),
                (2, 0) => Ok(Term::Iden),
                (2, 1) => Ok(Term::Unit),
                (2, 2) => Err(Error::ParseError("01010 (fail node)")),
                (2, 3) => Err(Error::ParseError("01011 (stop code)")),
                (3, 0) => {
                    let mut h = [0; 32];
                    for byte in &mut h {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => *byte |= 1 << (7 - b),
                                Some(false) => {}
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    Ok(Term::Hidden(cmr::Cmr::from(h)))
                }
                (3, 1) => Ok(Term::Witness(())),
                (_, _) => unreachable!("we read only so many bits"),
            }
        }
    }
}

pub fn encode_node_no_witness<T, W: BitWrite, Ext: extension::Jet>(
    node: &Term<T, Ext>,
    writer: &mut W,
) -> io::Result<usize> {
    match *node {
        Term::Comp(i, j) => {
            let ret = writer.write_u8(0, 5)?
                + encode_natural(i, &mut *writer)?
                + encode_natural(j, &mut *writer)?;
            Ok(ret)
        }
        Term::Case(i, j) | Term::AssertL(i, j) | Term::AssertR(i, j) => {
            let ret = writer.write_u8(1, 5)?
                + encode_natural(i, &mut *writer)?
                + encode_natural(j, &mut *writer)?;
            Ok(ret)
        }
        Term::Pair(i, j) => {
            let ret = writer.write_u8(2, 5)?
                + encode_natural(i, &mut *writer)?
                + encode_natural(j, &mut *writer)?;
            Ok(ret)
        }
        Term::Disconnect(i, j) => {
            let ret = writer.write_u8(3, 5)?
                + encode_natural(i, &mut *writer)?
                + encode_natural(j, &mut *writer)?;
            Ok(ret)
        }
        Term::InjL(i) => Ok(writer.write_u8(4, 5)? + encode_natural(i, &mut *writer)?),
        Term::InjR(i) => Ok(writer.write_u8(5, 5)? + encode_natural(i, &mut *writer)?),
        Term::Take(i) => Ok(writer.write_u8(6, 5)? + encode_natural(i, &mut *writer)?),
        Term::Drop(i) => Ok(writer.write_u8(7, 5)? + encode_natural(i, &mut *writer)?),
        Term::Iden => writer.write_u8(8, 5),
        Term::Unit => writer.write_u8(9, 5),
        Term::Fail(..) => unimplemented!(),
        Term::Hidden(cmr) => {
            let mut len = writer.write_u8(6, 4)?;
            for byte in &cmr[..] {
                len += writer.write_u8(*byte, 8)?;
            }
            Ok(len)
        }
        Term::Witness(..) => writer.write_u8(7, 4),
        Term::Ext(ref b) => extension::Jet::encode(b, writer),
        Term::Jet(ref j) => j.encode(writer),
    }
}

pub fn decode_program_no_witness<I: Iterator<Item = u8>, Ext: extension::Jet>(
    iter: &mut BitIter<I>,
) -> Result<UnTypedProg<(), Ext>, Error> {
    let prog_len = decode_natural(&mut *iter, None)?;

    // FIXME make this a reasonable limit
    if prog_len > 1_000_000 {
        return Err(Error::TooManyNodes(prog_len));
    }

    let mut program = Vec::with_capacity(prog_len);
    for idx in 0..prog_len {
        let node = decode_node_no_witness(idx, iter)?;
        let node = if let Term::Case(i, j) = node {
            match (&program[idx - i], &program[idx - j]) {
                (Term::Hidden(..), Term::Hidden(..)) => {
                    return Err(Error::CaseMultipleHiddenChildren)
                }
                (Term::Hidden(..), _) => Term::AssertR(i, j),
                (_, Term::Hidden(..)) => Term::AssertL(i, j),
                _ => Term::Case(i, j),
            }
        } else {
            node
        };
        program.push(node);
    }

    Ok(UnTypedProg(program))
}

/// Encode a natural number according to section 7.2.1 of the Simplicity tech
/// report. Returns the length of the written number, in bits
pub fn encode_natural<W: BitWrite>(n: usize, writer: &mut W) -> io::Result<usize> {
    assert_ne!(n, 0); // Cannot encode zero
    let n_start = writer.n_written();
    let len = 8 * mem::size_of::<usize>() - n.leading_zeros() as usize - 1;

    if len == 0 {
        writer.write_bit(false)?;
        Ok(1)
    } else {
        writer.write_bit(true)?;
        encode_natural(len, &mut *writer)?;

        for i in 0..len {
            writer.write_bit(n & (1 << (len - i - 1)) != 0)?;
        }

        Ok(writer.n_written() - n_start)
    }
}

/// Decode a natural number according to section 7.2.1
/// of the Simplicity whitepaper.
/// Optionally provide a bound for the value being decoded.
/// If the value is strictly greater than bound, this function
/// returns an error
pub fn decode_natural<BitStream: Iterator<Item = bool>>(
    mut iter: BitStream,
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
            assert_matches!(
                decode_natural(truncated.into_iter(), None),
                Err(Error::EndOfStream)
            );

            let len = vec.len();

            // Encode/decode bitwise
            let mut encode = Vec::<bool>::new();
            encode_natural(target, &mut encode).expect("encoding to a Vec");
            assert_eq!(encode, vec);
            let decode = decode_natural(vec.into_iter(), None).unwrap();
            assert_eq!(target, decode);

            // Encode/decode bytewise
            let mut w = BitWriter::new(Vec::<u8>::new());
            encode_natural(target, &mut w).expect("encoding to a Vec");
            w.flush_all().expect("flushing");
            assert_eq!(w.n_written(), len);
            let r = BitIter::new(w.into_inner().into_iter());
            let decode = decode_natural(r, None).unwrap();

            assert_eq!(target, decode);
        }
    }
}
