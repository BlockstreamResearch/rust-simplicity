use super::{c_env::CTxEnv, frame_ffi::CFrameItem, jets_ffi};

pub fn add_32(dst: &mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool {
    unsafe { jets_ffi::add_32(dst, src, env) }
}

pub fn lock_time(dst: &mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool {
    unsafe { jets_ffi::lock_time(dst, src, env) }
}
