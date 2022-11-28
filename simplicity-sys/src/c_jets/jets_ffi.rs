use super::c_env::CTxEnv;
use super::frame_ffi::CFrameItem;

extern "C" {
    pub fn add_32(dst: &mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
    pub fn lock_time(dst: &mut CFrameItem, src: CFrameItem, env: *const CTxEnv) -> bool;
}
