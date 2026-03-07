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

pub use c_env::elements;
pub use c_frame::{byte_width, uword_width};
pub use frame_ffi::CFrameItem;

// The bindings use elements_ffi instead of jets_ffi.
pub use jets_ffi as elements_ffi;

#[cfg(feature = "test-utils")]
pub mod exec_ffi;

/// sanity checks for using the types.
/// We are not using the internal representation of the types at all, but
/// we do care about the size and alignments of the types.
///
/// This will also us detect whenever there is a change in the underlying C representation
pub fn sanity_checks() -> bool {
    sanity_check_report().is_empty()
}

#[must_use]
pub fn sanity_check_report() -> Vec<String> {
    let mut issues = Vec::new();

    macro_rules! check_layout {
        ($ty:ty, $c_size:path, $c_align:path, $name:expr) => {{
            let rust_size = std::mem::size_of::<$ty>();
            let rust_align = std::mem::align_of::<$ty>();
            let (c_size, c_align) = unsafe { ($c_size, $c_align) };

            if rust_size != c_size {
                issues.push(format!(
                    "{name}: size mismatch (rust={rust_size}, c={c_size})",
                    name = $name
                ));
            }

            if rust_align != c_align {
                issues.push(format!(
                    "{name}: align mismatch (rust={rust_align}, c={c_align})",
                    name = $name
                ));
            }
        }};
    }

    check_layout!(
        crate::ffi::UWORD,
        crate::ffi::c_sizeof_UWORD,
        crate::ffi::c_alignof_UWORD,
        "UWORD"
    );
    check_layout!(
        CFrameItem,
        frame_ffi::c_sizeof_frameItem,
        frame_ffi::c_alignof_frameItem,
        "CFrameItem"
    );
    check_layout!(
        elements::CRawBuffer,
        elements::c_sizeof_rawBuffer,
        elements::c_alignof_rawBuffer,
        "CRawBuffer"
    );
    check_layout!(
        elements::CRawInput,
        elements::c_sizeof_rawInput,
        elements::c_alignof_rawInput,
        "CRawInput"
    );
    check_layout!(
        elements::CRawOutput,
        elements::c_sizeof_rawOutput,
        elements::c_alignof_rawOutput,
        "CRawOutput"
    );
    check_layout!(
        elements::CRawTransaction,
        elements::c_sizeof_rawTransaction,
        elements::c_alignof_rawTransaction,
        "CRawTransaction"
    );
    check_layout!(
        elements::CTxEnv,
        elements::c_sizeof_txEnv,
        elements::c_alignof_txEnv,
        "CTxEnv"
    );
    check_layout!(
        elements::CRawTapEnv,
        elements::c_sizeof_rawTapEnv,
        elements::c_alignof_rawTapEnv,
        "CRawTapEnv"
    );

    issues
}
