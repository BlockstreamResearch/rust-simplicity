// SPDX-License-Identifier: CC0-1.0

//! FFI related to jets
// Typically, the only things in the sys crate are the
// FFI bindings. The high level code should be in the
// main crate.
// We don't follow this practice in the library of as now

pub mod c_env;
pub mod c_frame;
pub mod frame_ffi;
#[rustfmt::skip] pub mod jets_ffi;
#[rustfmt::skip] pub mod jets_wrapper;

pub use c_env::{CElementsTxEnv, CTapEnv, CTransaction};
pub use c_frame::{byte_width, uword_width};
pub use frame_ffi::CFrameItem;

// The bindings use elements_ffi instead of jets_ffi.
pub use jets_ffi as elements_ffi;

use crate::c_jets::c_env::{CRawBuffer, CRawInput, CRawOutput, CRawTapEnv, CRawTransaction};

#[cfg(feature = "test-utils")]
pub mod exec_ffi;

/// sanity checks for using the types.
/// We are not using the internal representation of the types at all, but
/// we do care about the size and alignments of the types.
///
/// This will also us detect whenever there is a change in the underlying C representation
pub fn sanity_checks() -> bool {
    unsafe {
        if std::mem::size_of::<crate::ffi::UWORD>() != crate::ffi::c_sizeof_UWORD
            || std::mem::align_of::<crate::ffi::UWORD>() != crate::ffi::c_alignof_UWORD
        {
            return false;
        }

        if std::mem::size_of::<CFrameItem>() != frame_ffi::c_sizeof_frameItem
            || std::mem::size_of::<CRawBuffer>() != c_env::c_sizeof_rawBuffer
            || std::mem::size_of::<CRawInput>() != c_env::c_sizeof_rawInput
            || std::mem::size_of::<CRawOutput>() != c_env::c_sizeof_rawOutput
            || std::mem::size_of::<CRawTransaction>() != c_env::c_sizeof_rawTransaction
            || std::mem::size_of::<CElementsTxEnv>() != c_env::c_sizeof_txEnv
            || std::mem::size_of::<CRawTapEnv>() != c_env::c_sizeof_rawTapEnv
        {
            return false;
        }

        if std::mem::align_of::<CFrameItem>() != frame_ffi::c_alignof_frameItem
            || std::mem::align_of::<CRawBuffer>() != c_env::c_alignof_rawBuffer
            || std::mem::align_of::<CRawInput>() != c_env::c_alignof_rawInput
            || std::mem::align_of::<CRawOutput>() != c_env::c_alignof_rawOutput
            || std::mem::align_of::<CRawTransaction>() != c_env::c_alignof_rawTransaction
            || std::mem::align_of::<CElementsTxEnv>() != c_env::c_alignof_txEnv
            || std::mem::align_of::<CRawTapEnv>() != c_env::c_alignof_rawTapEnv
        {
            return false;
        }
    }
    true
}
