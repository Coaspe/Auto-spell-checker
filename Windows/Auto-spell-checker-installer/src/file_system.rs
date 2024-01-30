use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::{ptr, slice};
use windows_sys::Win32::Foundation::S_OK;
use windows_sys::Win32::UI::Shell::{FOLDERID_Desktop, SHGetKnownFolderPath};

/// Retrieves the path to the user's desktop folder.
///
/// # Returns
///
/// - `Some(OsString)`: The path to the desktop folder if it was successfully retrieved.
/// - `None`: If the path to the desktop folder could not be retrieved.
pub fn get_desktop_path() -> Option<OsString> {
    unsafe {
        let mut desktop_path_ptr = ptr::null_mut();

        let result = SHGetKnownFolderPath(&FOLDERID_Desktop, 0, 0, &mut desktop_path_ptr);

        extern "C" {
            /// ```C
            /// #include <stddef.h> // for size_t
            ///
            ///     const wchar_t *p = str;
            ///     while (*p != L'\0') {
            ///         ++p;
            ///     }
            ///     return p - str;
            /// }
            /// ``
            fn wcslen(buf: *const u16) -> usize;
        }

        if result == S_OK {
            let slice_data_inner =
                slice::from_raw_parts(desktop_path_ptr, wcslen(desktop_path_ptr));
            let s = OsString::from_wide(&slice_data_inner);
            return Some(s);
        }
        return None;
    }
}
