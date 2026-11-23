//! JS bindings for `packages/core/src/keys.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `joinKey`.
fn join_key_inner(namespace: Option<String>, path: Vec<String>) -> Result<String, String> {
    let out = port::join_key(namespace, path);
    Ok(out)
}

#[wasm_bindgen(js_name = "joinKey")]
pub fn join_key(namespace: Option<String>, path: Vec<String>) -> Result<String, JsError> {
    join_key_inner(namespace, path).map_err(|e| JsError::new(&e))
}

/// `splitKey`.
fn split_key_inner(key: String, namespaced: Option<bool>) -> Result<String, String> {
    let out = port::split_key(key, namespaced);
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "splitKey")]
pub fn split_key(key: String, namespaced: Option<bool>) -> Result<String, JsError> {
    split_key_inner(key, namespaced).map_err(|e| JsError::new(&e))
}

/// `isKeyUnder`.
fn is_key_under_inner(key: String, prefix: String) -> Result<bool, String> {
    let out = port::is_key_under(key, prefix);
    Ok(out)
}

#[wasm_bindgen(js_name = "isKeyUnder")]
pub fn is_key_under(key: String, prefix: String) -> Result<bool, JsError> {
    is_key_under_inner(key, prefix).map_err(|e| JsError::new(&e))
}

/// `replaceKeyPrefix`.
fn replace_key_prefix_inner(key: String, from: String, to: String) -> Result<Vec<u16>, String> {
    let out = port::replace_key_prefix(key, from, to);
    Ok(out)
}

#[wasm_bindgen(js_name = "replaceKeyPrefix")]
pub fn replace_key_prefix(key: String, from: String, to: String) -> Result<Vec<u16>, JsError> {
    replace_key_prefix_inner(key, from, to).map_err(|e| JsError::new(&e))
}

// ---- the binary wire ------------------------------------------------------------------------

thread_local! {
    static WIRE_IN: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    static WIRE_OUT: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// This module's memory, where `index.cjs` reads and writes the wire buffers.
#[wasm_bindgen(js_name = "everythingRustWireMemory")]
pub fn everything_rust_wire_memory() -> JsValue {
    wasm_bindgen::memory()
}

/// The address of an argument buffer of `len` bytes.
#[wasm_bindgen(js_name = "everythingRustWireIn")]
pub fn everything_rust_wire_in(len: u32) -> u32 {
    WIRE_IN.with(|b| {
        let mut b = b.borrow_mut();
        b.resize(len as usize, 0);
        b.as_mut_ptr() as usize as u32
    })
}

/// The address of the last result.
#[wasm_bindgen(js_name = "everythingRustWireOut")]
pub fn everything_rust_wire_out() -> u32 {
    WIRE_OUT.with(|b| b.borrow().as_ptr() as usize as u32)
}

/// Reads `len` argument bytes, runs `f`, and keeps its result for `everythingRustWireOut`.
fn wire_io(
    len: u32,
    f: impl FnOnce(&mut wire::Reader, &mut wire::Writer) -> Result<(), String>,
) -> Result<u32, JsError> {
    let input = WIRE_IN.with(|b| std::mem::take(&mut *b.borrow_mut()));
    let mut w = wire::Writer::default();
    let result = f(
        &mut wire::Reader::new(&input[..(len as usize).min(input.len())]),
        &mut w,
    );
    WIRE_IN.with(|b| *b.borrow_mut() = input);
    match result {
        // A result of another shape than the TS type: JS runs the call through JSON.
        Err(e) if e.starts_with(wire::MISMATCH) => return Ok(u32::MAX),
        other => other.map_err(|e| JsError::new(&e))?,
    }
    let n = w.0.len();
    WIRE_OUT.with(|b| *b.borrow_mut() = w.0);
    Ok(u32::try_from(n).unwrap_or(u32::MAX))
}

static S_SPLIT_KEY_OUT: wire::Schema = wire::Schema::Obj(&[
    ("namespace", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("path", wire::Schema::List(&wire::Schema::Str16)),
]);

/// `splitKey` over the binary wire.
#[wasm_bindgen(js_name = "splitKeyWire")]
pub fn split_key_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let key: String = r.string16()?;
        let namespaced: Option<bool> = r.opt_js(|r| r.bool())?;
        let out = &port::split_key(key, namespaced);
        wire::write_value(w, out, &S_SPLIT_KEY_OUT)?;
        Ok(())
    })
}
