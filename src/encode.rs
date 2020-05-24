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

use std::{mem, io};

use bititer::BitIter;
use extension::bitcoin;
use {Error, Node};
use cmr;

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
            self.w.write(&[self.cache])?;
            self.cache_len = 0;
            self.cache = 0;
            self.write_bit(b)
        }
    }

    /// Write out all cached bits, including those written since the last
    /// byte boundary. (0s will be written after the actual data to pad out
    /// to the next byte boundary). Then flushes the underlying `io::Write`.
    fn flush_all(&mut self) -> io::Result<()> {
        self.w.write(&[self.cache])?;
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
pub fn decode_node_no_witness<I: Iterator<Item = u8>>(
    idx: usize,
    iter: &mut BitIter<I>,
) -> Result<Node<()>, Error> {
    match iter.next() {
        None => Err(Error::EndOfStream),
        Some(true) => {
            match iter.next() {
                None => Err(Error::EndOfStream),
                Some(false) => Ok(Node::Bitcoin(bitcoin::decode_node_no_witness(iter)?)),
                Some(true) => Err(Error::ParseError("invalid parse 11")),
            }
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
                (0, 0) => Ok(Node::Comp(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                // FIXME `Case` should check for asserts and reject if both children are hidden
                (0, 1) => Ok(Node::Case(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (0, 2) => Ok(Node::Pair(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (0, 3) => Ok(Node::Disconnect(
                    idx.checked_sub(decode_natural(&mut *iter)?).ok_or(Error::BadIndex)?,
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 0) => Ok(Node::InjL(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 1) => Ok(Node::InjR(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 2) => Ok(Node::Take(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (1, 3) => Ok(Node::Drop(
                    idx.checked_sub(decode_natural(iter)?).ok_or(Error::BadIndex)?,
                )),
                (2, 0) => Ok(Node::Iden),
                (2, 1) => Ok(Node::Unit),
                // FIXME Russell's code just rejects `Fail`
                (2, 2) => {
                    let mut h1 = [0; 32];
                    let mut h2 = [0; 32];
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h1[i] |= 1 << (7 - b),
                                Some(false) => {}
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h2[i] |= 1 << (7 - b),
                                Some(false) => {}
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    Ok(Node::Fail(h1, h2))
                },
                (2, 3) => Err(Error::ParseError("01011 (stop code)")),
                (3, 0) => {
                    let mut h = [0; 32];
                    for i in 0..32 {
                        for b in 0..8 {
                            match iter.next() {
                                Some(true) => h[i] |= 1 << (7 - b),
                                Some(false) => {}
                                None => return Err(Error::EndOfStream),
                            };
                        }
                    }
                    Ok(Node::Hidden(cmr::Cmr::from(h)))
                },
                (3, 1) => Ok(Node::Witness(())),
                (_, _) => unreachable!("we read only so many bits"),
            }
        }
    }
}

pub fn decode_program_no_witness<I: Iterator<Item = u8>>(
    iter: &mut BitIter<I>,
) -> Result<Vec<Node<()>>, Error> {
    let prog_len = decode_natural(&mut *iter)?;

    let mut program = Vec::with_capacity(prog_len);
    for i in 0..prog_len {
        program.push(decode_node_no_witness(i, iter)?);
    }

    Ok(program)
}

/// Encode a natural number according to section 7.2.1 of the Simplicity tech
/// report. Returns the length of the written number, in bits
pub fn encode_natural<W: BitWrite>(
    n: usize,
    writer: &mut W,
) -> io::Result<usize> {
    assert_ne!(n, 0); // Cannot encode zero
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

        Ok(writer.n_written())
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
            assert_eq!(
                decode_natural(truncated.into_iter()),
                Err(Error::EndOfStream),
            );

            let len = vec.len();

            // Encode/decode bitwise
            let mut encode = Vec::<bool>::new();
            encode_natural(target, &mut encode)
                .expect("encoding to a Vec");
            assert_eq!(encode, vec);
            let decode = decode_natural(vec.into_iter()).unwrap();
            assert_eq!(target, decode);

            // Encode/decode bytewise
            let mut w = BitWriter::new(Vec::<u8>::new());
            encode_natural(target, &mut w)
                .expect("encoding to a Vec");
            w.flush_all().expect("flushing");
            assert_eq!(w.n_written(), len);
            let r = BitIter::new(w.into_inner().into_iter());
            let decode = decode_natural(r).unwrap();

            assert_eq!(target, decode);
        }
    }
}

