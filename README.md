# Simple winAPI lib

This project provides functions to trace and retrieve information about windows in the Windows operating system.

## Functions

### `get_focus_hwnd() -> winapi::shared::windef::HWND`

This function retrieves the handle of the currently focused window.

---

### `get_title(hwnd: winapi::shared::windef::HWND) -> String`

This function retrieves the title of a window specified by its handle.

---

### `get_rect(hwnd: winapi::shared::windef::HWND) -> RECT`

This function retrieves the rectangle coordinates of a window specified by its handle.

---

### `get_process_id(hwnd: winapi::shared::windef::HWND) -> u32`

This function retrieves the process ID of a window specified by its handle.

---

### `get_class_name(hwnd: winapi::shared::windef::HWND) -> String`

This function retrieves the class name of a window specified by its handle.

---

### `get_process_path(hwnd: winapi::shared::windef::HWND) -> String`

This function retrieves the path of the process associated with a window specified by its handle.

---

Please refer to the documentation of each function for more details on their usage and return values.
