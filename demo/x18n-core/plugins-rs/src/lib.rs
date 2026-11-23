//! JS bindings for `packages/core/src/plugins.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;
pub mod utf16;

/// `extensionOf`.
fn extension_of_inner(path: String) -> Result<String, String> {
    let out = port::extension_of(path);
    Ok(out)
}

#[wasm_bindgen(js_name = "extensionOf")]
pub fn extension_of(path: String) -> Result<String, JsError> {
    extension_of_inner(path).map_err(|e| JsError::new(&e))
}
