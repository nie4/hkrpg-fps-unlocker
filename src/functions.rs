use crate::{offsets::{SET_TARGET_FRAME_RATE, SET_VSYNC_COUNT}, GAMEASSEMBLY_BASE};

type SetTargetFrameRate = unsafe extern "fastcall" fn(i32) -> ();
pub unsafe fn set_target_frame_rate(value: i32) {
    let func = std::mem::transmute::<usize, SetTargetFrameRate>(GAMEASSEMBLY_BASE.wrapping_add(SET_TARGET_FRAME_RATE));
    func(value);
}

type SetVsyncCount = unsafe extern "fastcall" fn(i32) -> ();
pub unsafe fn set_vsync_count(value: i32) {
    let func = std::mem::transmute::<usize, SetVsyncCount>(GAMEASSEMBLY_BASE.wrapping_add(SET_VSYNC_COUNT));
    func(value);
}
