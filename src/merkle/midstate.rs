// SPDX-License-Identifier: CC0-1.0

//! # Midstate Extensions
//!
//! Private extensions to the `sha256::Midstate` type (some of which we should probably upstream).

use hashes::sha256::{HashEngine, Midstate};
use hashes::HashEngine as _;

/// Private trait to abstract over the various byte arrays that we "cast" to and from midstates
/// during construction (but not after).
pub trait ByteArrayExt: Sized {
    fn to_byte_array(self) -> [u8; 32];
    fn from_byte_array(bytes: [u8; 32]) -> Self;
}

impl ByteArrayExt for [u8; 32] {
    #[inline]
    fn to_byte_array(self) -> [u8; 32] {
        self
    }
    #[inline]
    fn from_byte_array(bytes: [u8; 32]) -> Self {
        bytes
    }
}

impl ByteArrayExt for super::amr::Amr {
    #[inline]
    fn to_byte_array(self) -> [u8; 32] {
        self.to_byte_array()
    }
    #[inline]
    fn from_byte_array(bytes: [u8; 32]) -> Self {
        Self::from_byte_array(bytes)
    }
}

impl ByteArrayExt for super::cmr::Cmr {
    #[inline]
    fn to_byte_array(self) -> [u8; 32] {
        self.to_byte_array()
    }
    #[inline]
    fn from_byte_array(bytes: [u8; 32]) -> Self {
        Self::from_byte_array(bytes)
    }
}

impl ByteArrayExt for super::ihr::Ihr {
    #[inline]
    fn to_byte_array(self) -> [u8; 32] {
        self.to_byte_array()
    }
    #[inline]
    fn from_byte_array(bytes: [u8; 32]) -> Self {
        Self::from_byte_array(bytes)
    }
}

impl ByteArrayExt for super::ihr::Imr {
    #[inline]
    fn to_byte_array(self) -> [u8; 32] {
        self.to_byte_array()
    }
    #[inline]
    fn from_byte_array(bytes: [u8; 32]) -> Self {
        Self::from_byte_array(bytes)
    }
}

impl ByteArrayExt for super::tmr::Tmr {
    #[inline]
    fn to_byte_array(self) -> [u8; 32] {
        self.to_byte_array()
    }
    #[inline]
    fn from_byte_array(bytes: [u8; 32]) -> Self {
        Self::from_byte_array(bytes)
    }
}

/// Private extension trait to add methods to the `sha256::Midstate` type.
pub trait MidstateExt: Sized {
    /// Input two 32-byte blocks to the midstate to produce a new midstate.
    ///
    /// This one should be upstreamed (at least in the non-generic form which
    /// just takes a pair of `[u8; 32]`s).
    ///
    /// `zz_` prefix is so that when/if we upstream an inherent method called `update_2x32`, it
    /// won't break our code.
    fn zz_update_2x32<MR1: ByteArrayExt, MR2: ByteArrayExt>(self, left: MR1, right: MR2) -> Self;

    /// Input one 64-byte block to the midstate to produce a new midstate.
    ///
    /// This one should be upstreamed.
    ///
    /// `zz_` prefix is so that when/if we upstream an inherent method called `update_64`, it
    /// won't break our code.
    fn zz_update_64(self, bytes: [u8; 64]) -> Self;

    /// Input a 24-byte block of zeros, a big-endian 64-bit number, then a 32-byte block to produce
    /// a new midstate.
    #[inline]
    fn update_weight_then_32<MR: ByteArrayExt>(self, left_weight: u64, right: MR) -> Self {
        let mut left = [0; 32];
        left[24..].copy_from_slice(&left_weight.to_be_bytes());
        self.zz_update_2x32(left, right)
    }

    /// Input a 32-byte block of 0s, then the given 32-byte block, to produce a new midstate.
    #[inline]
    fn update_0_then_32<MR: ByteArrayExt>(self, bytes: MR) -> Self {
        self.zz_update_2x32([0; 32], bytes)
    }

    /// Reinterpret the bytes of the midstate as an AMR.
    fn into_merkle_root<MR: ByteArrayExt>(self) -> MR;
}

impl MidstateExt for Midstate {
    #[inline]
    fn zz_update_2x32<MR1: ByteArrayExt, MR2: ByteArrayExt>(self, left: MR1, right: MR2) -> Self {
        let mut engine = HashEngine::from_midstate(self);
        engine.input(&left.to_byte_array());
        engine.input(&right.to_byte_array());
        engine.midstate().expect("hashed exactly 64 bytes")
    }

    #[inline]
    fn zz_update_64(self, bytes: [u8; 64]) -> Self {
        let mut engine = HashEngine::from_midstate(self);
        engine.input(bytes.as_ref());
        engine.midstate().expect("hashed exactly 64 bytes")
    }

    /// Reinterpret the bytes of the midstate as an AMR.
    #[inline]
    fn into_merkle_root<MR: ByteArrayExt>(self) -> MR {
        MR::from_byte_array(self.to_parts().0)
    }
}
