use std::ptr::null_mut;
use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOMOVE, SWP_NOSIZE, GetForegroundWindow};

pub fn set_always_on_top(window_handle: isize, always_on_top: bool) -> bool {
    unsafe {
        let hwnd = window_handle as *mut _;
        let hwnd_insert_after = if always_on_top { HWND_TOPMOST } else { HWND_NOTOPMOST };
        
        SetWindowPos(
            hwnd,
            hwnd_insert_after,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE
        ) != 0
    }
}

pub fn get_current_window_handle() -> isize {
    unsafe {
        GetForegroundWindow() as isize
    }
}

pub fn disable_always_on_top(window_handle: isize) -> bool {
    set_always_on_top(window_handle, false)
}