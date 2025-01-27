use functions::{set_target_frame_rate, set_vsync_count};
use std::{sync::LazyLock, time::Duration};
use windows::core::s;
use windows::Win32::System::{LibraryLoader::GetModuleHandleA, SystemServices::DLL_PROCESS_ATTACH};

mod functions;
mod offsets;

pub static GAMEASSEMBLY_BASE: LazyLock<usize> = LazyLock::new(|| unsafe {
    loop {
        match GetModuleHandleA(s!("GameAssembly.dll")) {
            Ok(handle) => break handle.0 as usize,
            Err(_) => std::thread::sleep(Duration::from_millis(100)),
        }
    }
});

// Change these values
const VSYNC_COUNT: i32 = 0;
const TARGET_FRAMERATE: i32 = 144;

unsafe fn main_thread() {
    let _ = &*GAMEASSEMBLY_BASE;

    loop {
        std::thread::sleep(Duration::from_secs(5));
        set_vsync_count(VSYNC_COUNT);

        if VSYNC_COUNT == 0 {
            set_target_frame_rate(TARGET_FRAMERATE);
        }
    }
}

#[no_mangle]
unsafe extern "system" fn DllMain(_: *const u8, call_reason: u32, _: *const u8) -> u32 {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(|| main_thread());
    }

    1
}
