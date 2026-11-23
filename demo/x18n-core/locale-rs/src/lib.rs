//! JS bindings for `packages/core/src/locale.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `resolveFallbacks`.
fn resolve_fallbacks_inner(
    setting: String,
    locales: Vec<String>,
    source_locale: String,
) -> Result<String, String> {
    let setting = crate::js::from_json(&setting)?;
    let out = port::resolve_fallbacks(setting, locales, source_locale)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "resolveFallbacks")]
pub fn resolve_fallbacks(
    setting: String,
    locales: Vec<String>,
    source_locale: String,
) -> Result<String, JsError> {
    resolve_fallbacks_inner(setting, locales, source_locale).map_err(|e| JsError::new(&e))
}

/// `fallbackChain`.
fn fallback_chain_inner(fallbacks: String, locale: String) -> Result<String, String> {
    let fallbacks = crate::js::from_json(&fallbacks)?;
    let out = port::fallback_chain(fallbacks, locale)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "fallbackChain")]
pub fn fallback_chain(fallbacks: String, locale: String) -> Result<String, JsError> {
    fallback_chain_inner(fallbacks, locale).map_err(|e| JsError::new(&e))
}

/// `localeSpellingKey`.
fn locale_spelling_key_inner(code: String) -> Result<String, String> {
    let out = port::locale_spelling_key(code);
    Ok(out)
}

#[wasm_bindgen(js_name = "localeSpellingKey")]
pub fn locale_spelling_key(code: String) -> Result<String, JsError> {
    locale_spelling_key_inner(code).map_err(|e| JsError::new(&e))
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

static S_RESOLVE_FALLBACKS_0: wire::Schema = wire::Schema::Dyn;
static S_RESOLVE_FALLBACKS_OUT: wire::Schema = wire::Schema::Dyn;

/// `resolveFallbacks` over the binary wire.
#[wasm_bindgen(js_name = "resolveFallbacksWire")]
pub fn resolve_fallbacks_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let setting: Value = wire::read_value(r, &S_RESOLVE_FALLBACKS_0)?;
        let locales: Vec<String> = r.list(|r| r.string16())?;
        let source_locale: String = r.string16()?;
        let out = &port::resolve_fallbacks(setting, locales, source_locale)?;
        wire::write_value(w, out, &S_RESOLVE_FALLBACKS_OUT)?;
        Ok(())
    })
}

static S_FALLBACK_CHAIN_0: wire::Schema = wire::Schema::Dyn;
static S_FALLBACK_CHAIN_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Str16);

/// `fallbackChain` over the binary wire.
#[wasm_bindgen(js_name = "fallbackChainWire")]
pub fn fallback_chain_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let fallbacks: Value = wire::read_value(r, &S_FALLBACK_CHAIN_0)?;
        let locale: String = r.string16()?;
        let out = &port::fallback_chain(fallbacks, locale)?;
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_FALLBACK_CHAIN_OUT.inner())?;
        }
        Ok(())
    })
}
