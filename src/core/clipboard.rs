//! Clipboard abstraction
use crate::core::Context;

mod clipboard {
    use crate::core::Context;

    pub fn get(_ctx: &mut Context) -> Option<String> {
        crate::wasm::clipboard_get()
    }

    pub fn set(_ctx: &mut Context, data: &str) {
        crate::wasm::clipboard_set(data);
    }
}

/// Get current clipboard value
pub fn get(ctx: &mut Context) -> Option<String> {
    clipboard::get(ctx)
}

/// Save value clipboard
pub fn set(ctx: &mut Context, data: &str) {
    clipboard::set(ctx, data);
}
