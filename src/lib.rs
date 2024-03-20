use std::fmt::Display;

use winapi::um::{
    processthreadsapi::OpenProcess,
    psapi:: GetProcessImageFileNameW,
    winnt::PROCESS_ALL_ACCESS,
    winuser::{
        GetForegroundWindow, GetWindowRect, GetWindowTextW,
        GetWindowThreadProcessId, RealGetWindowClassW,
    },
};


/// Get the handle of the window that has the focus.
pub fn get_focus_hwnd() -> winapi::shared::windef::HWND {
    unsafe { GetForegroundWindow() }
}

/// Get the title of the window.
pub fn get_title(hwnd: winapi::shared::windef::HWND) -> String {
    let mut name: [u16; 256] = [0; 256];
    unsafe {
        GetWindowTextW(hwnd, name.as_mut_ptr() as *mut u16, 256);
    }
    String::from_utf16(&name).unwrap().trim_end_matches('\0').to_string()
}

/// Get the rectangle of the window.
pub fn get_rect(hwnd: winapi::shared::windef::HWND) -> RECT {
    let mut lp_rect = winapi::shared::windef::RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        GetWindowRect(hwnd, &mut lp_rect);
    }
    RECT(lp_rect)
}
 
/// Get the process ID of the window.
pub fn get_process_id(hwnd: winapi::shared::windef::HWND) -> u32 {
    let mut lpdw_process_id: u32 = 0;
    unsafe { GetWindowThreadProcessId(hwnd, &mut lpdw_process_id) };
    lpdw_process_id
}

/// Get the class name of the window.
pub fn get_class_name(hwnd: winapi::shared::windef::HWND) -> String {
    let mut ptsz_class_name: [u16; 256] = [0; 256];
    unsafe {
        RealGetWindowClassW(hwnd, ptsz_class_name.as_mut_ptr(), 256);
    }
    String::from_utf16(&ptsz_class_name).unwrap().trim_end_matches('\0').to_string()
}

/// Get the path of the process.
pub fn get_process_path(hwnd: winapi::shared::windef::HWND) -> String {
    let process_id = get_process_id(hwnd);
    unsafe {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);

        let mut lpsz_file_name: [u16; 256] = [0; 256];
        GetProcessImageFileNameW(handle, lpsz_file_name.as_mut_ptr() as *mut u16, 256);

        String::from_utf16(&lpsz_file_name).unwrap().trim_end_matches('\0').to_string()
    }
}

/// Wrapper struct for the RECT type.
pub struct RECT(winapi::shared::windef::RECT);

impl Display for RECT {
    /// Formats the RECT struct as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rect = self.0;
        write!(
            f,
            "RECT {{ left: {}, top: {}, right: {}, bottom: {} }}",
            rect.left, rect.top, rect.right, rect.bottom
        )
    }
}

#[cfg(test)]
/// This module contains unit tests for the functions in the `super` module.
mod tests {
    use super::*;

    /// Test case for the `get_focus_hwnd` function.
    #[test]
    fn test_get_focus_hwnd() {
        let hwnd = get_focus_hwnd();
        assert_ne!(hwnd, std::ptr::null_mut());
    }

    /// Test case for the `get_title` function.
    #[test]
    fn test_get_title() {
        let hwnd = get_focus_hwnd();
        let title = get_title(hwnd);
        assert!(!title.is_empty());
    }

    /// Test case for the `get_rect` function.
    #[test]
    fn test_get_rect() {
        let hwnd = get_focus_hwnd();
        let rect = get_rect(hwnd);
        assert_ne!(rect.0.left, rect.0.right);
        assert_ne!(rect.0.top, rect.0.bottom);
    }

    /// Test case for the `get_process_id` function.
    #[test]
    fn test_get_process_id() {
        let hwnd = get_focus_hwnd();
        let process_id = get_process_id(hwnd);
        assert_ne!(process_id, 0);
    }

    /// Test case for the `get_class_name` function.
    #[test]
    fn test_get_class_name() {
        let hwnd = get_focus_hwnd();
        let class_name = get_class_name(hwnd);
        assert!(!class_name.is_empty());
    }

    /// Test case for the `get_process_path` function.
    #[test]
    fn test_get_process_path() {
        let hwnd = get_focus_hwnd();
        let process_path = get_process_path(hwnd);
        assert!(!process_path.is_empty());
    }
}
