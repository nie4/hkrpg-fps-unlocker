use std::{sync::LazyLock, time::Duration};
use windows::core::s;
use windows::Win32::System::{LibraryLoader::GetModuleHandleA, SystemServices::DLL_PROCESS_ATTACH};

static UNITYPLAYER: LazyLock<usize> = LazyLock::new(|| unsafe {
    loop {
        match GetModuleHandleA(s!("UnityPlayer.dll")) {
            Ok(handle) => break handle.0 as usize,
            Err(_) => std::thread::sleep(Duration::from_millis(100)),
        }
    }
});

const OFFSET: usize = 0x1C9A8D0;

// Change this value
const TARGET_FRAMERATE: i32 = 144;

unsafe fn main_thread() {
    let unity_player = &*UNITYPLAYER;

    let fps_value = unity_player.wrapping_add(OFFSET) as *mut i32;

    loop {
        std::thread::sleep(Duration::from_secs(5));
        *fps_value = TARGET_FRAMERATE;
    }
}

#[no_mangle]
unsafe extern "system" fn DllMain(_: *const u8, call_reason: u32, _: *const u8) -> u32 {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(|| main_thread());
    }

    1
}
