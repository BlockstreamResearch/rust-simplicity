// SPDX-License-Identifier: CC0-1.0

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
        for b in buf {
            for i in 0..8 {
                self.write_bit((b & (1 << (7 - i))) != 0)?;
            }
        }
        Ok(buf.len())
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
        if self.cache_len > 0 {
            self.w.write_all(&[self.cache])?;
            self.cache_len = 0;
            self.cache = 0;
        }

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

/// Write the result of a bit operation into a byte vector and return the vector.
///
/// I/O to a vector never fails.
pub fn write_to_vec<F>(f: F) -> Vec<u8>
where
    F: FnOnce(&mut BitWriter<&mut Vec<u8>>) -> io::Result<usize>,
{
    let mut bytes = Vec::new();
    let mut bits = BitWriter::new(&mut bytes);
    f(&mut bits).expect("I/O to vector never fails");
    bits.flush_all().expect("I/O to vector never fails");
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jet::Core;
    use crate::node::CoreConstructible;
    use crate::types;
    use crate::ConstructNode;
    use std::sync::Arc;

    #[test]
    fn vec() {
        let program = Arc::<ConstructNode<Core>>::unit(&types::Context::new());
        let _ = write_to_vec(|w| program.encode_without_witness(w));
    }

    #[test]
    fn empty_vec() {
        let vec = write_to_vec(|_| Ok(0));
        assert!(vec.is_empty());
    }
}
