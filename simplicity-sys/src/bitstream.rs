// Simplicity Bindings
// Written in 2022 by
//   Christian Lewe <clewe@blockstream.com>
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

use libc::{c_char, c_int, c_uchar, c_void, FILE};
use std::marker::PhantomData;

/// Stream of bits.
#[repr(C)]
pub struct Bitstream<'a> {
    file: *mut FILE,
    available: c_int,
    byte: c_uchar,
    _marker: PhantomData<&'a u8>,
}

impl<'a> From<&'a [u8]> for Bitstream<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        unsafe {
            let mode = "r";
            let file = libc::fmemopen(
                bytes.as_ptr() as *mut c_void,
                bytes.len(),
                mode.as_ptr() as *const c_char,
            );

            Bitstream {
                file,
                available: 0,
                byte: c_uchar::default(),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> Drop for Bitstream<'a> {
    fn drop(&mut self) {
        unsafe {
            libc::fclose(self.file);
        }
    }
}
