// SPDX-License-Identifier: CC0-1.0

//! Frame related FFI bindings and data structures

use crate::ffi::{c_size_t, c_uchar, UWORD};

/// FrameItem for C FFI.
/// This type is only used to pass data to the C FFI.
#[derive(Debug)]
#[repr(C)]
pub struct CFrameItem {
    pub edge: *const c_uchar,
    pub len: c_size_t,
}

extern "C" {
    pub static c_sizeof_frameItem: c_size_t;
    pub static c_alignof_frameItem: c_size_t;

    pub(crate) fn c_initWriteFrame(frame: &mut CFrameItem, n: c_size_t, from: *const UWORD);
    pub(crate) fn c_initReadFrame(frame: &mut CFrameItem, n: c_size_t, from: *const UWORD);

    pub fn c_readBit(frame: &mut CFrameItem) -> bool;
    pub fn c_writeBit(frame: &mut CFrameItem, bit: bool);
    pub fn c_forwardBits(frame: &mut CFrameItem, n: usize);
    pub fn c_skipBits(frame: &mut CFrameItem, n: usize);
}
