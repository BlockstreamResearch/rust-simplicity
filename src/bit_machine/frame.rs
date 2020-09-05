// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//   Sanket Kanjalkar <sanket1729@gmail.com>
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

//! # Simplicity Frame
//!
//! Implementation of Frames in the Simplicity BitMachine.
//! A frame is a, possibly empty, cell array with a cursor referencing
//! a cell in the array.

use std::fmt;
use std::mem::size_of;
use std::ops::{Add, Shl};

/// Context to access a sub-slice of [`super::exec::BitMachine`]'s data.
/// Read and write operations require a reference to the data,
/// as it is not contained in this struct.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Frame {
    /// Current position of the cursor.
    /// For read frames, this is the next bit which is to be read.
    /// For write frames, this is the next bit which is to be (over)written.
    pub(crate) cursor: usize,
    /// Start index of this frame in the referenced data.
    pub(crate) start: usize,
    /// The total length of this frame.
    pub(crate) len: usize,
}

impl Frame {
    /// Create a new frame that starts at the given index and that is of given length.
    pub(crate) fn new(start: usize, len: usize) -> Self {
        Frame {
            cursor: start,
            start,
            len,
        }
    }

    /// Reset the cursor to the start.
    pub(crate) fn reset_cursor(&mut self) {
        self.cursor = self.start;
    }

    /// Return the current bit.
    pub(crate) fn peek_bit(&self, data: &[u8]) -> bool {
        let (byte_index, bit_index) = get_indices(self.cursor);
        data[byte_index] & (1 << (7 - bit_index)) != 0
    }

    /// Return the current bit and advance the cursor.
    pub(crate) fn read_bit(&mut self, data: &[u8]) -> bool {
        let (byte_index, bit_index) = get_indices(self.cursor);
        let bit = data[byte_index] & (1 << (7 - bit_index)) != 0;
        self.cursor += 1;
        bit
    }

    /// Read a big-endian u8 value and advance the cursor.
    pub(crate) fn read_u8(&mut self, data: &[u8]) -> u8 {
        self.read_unsigned(data)
    }

    /// Read a big-endian u16 value and advance the cursor.
    pub(crate) fn read_u16(&mut self, data: &[u8]) -> u16 {
        self.read_unsigned(data)
    }

    /// Read a big-endian u32 value and advance the cursor.
    pub(crate) fn read_u32(&mut self, data: &[u8]) -> u32 {
        self.read_unsigned(data)
    }

    /// Read a big-endian u64 value and advance the cursor.
    pub(crate) fn read_u64(&mut self, data: &[u8]) -> u64 {
        self.read_unsigned(data)
    }

    /// Write the given value to the current bit and advance the cursor.
    pub(crate) fn write_bit(&mut self, bit: bool, data: &mut [u8]) {
        let (byte_index, bit_index) = get_indices(self.cursor);
        let write_mask = 1 << (7 - bit_index);

        if bit {
            data[byte_index] |= write_mask;
        } else {
            data[byte_index] &= !write_mask;
        }

        self.cursor += 1;
    }

    /// Write a big-endian u8 value and advance the cursor.
    pub(crate) fn write_u8(&mut self, value: u8, data: &mut [u8]) {
        for idx in 0..8 {
            self.write_bit(value & (1 << (7 - idx)) != 0, data);
        }
    }

    /// Write a big-endian u8 value and advance the cursor.
    pub(crate) fn write_u16(&mut self, value: u16, data: &mut [u8]) {
        for idx in 0..16 {
            self.write_bit(value & (1 << (15 - idx)) != 0, data);
        }
    }

    /// Write a big-endian u8 value and advance the cursor.
    pub(crate) fn write_u32(&mut self, value: u32, data: &mut [u8]) {
        for idx in 0..32 {
            self.write_bit(value & (1 << (31 - idx)) != 0, data);
        }
    }

    /// Write a big-endian u8 value and advance the cursor.
    pub(crate) fn write_u64(&mut self, value: u64, data: &mut [u8]) {
        for idx in 0..64 {
            self.write_bit(value & (1 << (63 - idx)) != 0, data);
        }
    }

    /// Move the cursor forward by the given length.
    pub(crate) fn move_cursor_forward(&mut self, len: usize) {
        self.cursor += len;
    }

    /// Move the cursor backward by the given length.
    pub(crate) fn move_cursor_backward(&mut self, len: usize) {
        self.cursor -= len;
    }

    /// Copy a bit string of given length from another frame into the present one.
    pub(crate) fn copy_from(&mut self, other: &Self, len: usize, data: &mut [u8]) {
        for i in 0..len {
            let (other_byte_index, other_bit_index) = get_indices(other.cursor + i);
            let bit = data[other_byte_index] & (1 << (7 - other_bit_index)) != 0;
            self.write_bit(bit, data);
        }
    }

    /// Extend the present frame with a read-only reference the the data
    /// and return the resulting struct.
    pub fn to_frame_data<'a>(&self, data: &'a [u8]) -> FrameData<'a> {
        FrameData::new(self, data)
    }

    fn read_unsigned<T>(&mut self, data: &[u8]) -> T
    where
        T: From<u8> + Shl<usize, Output = T> + Add<Output = T>,
    {
        let (mut self_byte_index, self_bit_index) = get_indices(self.cursor);
        let mut read_number;

        let number_bits = size_of::<T>() * 8;
        let number_leading_bits = 8 - self_bit_index;
        let number_full_bytes = (number_bits - number_leading_bits) / 8;
        let number_trailing_bits = number_bits - number_leading_bits - number_full_bytes * 8;

        // Read leading bits
        let read_mask = 0xff >> self_bit_index;
        let masked_data = data[self_byte_index] & read_mask;
        read_number = T::from(masked_data << self_bit_index);
        self_byte_index += 1;

        for _ in 0..number_full_bytes {
            let current_number = T::from(data[self_byte_index]) << self_bit_index;
            read_number = (read_number << 8) + current_number;
            self_byte_index += 1;
        }

        if number_trailing_bits != 0 {
            let read_mask = 0xff << (8 - number_trailing_bits);
            let masked_data = data[self_byte_index] & read_mask;
            read_number = read_number + T::from(masked_data >> (8 - number_trailing_bits));
        }

        self.cursor += number_bits;
        read_number
    }
}

/// View onto a sub-slice of the Bit Machine's data.
/// In contrast to [`Frame`],
/// this struct contains a read-only reference to the data
/// and can print / iterate over it.
#[derive(Eq, PartialEq)]
pub(crate) struct FrameData<'a> {
    data: &'a [u8],
    start: usize,
    cursor: usize,
    end: usize,
}

impl<'a> FrameData<'a> {
    fn new(frame: &Frame, data: &'a [u8]) -> Self {
        FrameData {
            data,
            start: frame.start,
            cursor: frame.cursor,
            end: frame.start + frame.len,
        }
    }
}

impl<'a> fmt::Debug for FrameData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;

        for i in self.start..self.end {
            if i == self.cursor {
                f.write_str("^")?;
            }

            let (byte_index, bit_index) = get_indices(i);
            let bit = self.data[byte_index] & (1 << (7 - bit_index)) != 0;

            if bit {
                f.write_str("1")?;
            } else {
                f.write_str("0")?;
            }
        }

        if self.cursor == self.end {
            f.write_str("^")?;
        }

        f.write_str("]")?;
        Ok(())
    }
}

impl<'a> ExactSizeIterator for FrameData<'a> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'a> Iterator for FrameData<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.end {
            None
        } else {
            let (byte_index, bit_index) = get_indices(self.cursor);
            let next_bit = self.data[byte_index] & (1 << (7 - bit_index)) != 0;
            self.cursor += 1;
            Some(next_bit)
        }
    }
}

fn get_indices(cursor: usize) -> (usize, usize) {
    let byte_index = cursor / 8;
    let bit_index = cursor % 8;

    (byte_index, bit_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::bitvec_to_bytevec;

    #[test]
    fn read_unsigned() {
        let bytes = (0..100).collect::<Vec<u8>>();
        let mut frame = Frame::new(0, 100 * 8);

        assert_eq!(frame.read_u8(&bytes), 0);
        assert_eq!(frame.read_u8(&bytes), 1);
        assert_eq!(frame.read_u8(&bytes), 2);
        assert_eq!(frame.read_u16(&bytes), 3 * 256 + 4);
        assert_eq!(frame.read_u16(&bytes), 5 * 256 + 6);
        assert_eq!(
            frame.read_u32(&bytes),
            7 * 2u32.pow(24) + 8 * 2u32.pow(16) + 9 * 2u32.pow(8) + 10
        );
        assert_eq!(
            frame.read_u32(&bytes),
            11 * 2u32.pow(24) + 12 * 2u32.pow(16) + 13 * 2u32.pow(8) + 14
        );
        assert_eq!(
            frame.read_u64(&bytes),
            15 * 2u64.pow(8 * 7)
                + 16 * 2u64.pow(8 * 6)
                + 17 * 2u64.pow(8 * 5)
                + 18 * 2u64.pow(8 * 4)
                + 19 * 2u64.pow(8 * 3)
                + 20 * 2u64.pow(8 * 2)
                + 21 * 2u64.pow(8)
                + 22
        );

        // assert_eq!(f.read_u8(&vector), 23);
        // 23 = 0001 0111
        // our iterator reads from behind, so it should read
        // 0 -> 0 -> 0 -> 1
        assert_eq!(frame.read_bit(&bytes), false);
        assert_eq!(frame.read_bit(&bytes), false);
        assert_eq!(frame.read_bit(&bytes), false);
        assert_eq!(frame.read_bit(&bytes), true);

        // (0111 | 0001) 1000
        // 16*7 + 1 = 113
        assert_eq!(frame.read_u8(&bytes), 113);
        assert_eq!(frame.read_bit(&bytes), true);
        assert_eq!(frame.read_u8(&bytes), 3);
        assert_eq!(frame.read_u16(&bytes), 9027);
        assert_eq!(frame.read_u32(&bytes), 1669571523);
    }

    #[test]
    fn test_to_frame_data_iter() {
        let bytes = (0..100).collect::<Vec<u8>>();
        let frame = Frame::new(0, 100 * 8);
        let bits = frame.to_frame_data(&bytes).collect::<Vec<_>>();
        let computed_bytes = bitvec_to_bytevec(&bits);

        assert_eq!(bytes, computed_bytes);
    }
}
