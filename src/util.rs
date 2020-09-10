// Rust Bitcoin Library
// Written by
//   The Rust Bitcoin developers
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

// utility functions from rust-bitcoin
// Fix these after bitcoin_num crate is released
macro_rules! define_slice_to_be {
    ($name: ident, $type: ty) => {
        #[inline]
        pub fn $name(slice: &[u8]) -> $type {
            assert_eq!(slice.len(), ::std::mem::size_of::<$type>());
            let mut res = 0;
            for i in 0..::std::mem::size_of::<$type>() {
                res |= (slice[i] as $type) << (::std::mem::size_of::<$type>() - i - 1) * 8;
            }
            res
        }
    };
}

define_slice_to_be!(slice_to_u64_be, u64);
define_slice_to_be!(slice_to_u32_be, u32);
