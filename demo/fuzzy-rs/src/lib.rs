//! JS bindings for `packages/core/src/fuzzy.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod port;
pub mod utf16;

/// `fuzzyScore`.
fn fuzzy_score_inner(query: String, text: String) -> Result<f64, String> {
    let out = port::fuzzy_score(query, text);
    Ok(out)
}

#[wasm_bindgen(js_name = "fuzzyScore")]
pub fn fuzzy_score(query: String, text: String) -> Result<f64, JsError> {
    fuzzy_score_inner(query, text).map_err(|e| JsError::new(&e))
}
