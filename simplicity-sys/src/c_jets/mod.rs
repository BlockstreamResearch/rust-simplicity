//! FFI related to jets
// Typically, the only things in the sys crate are the
// FFI bindings. The high level code should be in the
// main crate.
// We don't follow this practice in the library of as now

pub mod c_env;
pub mod c_frame;
pub mod frame_ffi;
pub mod jets_ffi;
pub mod jets_wrapper;

pub use c_env::{CElementsTxEnv, CTapEnv, CTransaction};
pub use c_frame::{ffi_bytes_size, round_u_word};
pub use frame_ffi::CFrameItem;

use crate::c_jets::c_env::{CRawBuffer, CRawInput, CRawOutput, CRawTransaction};

/// sanity checks for using the types.
/// We are not using the internal representation of the types at all, but
/// we do care about the size of the types.
///
/// This will also us detect whenever there is a change in the underlying C representation
pub fn sanity_checks() -> bool {
    unsafe {
        if std::mem::size_of::<CFrameItem>() != frame_ffi::c_sizeof_frameItem {
            return false;
        }
        if std::mem::size_of::<CRawBuffer>() != c_env::c_sizeof_rawBuffer {
            return false;
        }
        if std::mem::size_of::<CRawInput>() != c_env::c_sizeof_rawInput {
            return false;
        }
        if std::mem::size_of::<CRawOutput>() != c_env::c_sizeof_rawOutput {
            return false;
        }
        if std::mem::size_of::<CRawTransaction>() != c_env::c_sizeof_rawTransaction {
            return false;
        }
        if std::mem::size_of::<CElementsTxEnv>() != c_env::c_sizeof_txEnv {
            return false;
        }
    }
    true
}

#[derive(Debug)]
pub enum CTxEnv<'a> {
    Null,
    ElementsTxEnv(&'a CElementsTxEnv),
}

impl CTxEnv<'_> {
    /// Unwraps elements tx env
    pub fn unwrap_elements_tx_env(&self) -> *const CElementsTxEnv {
        match self {
            CTxEnv::Null => panic!("ElementsEnv must not be null"),
            CTxEnv::ElementsTxEnv(env) => *env,
        }
    }
}
