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
//! Functionality to encode Simplicity programs. These programs
//! are encoded bitwise rather than bytewise, so given a hex dump of a
//! program it is not generally possible to read it visually the way you
//! can with Bitcoin Script.

use std::{io, mem};

use crate::extension;
use crate::extension::Jet as ExtNode;
use crate::Term;

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
            for byte in cmr.as_ref() {
                len += writer.write_u8(*byte, 8)?;
            }
            Ok(len)
        }
        Term::Witness(..) => writer.write_u8(7, 4),
        Term::Ext(ref b) => extension::Jet::encode(b, writer),
        Term::Jet(ref j) => j.encode(writer),
    }
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
