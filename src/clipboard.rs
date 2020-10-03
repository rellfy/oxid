//! OS clipboard abstraction

use crate::Context;

#[cfg(target_os = "linux")]
mod linux {
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

#[cfg(not(any(target_os = "linux", target_arch = "wasm32")))]
mod dummy {
    use crate::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        None
    }

    pub fn set(_ctx: &mut Context, _data: &str) {}
}

#[cfg(not(any(target_os = "linux", target_arch = "wasm32")))]
use dummy as clipboard;
#[cfg(target_arch = "wasm32")]
use wasm as clipboard;

#[cfg(target_os = "linux")]
use linux as clipboard;

/// Get current OS clipboard value
pub fn get(ctx: &mut Context) -> Option<String> {
    clipboard::get(ctx)
}

/// Save value to OS clipboard
pub fn set(ctx: &mut Context, data: &str) {
    clipboard::set(ctx, data);
}
