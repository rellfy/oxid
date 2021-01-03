//! OS clipboard abstraction

use crate::Context;

#[cfg(all(target_os = "linux", feature = "oxid-linux"))]
mod linux_x11 {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        use std::ffi::CString;

        let bufname = CString::new("CLIPBOARD").unwrap();
        let fmtname = CString::new("UTF8_STRING").unwrap();

        unsafe { oxid_linux::clipboard::get_clipboard(bufname.as_ptr(), fmtname.as_ptr()) }
    }

    pub fn set(_ctx: &mut Context, data: &str) {
        use std::ffi::CString;

        let bufname = CString::new("CLIPBOARD").unwrap();

        unsafe {
            oxid_linux::clipboard::claim_clipboard_ownership(bufname.as_ptr(), data.to_owned())
        };
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        oxid_wasm::clipboard_get()
    }

    pub fn set(_ctx: &mut Context, data: &str) {
        oxid_wasm::clipboard_set(data);
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        unsafe { oxid_windows::clipboard::get_clipboard_text() }
    }

    pub fn set(_ctx: &mut Context, data: &str) {
        unsafe { oxid_windows::clipboard::set_clipboard_text(data) };
    }
}

#[cfg(not(any(
all(target_os = "linux", feature = "oxid-linux"),
target_os = "windows",
target_arch = "wasm32"
)))]
mod dummy {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        None
    }

    pub fn set(_ctx: &mut Context, _data: &str) {}
}

#[cfg(not(any(
all(target_os = "linux", feature = "oxid-linux"),
target_os = "windows",
target_arch = "wasm32"
)))]
use dummy as clipboard;
#[cfg(all(target_os = "linux", feature = "oxid-linux"))]
use linux_x11 as clipboard;
#[cfg(target_arch = "wasm32")]
use wasm as clipboard;
#[cfg(target_os = "windows")]
use windows as clipboard;

/// Get current OS clipboard value
pub fn get(ctx: &mut Context) -> Option<String> {
    clipboard::get(ctx)
}

/// Save value to OS clipboard
pub fn set(ctx: &mut Context, data: &str) {
    clipboard::set(ctx, data);
}