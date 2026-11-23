//! JS bindings for `packages/core/src/text.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `diffText`.
fn diff_text_inner(before: String, after: String) -> Result<String, String> {
    let out = port::diff_text(before, after);
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "diffText")]
pub fn diff_text(before: String, after: String) -> Result<String, JsError> {
    diff_text_inner(before, after).map_err(|e| JsError::new(&e))
}

/// `applyTextEdits`.
fn apply_text_edits_inner(text: String, edits: String) -> Result<String, String> {
    let edits = crate::js::from_json(&edits)?;
    let out = port::apply_text_edits(text, edits)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "applyTextEdits")]
pub fn apply_text_edits(text: String, edits: String) -> Result<String, JsError> {
    apply_text_edits_inner(text, edits).map_err(|e| JsError::new(&e))
}

/// `rangeContains`.
fn range_contains_inner(range: String, offset: f64) -> Result<bool, String> {
    let range = crate::js::from_json(&range)?;
    let out = port::range_contains(range, offset)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "rangeContains")]
pub fn range_contains(range: String, offset: f64) -> Result<bool, JsError> {
    range_contains_inner(range, offset).map_err(|e| JsError::new(&e))
}

/// `hashText`.
fn hash_text_inner(text: String) -> Result<String, String> {
    let out = port::hash_text(text);
    Ok(out)
}

#[wasm_bindgen(js_name = "hashText")]
pub fn hash_text(text: String) -> Result<String, JsError> {
    hash_text_inner(text).map_err(|e| JsError::new(&e))
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

static S_DIFF_TEXT_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    (
        "range",
        wire::Schema::Obj(&[("start", wire::Schema::Num), ("end", wire::Schema::Num)]),
    ),
    ("text", wire::Schema::Str16),
]));

/// `diffText` over the binary wire.
#[wasm_bindgen(js_name = "diffTextWire")]
pub fn diff_text_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let before: String = r.string16()?;
        let after: String = r.string16()?;
        let out = &port::diff_text(before, after);
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_DIFF_TEXT_OUT.inner())?;
        }
        Ok(())
    })
}

static S_APPLY_TEXT_EDITS_1: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    (
        "range",
        wire::Schema::Obj(&[("start", wire::Schema::Num), ("end", wire::Schema::Num)]),
    ),
    ("text", wire::Schema::Str16),
]));
static S_APPLY_TEXT_EDITS_OUT: wire::Schema = wire::Schema::Str16;

/// `applyTextEdits` over the binary wire.
#[wasm_bindgen(js_name = "applyTextEditsWire")]
pub fn apply_text_edits_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let edits: Vec<Value> = r.list(|r| wire::read_value(r, S_APPLY_TEXT_EDITS_1.inner()))?;
        let out = &port::apply_text_edits(text, edits)?;
        wire::write_value(w, out, &S_APPLY_TEXT_EDITS_OUT)?;
        Ok(())
    })
}

static S_RANGE_CONTAINS_0: wire::Schema =
    wire::Schema::Obj(&[("start", wire::Schema::Num), ("end", wire::Schema::Num)]);

/// `rangeContains` over the binary wire.
#[wasm_bindgen(js_name = "rangeContainsWire")]
pub fn range_contains_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let range: Value = wire::read_value(r, &S_RANGE_CONTAINS_0)?;
        let offset: f64 = r.f64()?;
        let out = &port::range_contains(range, offset)?;
        w.bool(*out);
        Ok(())
    })
}
