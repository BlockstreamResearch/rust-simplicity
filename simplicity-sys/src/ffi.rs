// SPDX-License-Identifier: CC0-1.0

//! FFI Bindings
//!
//! This module contains bindings to the C library types and functions
//! that are required to execute jets.
//! It is split into several modules, each one corresponding to a `.h` file
//! in the C library.
//!
//! All types are converted to CamelCase and prefixed with the letter C;
//! function names are unchanged.

#![allow(non_camel_case_types)]

pub use core::ffi::c_void;

pub type c_uchar = u8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_size_t = usize;
#[cfg(target_arch = "wasm32")]
pub type c_uint_fast16_t = u16;
#[cfg(not(target_arch = "wasm32"))]
pub type c_uint_fast16_t = usize;
pub type c_uint_least32_t = u32;

extern "C" {
    pub static c_sizeof_uchar: c_size_t;
    pub static c_alignof_uchar: c_size_t;
    pub static c_sizeof_int: c_size_t;
    pub static c_alignof_int: c_size_t;
    pub static c_sizeof_uint: c_size_t;
    pub static c_alignof_uint: c_size_t;
    pub static c_sizeof_size_t: c_size_t;
    pub static c_alignof_size_t: c_size_t;
    pub static c_sizeof_uint_fast16_t: c_size_t;
    pub static c_alignof_uint_fast16_t: c_size_t;
    pub static c_sizeof_uint_least32_t: c_size_t;
    pub static c_alignof_uint_least32_t: c_size_t;
}

pub type ubounded = c_uint_least32_t;
/// Used with `evalTCOProgram` to enforce consensus limits.
pub const BUDGET_MAX: ubounded = 4000050;
/// The max value of UBOUNDED_MAX
pub const UBOUNDED_MAX: ubounded = ubounded::MAX;

extern "C" {
    pub static c_sizeof_ubounded: c_size_t;
    pub static c_alignof_ubounded: c_size_t;
}

pub mod bounded {
    use super::ubounded;
    extern "C" {
        pub static c_overhead: ubounded;
    }

    /// constant Overhead of each jet
    pub fn cost_overhead() -> ubounded {
        unsafe { c_overhead }
    }
}

pub type UWORD = c_uint_fast16_t;

extern "C" {
    pub static c_sizeof_UWORD: c_size_t;
    pub static c_alignof_UWORD: c_size_t;
}

pub mod sha256 {
    use hashes::sha256::Midstate;
    use libc::size_t;

    /// The 256-bit array of a SHA-256 hash or midstate.
    #[repr(C)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    pub struct CSha256Midstate {
        pub s: [u32; 8],
    }

    impl From<CSha256Midstate> for Midstate {
        fn from(c_midstate: CSha256Midstate) -> Midstate {
            let mut inner = [0; 32];
            for (idx, chunk) in c_midstate.s.iter().enumerate() {
                inner[idx * 4..(idx + 1) * 4].copy_from_slice(&chunk.to_be_bytes());
            }
            Midstate(inner)
        }
    }

    impl From<Midstate> for CSha256Midstate {
        fn from(midstate: Midstate) -> CSha256Midstate {
            let mut s = [0; 8];
            for (idx, chunk) in midstate.0.chunks(4).enumerate() {
                s[idx] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            }
            CSha256Midstate { s }
        }
    }

    extern "C" {
        pub static c_sizeof_sha256_midstate: size_t;
        pub static c_alignof_sha256_midstate: size_t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    #[rustfmt::skip]
    fn test_sizes() {
        unsafe {
            assert_eq!(size_of::<c_uchar>(), c_sizeof_uchar);
            assert_eq!(size_of::<c_int>(), c_sizeof_int);
            assert_eq!(size_of::<c_uint>(), c_sizeof_uint);
            assert_eq!(size_of::<c_size_t>(), c_sizeof_size_t);
            assert_eq!(size_of::<c_uint_fast16_t>(), c_sizeof_uint_fast16_t);
            assert_eq!(size_of::<c_uint_least32_t>(), c_sizeof_uint_least32_t);
            assert_eq!(size_of::<ubounded>(), c_sizeof_ubounded);
            assert_eq!(size_of::<UWORD>(), c_sizeof_UWORD);
            assert_eq!(size_of::<sha256::CSha256Midstate>(), sha256::c_sizeof_sha256_midstate);
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_aligns() {
        unsafe {
            assert_eq!(align_of::<c_uchar>(), c_alignof_uchar);
            assert_eq!(align_of::<c_int>(), c_alignof_int);
            assert_eq!(align_of::<c_uint>(), c_alignof_uint);
            assert_eq!(align_of::<c_size_t>(), c_alignof_size_t);
            assert_eq!(align_of::<c_uint_fast16_t>(), c_alignof_uint_fast16_t);
            assert_eq!(align_of::<c_uint_least32_t>(), c_alignof_uint_least32_t);
            assert_eq!(align_of::<ubounded>(), c_alignof_ubounded);
            assert_eq!(align_of::<UWORD>(), c_alignof_UWORD);
            assert_eq!(align_of::<sha256::CSha256Midstate>(), sha256::c_alignof_sha256_midstate);
        }
    }
}
