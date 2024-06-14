use winapi::{
    shared::windef::HWND,
    um::{
        winnt::{LONG, LPCSTR},
        winuser::{
            {
                FindWindowA,
                GetWindowLongA,
                SetWindowLongA,
                GWL_EXSTYLE,
                WS_EX_TRANSPARENT,
                WS_EX_LAYERED,
                WS_EX_TOOLWINDOW,
                WS_EX_APPWINDOW
            }
        }
    },
    ctypes::{__int32},
};
use std::ptr::null_mut;
use std::ffi::CString;
use tauri::{
    Runtime,
    window::Window,
};

pub fn penetrable<R:Runtime>(window:Window<R>) {
    #[cfg(target_os = "windows")]
    unsafe {
        let title = window.title().unwrap();
        let str = CString::new(title).unwrap().as_ptr();
        let h: HWND = FindWindowA(null_mut(), str as LPCSTR);
        let extended_style: __int32 = GetWindowLongA(h, GWL_EXSTYLE);
        let style = extended_style | WS_EX_TRANSPARENT as LONG | WS_EX_LAYERED as LONG | WS_EX_TOOLWINDOW as LONG | WS_EX_APPWINDOW as LONG;
        SetWindowLongA(h, GWL_EXSTYLE, style);
    }
}
