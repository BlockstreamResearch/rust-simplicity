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

use crate::Cmr;

/// Attempted to read from a bit iterator, but there was no more data
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EarlyEndOfStreamError;

/// Two-bit type used during decoding
///
/// Use of an enum rather than a numeric primitive type makes it easier to
/// match on.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum u2 {
    _0,
    _1,
    _2,
    _3,
}

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

    /// Reads a single bit from the iterator.
    pub fn read_bit(&mut self) -> Result<bool, EarlyEndOfStreamError> {
        self.next().ok_or(EarlyEndOfStreamError)
    }

    /// Reads two bits from the iterator.
    pub fn read_u2(&mut self) -> Result<u2, EarlyEndOfStreamError> {
        match (self.next(), self.next()) {
            (Some(false), Some(false)) => Ok(u2::_0),
            (Some(false), Some(true)) => Ok(u2::_1),
            (Some(true), Some(false)) => Ok(u2::_2),
            (Some(true), Some(true)) => Ok(u2::_3),
            _ => Err(EarlyEndOfStreamError),
        }
    }

    /// Reads a byte from the iterator.
    pub fn read_u8(&mut self) -> Result<u8, EarlyEndOfStreamError> {
        debug_assert!(self.read_bits > 0);
        let cached = self.cached_byte;
        self.cached_byte = self.iter.next().ok_or(EarlyEndOfStreamError)?;

        Ok(cached.checked_shl(self.read_bits as u32).unwrap_or(0)
            + (self.cached_byte >> (8 - self.read_bits)))
    }

    /// Reads a 256-bit CMR from the iterator.
    pub fn read_cmr(&mut self) -> Result<Cmr, EarlyEndOfStreamError> {
        let mut ret = [0; 32];
        for byte in &mut ret {
            *byte = self.read_u8()?;
        }
        Ok(Cmr::from(ret))
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
        assert_eq!(iter.read_bit(), Err(EarlyEndOfStreamError));
        assert_eq!(iter.read_u2(), Err(EarlyEndOfStreamError));
        assert_eq!(iter.read_u8(), Err(EarlyEndOfStreamError));
        assert_eq!(iter.read_cmr(), Err(EarlyEndOfStreamError));
        assert_eq!(iter.n_total_read(), 0);
    }

    #[test]
    fn one_bit_iter() {
        let mut iter = BitIter::from([0x80].iter().cloned());
        assert_eq!(iter.read_bit(), Ok(true));
        assert_eq!(iter.read_bit(), Ok(false));
        assert_eq!(iter.read_u8(), Err(EarlyEndOfStreamError));
        assert_eq!(iter.n_total_read(), 2);
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
    fn byte_by_byte() {
        let mut iter = BitIter::from([0x0f, 0xaa].iter().cloned());
        assert_eq!(iter.read_u8(), Ok(0x0f));
        assert_eq!(iter.read_u8(), Ok(0xaa));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn regression_1() {
        let mut iter = BitIter::from([0x34, 0x90].iter().cloned());
        assert_eq!(iter.read_u2(), Ok(u2::_0)); // 0011
        assert_eq!(iter.read_u2(), Ok(u2::_3)); // 0011
        assert_eq!(iter.next(), Some(false)); // 0
        assert_eq!(iter.read_u2(), Ok(u2::_2)); // 10
        assert_eq!(iter.read_u2(), Ok(u2::_1)); // 01
        assert_eq!(iter.n_total_read(), 9);
    }
}
