use ctor::ctor;
use std::time::Duration;
use windows::core::s;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

const OFFSET: usize = 0x1CE8610;

// Change this value
const TARGET_FRAMERATE: i32 = 144;

unsafe fn unlocker_thread() {
    let unity_player = loop {
        match GetModuleHandleA(s!("UnityPlayer.dll")) {
            Ok(handle) => break handle.0 as usize,
            Err(_) => std::thread::sleep(Duration::from_millis(100)),
        }
    };

    let fps_value = unity_player.wrapping_add(OFFSET) as *mut i32;

    loop {
        std::thread::sleep(Duration::from_secs(5));
        *fps_value = TARGET_FRAMERATE;
    }
}

#[ctor]
unsafe fn main() {
    std::thread::spawn(|| unlocker_thread());
}
