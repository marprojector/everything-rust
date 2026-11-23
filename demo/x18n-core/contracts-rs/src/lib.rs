//! JS bindings for `packages/core/src/contracts.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// `toWireRange`.
fn to_wire_range_inner(text: Option<String>, start: f64, end: f64) -> Result<String, String> {
    let out = port::to_wire_range(text, start, end);
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "toWireRange")]
pub fn to_wire_range(text: Option<String>, start: f64, end: f64) -> Result<String, JsError> {
    to_wire_range_inner(text, start, end).map_err(|e| JsError::new(&e))
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

static S_TO_WIRE_RANGE_OUT: wire::Schema = wire::Schema::Obj(&[
    ("line", wire::Schema::Num),
    ("column", wire::Schema::Num),
    ("endLine", wire::Schema::Num),
    ("endColumn", wire::Schema::Num),
    ("start", wire::Schema::Num),
    ("end", wire::Schema::Num),
]);

/// `toWireRange` over the binary wire.
#[wasm_bindgen(js_name = "toWireRangeWire")]
pub fn to_wire_range_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: Option<String> = r.opt_js(|r| r.string16())?;
        let start: f64 = r.f64()?;
        let end: f64 = r.f64()?;
        let out = &port::to_wire_range(text, start, end);
        wire::write_value(w, out, &S_TO_WIRE_RANGE_OUT)?;
        Ok(())
    })
}
