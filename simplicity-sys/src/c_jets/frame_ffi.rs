//! Frame related FFI bindings and data structures

use std::os::raw::c_uchar;

use libc::size_t;

/// FrameItem for C FFI.
/// This type is only used to pass data to the C FFI.
#[derive(Debug)]
#[repr(C)]
pub struct CFrameItem {
    pub edge: *const c_uchar,
    pub len: size_t,
}

extern "C" {
    pub static c_sizeof_frameItem: size_t;
    pub static c_sizeof_UWORD: size_t;

    pub static c_alignof_frameItem: size_t;
    pub static c_alignof_UWORD: size_t;

    pub(crate) fn c_initWriteFrame(frame: &mut CFrameItem, n: size_t, from: *const size_t);
    pub(crate) fn c_initReadFrame(frame: &mut CFrameItem, n: size_t, from: *const size_t);

    pub fn c_readBit(frame: &mut CFrameItem) -> bool;
    pub fn c_writeBit(frame: &mut CFrameItem, bit: bool);
    pub fn c_forwardBits(frame: &mut CFrameItem, n: usize);
    pub fn c_skipBits(frame: &mut CFrameItem, n: usize);
}
