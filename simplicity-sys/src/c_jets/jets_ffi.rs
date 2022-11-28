use std::os::raw::c_void;

use super::c_env::CElementsTxEnv;
use super::frame_ffi::CFrameItem;

extern "C" {
    pub fn add_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn lock_time(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
}
