use ctor::ctor;
use std::{thread, time::Duration};
use windows::{Win32::System::LibraryLoader::GetModuleHandleA, core::s};

const OFFSET: usize = 0x1D39750;
const TARGET_FRAMERATE: i32 = 144;

#[ctor]
fn unlocker() {
    thread::spawn(|| {
        let base = loop {
            if let Ok(handle) = unsafe { GetModuleHandleA(s!("UnityPlayer.dll")) } {
                break handle.0 as usize;
            }
            thread::sleep(Duration::from_secs(1));
        };
        let fps_ptr = (base + OFFSET) as *mut i32;
        loop {
            thread::sleep(Duration::from_secs(5));
            unsafe { *fps_ptr = TARGET_FRAMERATE; }
        }
    });
}
