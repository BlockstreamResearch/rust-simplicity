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

//! Bit Iterator functionality
//!
//! Simplicity programs are encoded bitwise rather than bytewise. This
//! module provides some helper functionality to make efficient parsing
//! easier. In particular, the `BitIter` type takes a byte iterator and
//! wraps it with some additional functionality (including implementing
//! `Iterator<Item=bool>`.
//!

/// Bitwise iterator formed from a wrapped bytewise iterator. Bytes are
/// interpreted big-endian, i.e. MSB is returned first
#[derive(Debug)]
pub struct BitIter<I: Iterator<Item = u8>> {
    /// Byte iterator
    iter: I,
    /// Current byte that contains next bit
    cached_byte: u8,
    /// Number of read bits in current byte
    read_bits: usize,
    /// Total number of read bits
    total_read: usize,
}

impl From<Vec<u8>> for BitIter<std::vec::IntoIter<u8>> {
    fn from(v: Vec<u8>) -> Self {
        BitIter {
            iter: v.into_iter(),
            cached_byte: 0,
            read_bits: 8,
            total_read: 0,
        }
    }
}

impl<'a> From<&'a [u8]> for BitIter<std::iter::Copied<std::slice::Iter<'a, u8>>> {
    fn from(sl: &'a [u8]) -> Self {
        BitIter {
            iter: sl.iter().copied(),
            cached_byte: 0,
            read_bits: 8,
            total_read: 0,
        }
    }
}

impl<I: Iterator<Item = u8>> From<I> for BitIter<I> {
    fn from(iter: I) -> Self {
        BitIter {
            iter,
            cached_byte: 0,
            read_bits: 8,
            total_read: 0,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for BitIter<I> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.read_bits < 8 {
            self.read_bits += 1;
            self.total_read += 1;
            Some(self.cached_byte & (1 << (8 - self.read_bits as u8)) != 0)
        } else {
            self.cached_byte = self.iter.next()?;
            self.read_bits = 0;
            self.next()
        }
    }
}

impl<I: Iterator<Item = u8>> BitIter<I> {
    /// Creates a new bitwise iterator from a bytewise one. Equivalent
    /// to using `From`
    pub fn new(iter: I) -> Self {
        Self::from(iter)
    }

    /// Reads up to 64 bits as a big-endian number
    ///
    /// `n` must be between 0 and 63 inclusive; the function will panic
    /// otherwise.
    ///
    /// Returns `None` if the underlying byte iterator runs out
    pub fn read_bits_be(&mut self, mut n: usize) -> Option<u64> {
        assert!(n < 64);
        if n == 0 {
            return Some(0);
        }

        // 0 <= avail_bits <= 8
        // 0 <= n < 64
        let avail_bits = 8 - self.read_bits;
        if avail_bits < n {
            n -= avail_bits;
            let mask = if avail_bits < 8 {
                (1 << avail_bits) - 1
            } else {
                0xff
            };
            let pre_result = ((self.cached_byte & mask) as u64) << n;
            self.cached_byte = self.iter.next()?;
            self.read_bits = 0;

            if let Some(read) = self.read_bits_be(n) {
                self.total_read += avail_bits;
                Some(pre_result + read)
            } else {
                None
            }
        } else {
            // 0 <= n <= 8
            self.read_bits += n;
            self.total_read += n;
            let mask = if n < 8 { (1 << n) - 1 } else { 0xff };
            Some(((self.cached_byte >> (avail_bits - n)) & mask) as u64)
        }
    }

    /// Accessor for the number of bits which have been read,
    /// in total, from this iterator
    pub fn n_total_read(&self) -> usize {
        self.total_read
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_iter() {
        let mut iter = BitIter::from([].iter().cloned());
        assert!(iter.next().is_none());
        assert_eq!(iter.read_bits_be(0), Some(0));
        assert_eq!(iter.read_bits_be(1), None);
        assert_eq!(iter.read_bits_be(63), None);
        assert_eq!(iter.n_total_read(), 0);
    }

    #[test]
    fn one_bit_iter() {
        let mut iter = BitIter::from([0x80].iter().cloned());
        assert_eq!(iter.read_bits_be(0), Some(0));
        assert_eq!(iter.read_bits_be(1), Some(1));
        assert_eq!(iter.read_bits_be(7), Some(0));
        assert_eq!(iter.read_bits_be(1), None);
        assert_eq!(iter.n_total_read(), 8);
    }

    #[test]
    fn bit_by_bit() {
        let mut iter = BitIter::from([0x0f, 0xaa].iter().cloned());
        for _ in 0..4 {
            assert_eq!(iter.next(), Some(false));
        }
        for _ in 0..4 {
            assert_eq!(iter.next(), Some(true));
        }
        for _ in 0..4 {
            assert_eq!(iter.next(), Some(true));
            assert_eq!(iter.next(), Some(false));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn six_by_six() {
        let mut iter = BitIter::from([0x0f, 0xaa, 0x00].iter().cloned());
        assert_eq!(iter.read_bits_be(6), Some(0x03)); // 00 0011
        assert_eq!(iter.read_bits_be(6), Some(0x3a)); // 11 1010
        assert_eq!(iter.read_bits_be(6), Some(0x28)); // 10 1000
        assert_eq!(iter.read_bits_be(6), Some(0x00)); // 00 0000
        assert_eq!(iter.n_total_read(), 24);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.n_total_read(), 24);
    }

    #[test]
    fn regression_1() {
        let mut iter = BitIter::from([0x34, 0x90].iter().cloned());
        assert_eq!(iter.read_bits_be(4), Some(0x03)); // 0011
        assert_eq!(iter.next(), Some(false)); // 0
        assert_eq!(iter.read_bits_be(2), Some(0x02)); // 10
        assert_eq!(iter.read_bits_be(2), Some(0x01)); // 01
        assert_eq!(iter.n_total_read(), 9);
    }
}
