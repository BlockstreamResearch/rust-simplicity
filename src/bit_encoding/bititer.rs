// SPDX-License-Identifier: CC0-1.0

//! Bit Iterator functionality
//!
//! Simplicity programs are encoded bitwise rather than bytewise. This
//! module provides some helper functionality to make efficient parsing
//! easier. In particular, the `BitIter` type takes a byte iterator and
//! wraps it with some additional functionality (including implementing
//! `Iterator<Item=bool>`.
//!

use crate::decode;
use crate::{Cmr, FailEntropy};
use std::{error, fmt};

/// Attempted to read from a bit iterator, but there was no more data
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EarlyEndOfStreamError;

/// Closed out a bit iterator and there was remaining data.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CloseError {
    /// The iterator was closed but the underlying byte iterator was
    /// still yielding data.
    TrailingBytes {
        /// The first unused byte from the iterator.
        first_byte: u8,
    },
    IllegalPadding {
        masked_padding: u8,
        n_bits: usize,
    },
}

impl fmt::Display for CloseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CloseError::TrailingBytes { first_byte } => {
                write!(f, "bitstream had trailing bytes 0x{:02x}...", first_byte)
            }
            CloseError::IllegalPadding {
                masked_padding,
                n_bits,
            } => write!(
                f,
                "bitstream had {n_bits} bits in its last byte 0x{:02x}, not all zero",
                masked_padding
            ),
        }
    }
}

impl error::Error for CloseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

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

impl From<u2> for u8 {
    fn from(s: u2) -> u8 {
        match s {
            u2::_0 => 0,
            u2::_1 => 1,
            u2::_2 => 2,
            u2::_3 => 3,
        }
    }
}

/// Bitwise iterator formed from a wrapped bytewise iterator. Bytes are
/// interpreted big-endian, i.e. MSB is returned first
#[derive(Debug, Clone)]
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
            // Set to 8 to force next `Iterator::next` to read a new byte
            // from the underlying iterator
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
            // Set to 8 to force next `Iterator::next` to read a new byte
            // from the underlying iterator
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
            // Set to 8 to force next `Iterator::next` to read a new byte
            // from the underlying iterator
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

impl<'a> BitIter<std::iter::Copied<std::slice::Iter<'a, u8>>> {
    /// Creates a new bitwise iterator from a bytewise one. Equivalent
    /// to using `From`
    pub fn byte_slice_window(sl: &'a [u8], start: usize, end: usize) -> Self {
        assert!(start <= end);
        assert!(end <= sl.len() * 8);

        let actual_sl = &sl[start / 8..end.div_ceil(8)];
        let mut iter = actual_sl.iter().copied();

        let read_bits = start % 8;
        if read_bits == 0 {
            BitIter {
                iter,
                cached_byte: 0,
                read_bits: 8,
                total_read: 0,
            }
        } else {
            BitIter {
                cached_byte: iter.by_ref().next().unwrap(),
                iter,
                read_bits,
                total_read: 0,
            }
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
        self.total_read += 8;

        Ok(cached.checked_shl(self.read_bits as u32).unwrap_or(0)
            + (self.cached_byte >> (8 - self.read_bits)))
    }

    /// Reads a 256-bit CMR from the iterator.
    pub fn read_cmr(&mut self) -> Result<Cmr, EarlyEndOfStreamError> {
        let mut ret = [0; 32];
        for byte in &mut ret {
            *byte = self.read_u8()?;
        }
        Ok(Cmr::from_byte_array(ret))
    }

    /// Reads a 512-bit fail-combinator entropy from the iterator.
    pub fn read_fail_entropy(&mut self) -> Result<FailEntropy, EarlyEndOfStreamError> {
        let mut ret = [0; 64];
        for byte in &mut ret {
            *byte = self.read_u8()?;
        }
        Ok(FailEntropy::from_byte_array(ret))
    }

    /// Decode a natural number from bits.
    ///
    /// If a bound is specified, then the decoding terminates before trying to
    /// decode a larger number.
    pub fn read_natural(&mut self, bound: Option<usize>) -> Result<usize, decode::Error> {
        decode::decode_natural(self, bound)
    }

    /// Accessor for the number of bits which have been read,
    /// in total, from this iterator
    pub fn n_total_read(&self) -> usize {
        self.total_read
    }

    /// Consumes the bit iterator, checking that there are no remaining
    /// bytes and that any unread bits are zero.
    pub fn close(mut self) -> Result<(), CloseError> {
        if let Some(first_byte) = self.iter.next() {
            return Err(CloseError::TrailingBytes { first_byte });
        }

        debug_assert!(self.read_bits >= 1);
        debug_assert!(self.read_bits <= 8);
        let n_bits = 8 - self.read_bits;
        let masked_padding = self.cached_byte & ((1u8 << n_bits) - 1);
        if masked_padding != 0 {
            Err(CloseError::IllegalPadding {
                masked_padding,
                n_bits,
            })
        } else {
            Ok(())
        }
    }
}

/// Functionality for Boolean iterators to collect their bits or bytes.
pub trait BitCollector: Sized {
    /// Collect the bits of the iterator into a byte vector and a bit length.
    fn collect_bits(self) -> (Vec<u8>, usize);

    /// Try to collect the bits of the iterator into a clean byte vector.
    ///
    /// This fails if the number of bits is not divisible by 8.
    fn try_collect_bytes(self) -> Result<Vec<u8>, &'static str> {
        let (bytes, bit_length) = self.collect_bits();
        if bit_length % 8 == 0 {
            Ok(bytes)
        } else {
            Err("Number of collected bits is not divisible by 8")
        }
    }
}

impl<I: Iterator<Item = bool>> BitCollector for I {
    fn collect_bits(self) -> (Vec<u8>, usize) {
        let mut bytes = vec![];
        let mut unfinished_byte = Vec::with_capacity(8);

        for bit in self {
            unfinished_byte.push(bit);

            if unfinished_byte.len() == 8 {
                bytes.push(
                    unfinished_byte
                        .iter()
                        .fold(0, |acc, &b| acc * 2 + u8::from(b)),
                );
                unfinished_byte.clear();
            }
        }

        let bit_length = bytes.len() * 8 + unfinished_byte.len();

        if !unfinished_byte.is_empty() {
            unfinished_byte.resize(8, false);
            bytes.push(
                unfinished_byte
                    .iter()
                    .fold(0, |acc, &b| acc * 2 + u8::from(b)),
            );
        }

        (bytes, bit_length)
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

    #[test]
    fn byte_slice_window() {
        let data = [0x12, 0x23, 0x34];

        let mut full = BitIter::byte_slice_window(&data, 0, 24);
        assert_eq!(full.read_u8(), Ok(0x12));
        assert_eq!(full.n_total_read(), 8);
        assert_eq!(full.read_u8(), Ok(0x23));
        assert_eq!(full.n_total_read(), 16);
        assert_eq!(full.read_u8(), Ok(0x34));
        assert_eq!(full.n_total_read(), 24);
        assert_eq!(full.read_u8(), Err(EarlyEndOfStreamError));

        let mut mid = BitIter::byte_slice_window(&data, 8, 16);
        assert_eq!(mid.read_u8(), Ok(0x23));
        assert_eq!(mid.read_u8(), Err(EarlyEndOfStreamError));

        let mut offs = BitIter::byte_slice_window(&data, 4, 20);
        assert_eq!(offs.read_u8(), Ok(0x22));
        assert_eq!(offs.read_u8(), Ok(0x33));
        assert_eq!(offs.read_u8(), Err(EarlyEndOfStreamError));

        let mut shift1 = BitIter::byte_slice_window(&data, 1, 24);
        assert_eq!(shift1.read_u8(), Ok(0x24));
        assert_eq!(shift1.read_u8(), Ok(0x46));
        assert_eq!(shift1.read_u8(), Err(EarlyEndOfStreamError));

        let mut shift7 = BitIter::byte_slice_window(&data, 7, 24);
        assert_eq!(shift7.read_u8(), Ok(0x11));
        assert_eq!(shift7.read_u8(), Ok(0x9a));
        assert_eq!(shift7.read_u8(), Err(EarlyEndOfStreamError));
    }
}
