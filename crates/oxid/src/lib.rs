extern crate oxid_core as oxid;
pub extern crate oxid_framework as framework;
pub use oxid::*;

pub fn js() -> String {
    format!(
        "{}\n{}",
        include_str!("../../native/wasm/js/gl.js"),
        include_str!("../../native/wasm/js/oxid.js"),
    )
}
