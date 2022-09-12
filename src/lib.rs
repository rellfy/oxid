pub mod core;
pub mod wasm;
pub mod framework;

pub fn js() -> String {
    format!(
        "{}\n{}",
        include_str!("wasm/js/gl.js"),
        include_str!("wasm/js/oxid.js"),
    )
}
