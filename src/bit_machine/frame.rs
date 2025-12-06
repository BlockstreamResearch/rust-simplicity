// SPDX-License-Identifier: CC0-1.0

//! # Simplicity Frame
//!
//! Implementation of Frames in the Simplicity BitMachine.
//! A frame is a, possibly empty, cell array with a cursor referencing
//! a cell in the array.

use crate::BitIter;

/// Context to access a sub-slice of [`super::exec::BitMachine`]'s data.
/// Read and write operations require a reference to the data,
/// as it is not contained in this struct.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct Frame {
    /// Current position of the cursor.
    /// For read frames, this is the next bit which is to be read.
    /// For write frames, this is the next bit which is to be (over)written.
    cursor: usize,
    /// Start index of this frame in the referenced data.
    start: usize,
    /// The total bit length of this frame.
    len: usize,
}

impl Frame {
    /// Create a new frame that starts at the given index and that is of given length.
    pub(super) fn new(start: usize, len: usize) -> Self {
        Frame {
            cursor: start,
            start,
            len,
        }
    }

    /// Return the start index of the frame inside the referenced data.
    pub(super) fn start(&self) -> usize {
        self.start
    }

    /// Return the bit width of the frame.
    pub(super) fn bit_width(&self) -> usize {
        self.len
    }

    /// Reset the cursor to the start.
    pub(super) fn reset_cursor(&mut self) {
        self.cursor = self.start;
    }

    /// Return the current bit.
    pub(super) fn peek_bit(&self, data: &[u8]) -> bool {
        let (byte_index, bit_index) = get_indices(self.cursor);
        data[byte_index] & (1 << (7 - bit_index)) != 0
    }

    /// Return the current bit and advance the cursor.
    pub(super) fn read_bit(&mut self, data: &[u8]) -> bool {
        let (byte_index, bit_index) = get_indices(self.cursor);
        let bit = data[byte_index] & (1 << (7 - bit_index)) != 0;
        self.cursor += 1;
        bit
    }

    /// Write the given value to the current bit and advance the cursor.
    pub(super) fn write_bit(&mut self, bit: bool, data: &mut [u8]) {
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
    pub(super) fn write_u8(&mut self, value: u8, data: &mut [u8]) {
        for idx in 0..8 {
            self.write_bit(value & (1 << (7 - idx)) != 0, data);
        }
    }

    /// Move the cursor forward by the given length.
    pub(super) fn move_cursor_forward(&mut self, len: usize) {
        self.cursor += len;
    }

    /// Move the cursor backward by the given length.
    pub(super) fn move_cursor_backward(&mut self, len: usize) {
        self.cursor -= len;
    }

    /// Copy a bit string of given length from another frame into the present one.
    pub(super) fn copy_from(&mut self, other: &Self, len: usize, data: &mut [u8]) {
        for i in 0..len {
            let (other_byte_index, other_bit_index) = get_indices(other.cursor + i);
            let bit = data[other_byte_index] & (1 << (7 - other_bit_index)) != 0;
            self.write_bit(bit, data);
        }
    }

    /// Extend the present frame with a read-only reference the the data
    /// and return the resulting struct.
    pub(super) fn as_bit_iter<'a>(
        &self,
        data: &'a [u8],
    ) -> BitIter<core::iter::Copied<core::slice::Iter<'a, u8>>> {
        BitIter::byte_slice_window(data, self.start, self.start + self.len)
    }
}

fn get_indices(cursor: usize) -> (usize, usize) {
    let byte_index = cursor / 8;
    let bit_index = cursor % 8;

    (byte_index, bit_index)
}
