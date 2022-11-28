//! High level APIs to creating read/write CFrames
//!

use super::frame_ffi;
use super::frame_ffi::CFrameItem;

impl CFrameItem {
    /// Allocate a new frame item with dummy values
    unsafe fn new_unchecked() -> Self {
        Self {
            edge: std::ptr::null(),
            len: 0,
        }
    }

    /// Initialize a new read frame.
    /// 'n' is the number of cells for the read frame.
    /// 'from' is a pointer to the beginning of the new slice for the array of u8 to hold the frame's cells.
    ///
    /// Note: The C implementation uses array of UWORD for `from`. UWORD maps to uint_fast16_t which
    /// maps to usize on both 32-bit and 64-bit platforms.
    pub unsafe fn new_read(n: usize, from: *const usize) -> Self {
        // Allocate a new vector of required size
        let mut frame = CFrameItem::new_unchecked();
        frame_ffi::c_initReadFrame(&mut frame, n, from);
        frame
    }

    /// Initialize a new write frame.
    /// 'n' is the number of cells for the write frame.
    /// 'from' is a pointer to the one-past-the-end of the new slice for the array of UWORDS to hold the frame's cells.
    pub unsafe fn new_write(n: usize, from: *mut usize) -> Self {
        // Allocate a new vector of required size
        let mut frame = CFrameItem::new_unchecked();
        frame_ffi::c_initWriteFrame(&mut frame, n, from);
        frame
    }
}

// Number of uwords required to hold n bits
pub fn round_u_word(n: usize) -> usize {
    unsafe {
        (n + 8 * frame_ffi::c_sizeof_UWORD - 1) as usize / (8 * frame_ffi::c_sizeof_UWORD as usize)
    }
}

pub fn ffi_bytes_size(n: usize) -> usize {
    unsafe { round_u_word(n) * frame_ffi::c_sizeof_UWORD }
}
