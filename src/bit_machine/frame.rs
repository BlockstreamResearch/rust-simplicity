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
//! Implementation of the Frame in Simplicity BitMachine.
//! A frame is a, possibly empty, cell array with a cursor referencing
//! a cell in the array.

use std::{fmt, ptr};

/// A frame used internally by the Bit Machine to keep track of
/// where we are reading or writing to
pub(crate) struct Frame {
    /// Base pointer to the data
    pub(in crate::bit_machine) data: *mut u8,
    /// Current position of the cursor. For read frames this points to
    /// the next bit which is to be read. For write frames, this corresponds
    /// to the next bit where data would be written
    pub(in crate::bit_machine) abs_pos: isize,
    /// Start index in of this frame in data
    pub(in crate::bit_machine) start: isize,
    /// The total length of this frame.
    pub(in crate::bit_machine) len: isize,
}

impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            for i in 0..self.len {
                if i == self.abs_pos - self.start {
                    f.write_str("^")?;
                }

                let p = self.data.offset((self.start + i) / 8);
                if *p & (1 << (7 - (self.start + i) % 8)) != 0 {
                    f.write_str("1")?;
                } else {
                    f.write_str("0")?;
                }
            }
        }
        Ok(())
    }
}

impl Iterator for Frame {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.abs_pos < self.start + self.len {
            let bit = self.peek_bit();
            self.abs_pos += 1;
            Some(bit)
        } else {
            None
        }
    }
}

macro_rules! READ_UNSIGNED {
    ($fn_name: ident, $ret_ty: ty) => {
        pub(crate) fn $fn_name(&mut self) -> $ret_ty {
            unsafe {
                let ret_size = std::mem::size_of::<$ret_ty>() as isize;
                let init_pos = self.abs_pos;
                // 1. read all bits till the end of the current byte.
                // Reads between 1 bit and 8 bits inclusive.
                let mut ret: $ret_ty = 0;
                let mask = 0xFF >> (self.abs_pos % 8);
                let mut p = self.data.offset(self.abs_pos / 8);
                ret += (mask & *p as $ret_ty);
                self.abs_pos += 8 - (self.abs_pos % 8);
                p = p.add(1);

                // 2. Read the next bytes that can be completely read
                for _ in 0..(ret_size - (self.abs_pos - init_pos + 7) / 8) {
                    ret = ret * (2 as $ret_ty).pow(8) + (*p as $ret_ty);
                    self.abs_pos += 8;
                    p = p.add(1);
                    debug_assert!(self.abs_pos % 8 == 0);
                }

                //3. Read partially the last byte if required
                let remaining_bits = ret_size * 8 - (self.abs_pos - init_pos);
                if remaining_bits != 0 {
                    let mask = 0xff;
                    ret = ret * (2 as $ret_ty).pow(remaining_bits as u32)
                        + ((mask & *p) >> (8 - remaining_bits)) as $ret_ty;
                    self.abs_pos += remaining_bits;
                }
                ret
            }
        }
    };
}

impl Frame {
    pub(in crate::bit_machine) fn read_at_rel(&self, n: isize) -> bool {
        unsafe {
            let p = self.data.offset((self.abs_pos + n) / 8);
            *p & (1 << (7 - (self.abs_pos + n) % 8)) != 0
        }
    }

    pub(in crate::bit_machine) fn peek_bit(&self) -> bool {
        unsafe {
            let p = self.data.offset(self.abs_pos / 8);
            *p & (1 << (7 - self.abs_pos % 8)) != 0
        }
    }

    pub(crate) fn read_bit(&mut self) -> bool {
        unsafe {
            let p = self.data.offset(self.abs_pos / 8);
            let ret = *p & (1 << (7 - self.abs_pos % 8)) != 0;
            self.abs_pos += 1;
            ret
        }
    }

    READ_UNSIGNED!(read_u8, u8);
    READ_UNSIGNED!(read_u16, u16);
    READ_UNSIGNED!(read_u32, u32);
    READ_UNSIGNED!(read_u64, u64);

    /// Write a big-endian u64 value to the current write frame
    pub(in crate::bit_machine) fn write_u64(&mut self, data: u64) {
        for idx in 0..64 {
            self.write_bit(data & (1 << (63 - idx)) != 0);
        }
    }

    /// Write a big-endian u32 value to the current write frame
    pub(in crate::bit_machine) fn write_u32(&mut self, data: u32) {
        for idx in 0..32 {
            self.write_bit(data & (1 << (31 - idx)) != 0);
        }
    }

    /// Write a big-endian u16 value to the current write frame
    pub(in crate::bit_machine) fn write_u16(&mut self, data: u16) {
        for idx in 0..16 {
            self.write_bit(data & (1 << (15 - idx)) != 0);
        }
    }

    /// Write a big-endian u8 value to the current write frame
    pub(in crate::bit_machine) fn write_u8(&mut self, data: u8) {
        for idx in 0..8 {
            self.write_bit(data & (1 << (7 - idx)) != 0);
        }
    }

    pub(in crate::bit_machine) fn write_bit(&mut self, b: bool) {
        let mask = 1 << (7 - self.abs_pos % 8);
        unsafe {
            let p = self.data.offset(self.abs_pos / 8);
            if b {
                *p |= mask;
            } else {
                *p &= !mask;
            }
        }
        self.abs_pos += 1;
    }

    pub(in crate::bit_machine) fn fwd(&mut self, n: usize) {
        self.abs_pos += n as isize;
    }

    pub(in crate::bit_machine) fn back(&mut self, n: usize) {
        self.abs_pos -= n as isize;
    }

    pub(in crate::bit_machine) fn copy_from(&mut self, other: &Frame, n: usize) {
        if self.abs_pos % 8 == 0 && other.abs_pos % 8 == 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    other.data.offset(other.abs_pos / 8),
                    self.data.offset(self.abs_pos / 8),
                    (n + 7) / 8,
                );
                self.abs_pos += n as isize;
            }
        } else {
            for i in 0..n as isize {
                let bit = unsafe {
                    let p = other.data.offset((other.abs_pos + i) / 8);
                    *p & (1 << (7 - (other.abs_pos + i) % 8)) != 0
                };
                self.write_bit(bit);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_u8() {
        let mut v = (0..100).collect::<Vec<u8>>();
        let p = v.as_mut_ptr();
        let mut f = Frame {
            data: p,
            abs_pos: 0,
            len: 8 * 100,
            start: 0,
        };
        assert_eq!(f.read_u8(), 0);
        assert_eq!(f.read_u8(), 1);
        assert_eq!(f.read_u8(), 2);
        assert_eq!(f.read_u16(), 3 * 256 + 4);
        assert_eq!(f.read_u16(), 5 * 256 + 6);
        assert_eq!(
            f.read_u32(),
            7 * 2u32.pow(24) + 8 * 2u32.pow(16) + 9 * 2u32.pow(8) + 10
        );
        assert_eq!(
            f.read_u32(),
            11 * 2u32.pow(24) + 12 * 2u32.pow(16) + 13 * 2u32.pow(8) + 14
        );
        assert_eq!(
            f.read_u64(),
            15 * 2u64.pow(8 * 7)
                + 16 * 2u64.pow(8 * 6)
                + 17 * 2u64.pow(8 * 5)
                + 18 * 2u64.pow(8 * 4)
                + 19 * 2u64.pow(8 * 3)
                + 20 * 2u64.pow(8 * 2)
                + 21 * 2u64.pow(8)
                + 22
        );

        // assert_eq!(f.read_u8(), 23);
        // 23 = 0001 0111
        // our iterator reads from behind, so it should read
        // 0 -> 0 -> 0 -> 1
        assert_eq!(f.read_bit(), false);
        assert_eq!(f.read_bit(), false);
        assert_eq!(f.read_bit(), false);
        assert_eq!(f.read_bit(), true);

        // (0111 | 0001) 1000
        // 16*7 + 1 = 113
        assert_eq!(f.read_u8(), 113);
        assert_eq!(f.read_bit(), true);
        assert_eq!(f.read_u8(), 3);
        assert_eq!(f.read_u16(), 9027);
        assert_eq!(f.read_u32(), 1669571523);
    }
}
