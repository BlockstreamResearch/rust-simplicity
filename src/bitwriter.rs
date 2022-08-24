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

use std::io;

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
