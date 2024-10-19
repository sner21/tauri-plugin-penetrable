use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongA, SetWindowLongA};
use windows::{
    core::{
        PCWSTR,
        HSTRING
    },
    Win32::
        UI::WindowsAndMessaging::{
            FindWindowW,
            GWL_EXSTYLE,
        },
};

use tauri::{
    Runtime,
    window::Window,
};

pub fn penetrable<R:Runtime>(window:Window<R>) {
    #[cfg(target_os = "windows")]
    unsafe {
        let title = window.title().unwrap();
        let hwnd = FindWindowW(None,  PCWSTR(HSTRING::from(title.as_str()).as_ptr()));
        let extended_style = GetWindowLongA(hwnd, GWL_EXSTYLE);
        let style = extended_style | 32u32 as i32| 524288u32  as i32;
        SetWindowLongA(hwnd, GWL_EXSTYLE, style);
    }
}
