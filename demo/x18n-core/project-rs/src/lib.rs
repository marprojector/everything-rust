//! JS bindings for `packages/core/src/project/`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `isOutdated`.
fn is_outdated_inner(source: String, target: String) -> Result<bool, String> {
    let source = crate::js::from_json(&source)?;
    let target = crate::js::from_json(&target)?;
    let out = port::is_outdated(source, target)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "isOutdated")]
pub fn is_outdated(source: String, target: String) -> Result<bool, JsError> {
    is_outdated_inner(source, target).map_err(|e| JsError::new(&e))
}

/// `valueText`.
fn value_text_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::value_text(value);
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "valueText")]
pub fn value_text(value: String) -> Result<String, JsError> {
    value_text_inner(value).map_err(|e| JsError::new(&e))
}

/// `sourceHash`.
fn source_hash_inner(value: String) -> Result<String, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::source_hash(value)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "sourceHash")]
pub fn source_hash(value: String) -> Result<String, JsError> {
    source_hash_inner(value).map_err(|e| JsError::new(&e))
}

/// `findReview`.
fn find_review_inner(data: String, id: String) -> Result<String, String> {
    let data = crate::js::from_json(&data)?;
    let out = port::find_review(data, id)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "findReview")]
pub fn find_review(data: String, id: String) -> Result<String, JsError> {
    find_review_inner(data, id).map_err(|e| JsError::new(&e))
}

/// `sortReviews`.
fn sort_reviews_inner(data: String) -> Result<String, String> {
    let data = crate::js::from_json(&data)?;
    let out = port::sort_reviews(data)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "sortReviews")]
pub fn sort_reviews(data: String) -> Result<String, JsError> {
    sort_reviews_inner(data).map_err(|e| JsError::new(&e))
}

/// `prefixMatches`.
fn prefix_matches_inner(usage: String, key: String) -> Result<bool, String> {
    let usage = crate::js::from_json(&usage)?;
    let out = port::prefix_matches(usage, key)?;
    Ok(out)
}

#[wasm_bindgen(js_name = "prefixMatches")]
pub fn prefix_matches(usage: String, key: String) -> Result<bool, JsError> {
    prefix_matches_inner(usage, key).map_err(|e| JsError::new(&e))
}

/// `suppressedRanges`.
fn suppressed_ranges_inner(text: String) -> Result<String, String> {
    let out = port::suppressed_ranges(text);
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "suppressedRanges")]
pub fn suppressed_ranges(text: String) -> Result<String, JsError> {
    suppressed_ranges_inner(text).map_err(|e| JsError::new(&e))
}

/// `categoriesOf`.
fn categories_of_inner(info: String, kind: String) -> Result<String, String> {
    let info = crate::js::from_json(&info)?;
    let kind = crate::js::from_json(&kind)?;
    let out = port::categories_of(info, kind)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "categoriesOf")]
pub fn categories_of(info: String, kind: String) -> Result<String, JsError> {
    categories_of_inner(info, kind).map_err(|e| JsError::new(&e))
}

/// `renderPrompt`.
fn render_prompt_inner(data: String) -> Result<String, String> {
    let data = crate::js::from_json(&data)?;
    let out = port::render_prompt(data)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "renderPrompt")]
pub fn render_prompt(data: String) -> Result<String, JsError> {
    render_prompt_inner(data).map_err(|e| JsError::new(&e))
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

static S_IS_OUTDATED_0: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("updatedAt", wire::Schema::Num),
    ("revision", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("author", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("value", wire::Schema::Dyn),
]));
static S_IS_OUTDATED_1: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("updatedAt", wire::Schema::Num),
    ("revision", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("author", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("value", wire::Schema::Dyn),
]));

/// `isOutdated` over the binary wire.
#[wasm_bindgen(js_name = "isOutdatedWire")]
pub fn is_outdated_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let source: Value = wire::read_value(r, &S_IS_OUTDATED_0)?;
        let target: Value = wire::read_value(r, &S_IS_OUTDATED_1)?;
        let out = &port::is_outdated(source, target)?;
        w.bool(*out);
        Ok(())
    })
}

static S_VALUE_TEXT_0: wire::Schema = wire::Schema::Dyn;
static S_VALUE_TEXT_OUT: wire::Schema = wire::Schema::Str16;

/// `valueText` over the binary wire.
#[wasm_bindgen(js_name = "valueTextWire")]
pub fn value_text_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_VALUE_TEXT_0)?;
        let out = &port::value_text(value);
        wire::write_value(w, out, &S_VALUE_TEXT_OUT)?;
        Ok(())
    })
}

static S_SOURCE_HASH_0: wire::Schema = wire::Schema::Dyn;

/// `sourceHash` over the binary wire.
#[wasm_bindgen(js_name = "sourceHashWire")]
pub fn source_hash_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_SOURCE_HASH_0)?;
        let out = &port::source_hash(value)?;
        w.string16(out);
        Ok(())
    })
}

static S_FIND_REVIEW_0: wire::Schema = wire::Schema::Obj(&[
    (
        "threads",
        wire::Schema::Map16(&wire::Schema::Map16(&wire::Schema::List(
            &wire::Schema::Obj(&[
                ("id", wire::Schema::Dyn),
                ("type", wire::Schema::Dyn),
                ("author", wire::Schema::Dyn),
                ("time", wire::Schema::Dyn),
                ("text", wire::Schema::Dyn),
                ("suggestion", wire::Schema::Dyn),
                ("value", wire::Schema::Dyn),
                ("resolved", wire::Schema::Dyn),
            ]),
        ))),
    ),
    (
        "sources",
        wire::Schema::Map16(&wire::Schema::Map16(&wire::Schema::Str16)),
    ),
]);
static S_FIND_REVIEW_OUT: wire::Schema = wire::Schema::OptJs(&wire::Schema::Obj(&[
    ("key", wire::Schema::Str16),
    ("locale", wire::Schema::Str16),
    (
        "comment",
        wire::Schema::Obj(&[
            ("id", wire::Schema::Str16),
            ("type", wire::Schema::Str16),
            (
                "author",
                wire::Schema::OptJs(&wire::Schema::Obj(&[
                    ("name", wire::Schema::Dyn),
                    ("email", wire::Schema::Dyn),
                ])),
            ),
            ("time", wire::Schema::Str16),
            ("text", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("suggestion", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("value", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("resolved", wire::Schema::OptJs(&wire::Schema::Bool)),
        ]),
    ),
]));

/// `findReview` over the binary wire.
#[wasm_bindgen(js_name = "findReviewWire")]
pub fn find_review_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let data: Value = wire::read_value(r, &S_FIND_REVIEW_0)?;
        let id: String = r.string16()?;
        let out = &port::find_review(data, id)?;
        wire::write_value(w, out, &S_FIND_REVIEW_OUT)?;
        Ok(())
    })
}

static S_SORT_REVIEWS_0: wire::Schema = wire::Schema::Obj(&[
    (
        "threads",
        wire::Schema::Map16(&wire::Schema::Map16(&wire::Schema::List(
            &wire::Schema::Obj(&[
                ("id", wire::Schema::Dyn),
                ("type", wire::Schema::Dyn),
                ("author", wire::Schema::Dyn),
                ("time", wire::Schema::Dyn),
                ("text", wire::Schema::Dyn),
                ("suggestion", wire::Schema::Dyn),
                ("value", wire::Schema::Dyn),
                ("resolved", wire::Schema::Dyn),
            ]),
        ))),
    ),
    (
        "sources",
        wire::Schema::Map16(&wire::Schema::Map16(&wire::Schema::Str16)),
    ),
]);
static S_SORT_REVIEWS_OUT: wire::Schema = wire::Schema::Obj(&[
    (
        "threads",
        wire::Schema::Map16(&wire::Schema::Map16(&wire::Schema::List(
            &wire::Schema::Obj(&[
                ("id", wire::Schema::Dyn),
                ("type", wire::Schema::Dyn),
                ("author", wire::Schema::Dyn),
                ("time", wire::Schema::Dyn),
                ("text", wire::Schema::Dyn),
                ("suggestion", wire::Schema::Dyn),
                ("value", wire::Schema::Dyn),
                ("resolved", wire::Schema::Dyn),
            ]),
        ))),
    ),
    (
        "sources",
        wire::Schema::Map16(&wire::Schema::Map16(&wire::Schema::Str16)),
    ),
]);

/// `sortReviews` over the binary wire.
#[wasm_bindgen(js_name = "sortReviewsWire")]
pub fn sort_reviews_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let data: Value = wire::read_value(r, &S_SORT_REVIEWS_0)?;
        let out = &port::sort_reviews(data)?;
        wire::write_value(w, out, &S_SORT_REVIEWS_OUT)?;
        Ok(())
    })
}

static S_PREFIX_MATCHES_0: wire::Schema = wire::Schema::Obj(&[
    ("key", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("raw", wire::Schema::Str16),
    ("path", wire::Schema::Str16),
    (
        "range",
        wire::Schema::Obj(&[("start", wire::Schema::Num), ("end", wire::Schema::Num)]),
    ),
    (
        "expression",
        wire::Schema::OptJs(&wire::Schema::Obj(&[
            ("start", wire::Schema::Num),
            ("end", wire::Schema::Num),
        ])),
    ),
    ("scanner", wire::Schema::Str16),
    ("kind", wire::Schema::Str16),
    ("dynamic", wire::Schema::Dyn),
    ("locale", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("scopeNamespace", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("scopePrefix", wire::Schema::OptJs(&wire::Schema::Str16)),
]);

/// `prefixMatches` over the binary wire.
#[wasm_bindgen(js_name = "prefixMatchesWire")]
pub fn prefix_matches_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let usage: Value = wire::read_value(r, &S_PREFIX_MATCHES_0)?;
        let key: String = r.string16()?;
        let out = &port::prefix_matches(usage, key)?;
        w.bool(*out);
        Ok(())
    })
}

static S_SUPPRESSED_RANGES_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Obj(&[
    ("start", wire::Schema::Num),
    ("end", wire::Schema::Num),
]));

/// `suppressedRanges` over the binary wire.
#[wasm_bindgen(js_name = "suppressedRangesWire")]
pub fn suppressed_ranges_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let text: String = r.string16()?;
        let out = &port::suppressed_ranges(text);
        w.tag(1);
        w.count(out.len());
        for x0 in out.iter() {
            wire::write_value(w, x0, S_SUPPRESSED_RANGES_OUT.inner())?;
        }
        Ok(())
    })
}

static S_CATEGORIES_OF_0: wire::Schema = wire::Schema::Obj(&[
    ("code", wire::Schema::Str16),
    ("language", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("region", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("displayName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("flag", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("pluralCategories", wire::Schema::List(&wire::Schema::Dyn)),
    ("ordinalCategories", wire::Schema::List(&wire::Schema::Dyn)),
]);
static S_CATEGORIES_OF_1: wire::Schema = wire::Schema::Dyn;
static S_CATEGORIES_OF_OUT: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);

/// `categoriesOf` over the binary wire.
#[wasm_bindgen(js_name = "categoriesOfWire")]
pub fn categories_of_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let info: Value = wire::read_value(r, &S_CATEGORIES_OF_0)?;
        let kind: Value = wire::read_value(r, &S_CATEGORIES_OF_1)?;
        let out = &port::categories_of(info, kind)?;
        wire::write_value(w, out, &S_CATEGORIES_OF_OUT)?;
        Ok(())
    })
}

static S_RENDER_PROMPT_0: wire::Schema = wire::Schema::Obj(&[
    ("kind", wire::Schema::Str16),
    ("project", wire::Schema::Str16),
    ("from", wire::Schema::Str16),
    ("to", wire::Schema::Str16),
    ("targetName", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("total", wire::Schema::Num),
    ("remaining", wire::Schema::Num),
    ("nextCursor", wire::Schema::OptJs(&wire::Schema::Str16)),
    ("rules", wire::Schema::List(&wire::Schema::Str16)),
    ("notes", wire::Schema::List(&wire::Schema::Str16)),
    (
        "glossary",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("term", wire::Schema::Str16),
            ("translation", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("note", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("doNotTranslate", wire::Schema::Bool),
        ])),
    ),
    (
        "examples",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("source", wire::Schema::Str16),
            ("target", wire::Schema::Str16),
        ])),
    ),
    (
        "items",
        wire::Schema::List(&wire::Schema::Obj(&[
            ("key", wire::Schema::Str16),
            ("source", wire::Schema::Str16),
            ("placeholders", wire::Schema::List(&wire::Schema::Str16)),
            ("existing", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("previousSource", wire::Schema::OptJs(&wire::Schema::Str16)),
            ("context", wire::Schema::List(&wire::Schema::Dyn)),
        ])),
    ),
    ("instructions", wire::Schema::List(&wire::Schema::Str16)),
]);
static S_RENDER_PROMPT_OUT: wire::Schema = wire::Schema::Str16;

/// `renderPrompt` over the binary wire.
#[wasm_bindgen(js_name = "renderPromptWire")]
pub fn render_prompt_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let data: Value = wire::read_value(r, &S_RENDER_PROMPT_0)?;
        let out = &port::render_prompt(data)?;
        wire::write_value(w, out, &S_RENDER_PROMPT_OUT)?;
        Ok(())
    })
}
