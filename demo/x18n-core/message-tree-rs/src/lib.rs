//! JS bindings for `packages/core/src/message-tree.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `isMessageLeaf`.
fn is_message_leaf_inner(value: String) -> Result<bool, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::is_message_leaf(value);
    Ok(out)
}

#[wasm_bindgen(js_name = "isMessageLeaf")]
pub fn is_message_leaf(value: String) -> Result<bool, JsError> {
    is_message_leaf_inner(value).map_err(|e| JsError::new(&e))
}

/// `isMessageTree`.
fn is_message_tree_inner(value: String) -> Result<bool, String> {
    let value = crate::js::from_json(&value)?;
    let out = port::is_message_tree(value);
    Ok(out)
}

#[wasm_bindgen(js_name = "isMessageTree")]
pub fn is_message_tree(value: String) -> Result<bool, JsError> {
    is_message_tree_inner(value).map_err(|e| JsError::new(&e))
}

/// `pathId`.
fn path_id_inner(path: Vec<String>) -> Result<String, String> {
    let out = port::path_id(path);
    Ok(out)
}

#[wasm_bindgen(js_name = "pathId")]
pub fn path_id(path: Vec<String>) -> Result<String, JsError> {
    path_id_inner(path).map_err(|e| JsError::new(&e))
}

/// `applyMessageOps`.
fn apply_message_ops_inner(tree: String, ops: String) -> Result<String, String> {
    let tree = crate::js::from_json(&tree)?;
    let ops = crate::js::from_json(&ops)?;
    let out = port::apply_message_ops(tree, ops)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "applyMessageOps")]
pub fn apply_message_ops(tree: String, ops: String) -> Result<String, JsError> {
    apply_message_ops_inner(tree, ops).map_err(|e| JsError::new(&e))
}

/// `buildTree`.
fn build_tree_inner(leaves: String) -> Result<String, String> {
    let leaves = crate::js::from_json(&leaves)?;
    let out = port::build_tree(leaves)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "buildTree")]
pub fn build_tree(leaves: String) -> Result<String, JsError> {
    build_tree_inner(leaves).map_err(|e| JsError::new(&e))
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

static S_IS_MESSAGE_LEAF_0: wire::Schema = wire::Schema::Dyn;

/// `isMessageLeaf` over the binary wire.
#[wasm_bindgen(js_name = "isMessageLeafWire")]
pub fn is_message_leaf_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_IS_MESSAGE_LEAF_0)?;
        let out = &port::is_message_leaf(value);
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_IS_MESSAGE_TREE_0: wire::Schema = wire::Schema::Dyn;

/// `isMessageTree` over the binary wire.
#[wasm_bindgen(js_name = "isMessageTreeWire")]
pub fn is_message_tree_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let value: Value = wire::read_value(r, &S_IS_MESSAGE_TREE_0)?;
        let out = &port::is_message_tree(value);
        wire::write_dyn(w, &serde_json::to_value(out).map_err(|e| e.to_string())?);
        Ok(())
    })
}

static S_APPLY_MESSAGE_OPS_0: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);
static S_APPLY_MESSAGE_OPS_1: wire::Schema = wire::Schema::List(&wire::Schema::Dyn);
static S_APPLY_MESSAGE_OPS_OUT: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);

/// `applyMessageOps` over the binary wire.
#[wasm_bindgen(js_name = "applyMessageOpsWire")]
pub fn apply_message_ops_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let tree: Value = wire::read_value(r, &S_APPLY_MESSAGE_OPS_0)?;
        let ops: Vec<Value> = r.list(|r| wire::read_value(r, S_APPLY_MESSAGE_OPS_1.inner()))?;
        let out = &port::apply_message_ops(tree, ops)?;
        wire::write_value(w, out, &S_APPLY_MESSAGE_OPS_OUT)?;
        Ok(())
    })
}

static S_BUILD_TREE_0: wire::Schema = wire::Schema::Dyn;
static S_BUILD_TREE_OUT: wire::Schema = wire::Schema::Map16(&wire::Schema::Dyn);

/// `buildTree` over the binary wire.
#[wasm_bindgen(js_name = "buildTreeWire")]
pub fn build_tree_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let leaves: Value = wire::read_value(r, &S_BUILD_TREE_0)?;
        let out = &port::build_tree(leaves)?;
        wire::write_value(w, out, &S_BUILD_TREE_OUT)?;
        Ok(())
    })
}
