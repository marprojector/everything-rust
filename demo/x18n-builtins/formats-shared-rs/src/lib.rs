//! JS bindings for `packages/builtins/src/formats/shared/`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `indentUnit`.
fn indent_unit_inner(text: String, indent: String, fallback: String) -> Result<String, String> {
    let indent = crate::js::from_json(&indent)?;
    let out = port::indent_unit(text, indent, fallback)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "indentUnit")]
pub fn indent_unit(text: String, indent: String, fallback: String) -> Result<String, JsError> {
    indent_unit_inner(text, indent, fallback).map_err(|e| JsError::new(&e))
}

/// `detectEol`.
fn detect_eol_inner(text: String) -> Result<String, String> {
    let out = port::detect_eol(text)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "detectEol")]
pub fn detect_eol(text: String) -> Result<String, JsError> {
    detect_eol_inner(text).map_err(|e| JsError::new(&e))
}

/// `leafText`.
fn leaf_text_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::leaf_text(value)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "leafText")]
pub fn leaf_text(value: String) -> Result<String, JsError> {
    leaf_text_inner(value).map_err(|e| JsError::new(&e))
}

/// `splice`.
fn splice_inner(text: String, range: String, insert: String) -> Result<String, String> {
    let range = crate::js::from_json(&range)?;
    let out = port::splice(text, range, insert)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "splice")]
pub fn splice(text: String, range: String, insert: String) -> Result<String, JsError> {
    splice_inner(text, range, insert).map_err(|e| JsError::new(&e))
}

/// `appendLines`.
fn append_lines_inner(text: String, chunk: String) -> Result<String, String> {
    let out = port::append_lines(text, chunk)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "appendLines")]
pub fn append_lines(text: String, chunk: String) -> Result<String, JsError> {
    append_lines_inner(text, chunk).map_err(|e| JsError::new(&e))
}

/// `splitLines`.
fn split_lines_inner(text: String) -> Result<String, String> {
    let out = port::split_lines(text)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "splitLines")]
pub fn split_lines(text: String) -> Result<String, JsError> {
    split_lines_inner(text).map_err(|e| JsError::new(&e))
}

/// `withBlankLines`.
fn with_blank_lines_inner(text: String, range: String) -> Result<String, String> {
    let range = crate::js::from_json(&range)?;
    let out = port::with_blank_lines(text, range)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "withBlankLines")]
pub fn with_blank_lines(text: String, range: String) -> Result<String, JsError> {
    with_blank_lines_inner(text, range).map_err(|e| JsError::new(&e))
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

static S_INDENT_UNIT_1: wire::Schema = wire::Schema::Dyn;

/// `indentUnit` over the binary wire.
#[wasm_bindgen(js_name = "indentUnitWire")]
pub fn indent_unit_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let indent: Value = wire::read_value(r, &S_INDENT_UNIT_1)?;
        let fallback: String = r.string16()?;
        let out = &port::indent_unit(text, indent, fallback)?;
        w.string16(out);
        Ok(())
    })
}

static S_LEAF_TEXT_0: wire::Schema = wire::Schema::Dyn;
static S_LEAF_TEXT_OUT: wire::Schema = wire::Schema::Str16;

/// `leafText` over the binary wire.
#[wasm_bindgen(js_name = "leafTextWire")]
pub fn leaf_text_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_LEAF_TEXT_0)?;
        let out = &port::leaf_text(value)?;
        wire::write_value(w, out, &S_LEAF_TEXT_OUT)?;
        Ok(())
    })
}

static S_SPLICE_1: wire::Schema =
    wire::Schema::Obj(&[("start", wire::Schema::Num), ("end", wire::Schema::Num)]);
static S_SPLICE_OUT: wire::Schema = wire::Schema::Str16;

/// `splice` over the binary wire.
#[wasm_bindgen(js_name = "spliceWire")]
pub fn splice_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let range: Value = wire::read_value(r, &S_SPLICE_1)?;
        let insert: String = r.string16()?;
        let out = &port::splice(text, range, insert)?;
        wire::write_value(w, out, &S_SPLICE_OUT)?;
        Ok(())
    })
}

static S_SPLIT_LINES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("start", wire::Schema::Num),
    ("end", wire::Schema::Num),
    ("next", wire::Schema::Num),
]));

/// `splitLines` over the binary wire.
#[wasm_bindgen(js_name = "splitLinesWire")]
pub fn split_lines_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::split_lines(text)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SPLIT_LINES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_WITH_BLANK_LINES_1: wire::Schema =
    wire::Schema::Obj(&[("start", wire::Schema::Num), ("end", wire::Schema::Num)]);
static S_WITH_BLANK_LINES_OUT: wire::Schema =
    wire::Schema::Obj(&[("start", wire::Schema::Num), ("end", wire::Schema::Num)]);

/// `withBlankLines` over the binary wire.
#[wasm_bindgen(js_name = "withBlankLinesWire")]
pub fn with_blank_lines_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let range: Value = wire::read_value(r, &S_WITH_BLANK_LINES_1)?;
        let out = &port::with_blank_lines(text, range)?;
        wire::write_value(w, out, &S_WITH_BLANK_LINES_OUT)?;
        Ok(())
    })
}
