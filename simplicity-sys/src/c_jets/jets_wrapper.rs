use crate::CElementsTxEnv;

use super::{frame_ffi::CFrameItem, jets_ffi};

pub fn add_32(dst: &mut CFrameItem, src: CFrameItem, _env: &()) -> bool {
    unsafe { jets_ffi::add_32(dst, src, std::ptr::null()) }
}

pub fn lock_time(dst: &mut CFrameItem, src: CFrameItem, env: &CElementsTxEnv) -> bool {
    unsafe { jets_ffi::lock_time(dst, src, env) }
}
