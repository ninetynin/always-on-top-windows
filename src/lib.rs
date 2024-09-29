use std::ptr::null_mut;
use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, HWND_NOTOPMOST, SWP_NOMOVE, SWP_NOSIZE, GetForegroundWindow};


/// Sets the specified window as always on top or not.
///
/// # Arguments
///
/// * `window_handle` - The handle of the window to modify. It should be provided as an `isize`.
/// * `always_on_top` - A boolean indicating whether to set the window as always on top (`true`) or not (`false`).
///
/// # Returns
///
/// Returns `true` if the operation was successful, or `false` otherwise.
///
/// # Safety
///
/// This function is unsafe because it interacts with raw pointers and Windows API, which can cause undefined behavior if misused.
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

/// Retrieves the handle of the currently active (foreground) window.
///
/// # Returns
///
/// Returns the handle of the current foreground window as an `isize`.
///
/// # Safety
///
/// This function is unsafe as it relies on the Windows API, which can return null or invalid pointers.
pub fn get_current_window_handle() -> isize {
    unsafe {
        GetForegroundWindow() as isize
    }
}


/// Disables the always on top setting for the specified window.
///
/// # Arguments
///
/// * `window_handle` - The handle of the window to modify. It should be provided as an `isize`.
///
/// # Returns
///
/// Returns `true` if the operation was successful, or `false` otherwise.
///
/// # Safety
///
/// This function is unsafe because it modifies window properties through raw pointers and the Windows API.
pub fn disable_always_on_top(window_handle: isize) -> bool {
    set_always_on_top(window_handle, false)
}