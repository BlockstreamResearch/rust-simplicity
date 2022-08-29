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

use libc::{c_uchar, size_t};

/// String of bits.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Bitstring {
    arr: *const c_uchar,
    len: size_t,
    offset: size_t,
}