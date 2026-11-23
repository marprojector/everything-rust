//! JS bindings for `src/rust/relevance.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use napi_derive::napi;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;

/// Batches allocate on all cores at once; mimalloc keeps that from serializing.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic".into())
}

#[allow(dead_code)]
fn run<T>(f: impl FnOnce() -> Result<T, String>) -> napi::Result<T> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(result) => result.map_err(napi::Error::from_reason),
        Err(payload) => Err(napi::Error::from_reason(format!(
            "Rust panic: {}",
            panic_message(&*payload)
        ))),
    }
}

/// `relevance`.
#[napi(js_name = "relevance")]
pub fn relevance(text: String, pattern: String, options: String) -> napi::Result<f64> {
    run(move || {
        let options = crate::js::from_json(&options)?;
        let out = port::relevance(text, pattern, options)?;
        Ok(out)
    })
}

/// Splits `text` at UTF-16 end offsets. A lone surrogate arrived as U+FFFD, one unit as before.
fn split_joined(text: &str, ends: &[u32]) -> Vec<String> {
    let ascii = text.is_ascii()
        && ends.last().is_none_or(|&e| e as usize == text.len())
        && ends.windows(2).all(|w| w[0] <= w[1]);
    let mut out = Vec::with_capacity(ends.len());
    if ascii {
        let mut from = 0;
        for &e in ends {
            out.push(text[from..e as usize].to_string());
            from = e as usize;
        }
        return out;
    }
    let mut chars = text.chars();
    let mut unit = 0usize;
    for &e in ends {
        let mut s = String::new();
        while unit < e as usize {
            let Some(c) = chars.next() else { break };
            unit += c.len_utf16();
            s.push(c);
        }
        out.push(s);
    }
    out
}

/// `relevance` over many `text` values kept in Rust: `run` maps them all in one call, on all cores;
/// `rank` returns the indexes of results >= `min`, best first.
#[napi(js_name = "RelevanceBatch")]
pub struct RelevanceBatch {
    inner: port::RelevanceIndex,
}

#[napi]
impl RelevanceBatch {
    #[napi(constructor)]
    pub fn new(items: Vec<String>) -> Self {
        Self {
            inner: port::RelevanceIndex::new(items),
        }
    }

    /// `new` for items sent as one joined string and their UTF-16 end offsets.
    #[napi(factory, js_name = "fromJoined")]
    pub fn from_joined(text: String, ends: napi::bindgen_prelude::Uint32Array) -> Self {
        Self {
            inner: port::RelevanceIndex::new(split_joined(&text, &ends)),
        }
    }

    /// The result for every item, in order.
    #[napi]
    pub fn run(
        &self,
        pattern: String,
        options: Value,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let out = run(|| self.inner.run(pattern, options))?;
        Ok(napi::bindgen_prelude::Float64Array::new(out))
    }

    /// Indexes of the items whose result is at least `min`, highest result first.
    #[napi]
    pub fn rank(
        &self,
        pattern: String,
        options: Value,
        min: f64,
    ) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        let out = run(|| self.inner.run(pattern, options))?;
        let mut hits: Vec<(f64, u32)> = out
            .into_iter()
            .enumerate()
            .map(|(i, r)| (r, u32::try_from(i).unwrap_or(u32::MAX)))
            .filter(|(r, _)| *r >= min)
            .collect();
        // Stable: equal results keep the items' order, as JS `sort` and Python `sorted` do.
        hits.sort_by(|a, b| b.0.total_cmp(&a.0));
        let order: Vec<u32> = hits.into_iter().map(|(_, i)| i).collect();
        Ok(napi::bindgen_prelude::Uint32Array::new(order))
    }
}

// ---- the binary wire ------------------------------------------------------------------------

type WireBuffers = (
    napi::bindgen_prelude::Uint8Array,
    napi::bindgen_prelude::Uint8Array,
    napi::bindgen_prelude::Uint32Array,
);

thread_local! {
    static WIRE: std::cell::RefCell<Option<WireBuffers>> = const { std::cell::RefCell::new(None) };
    static PENDING: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    static SCRATCH: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Registers the argument, result and argument-length buffers `index.cjs` uses.
#[napi(js_name = "everythingRustWireInit")]
pub fn everything_rust_wire_init(
    input: napi::bindgen_prelude::Uint8Array,
    output: napi::bindgen_prelude::Uint8Array,
    len: napi::bindgen_prelude::Uint32Array,
) {
    WIRE.with(|w| *w.borrow_mut() = Some((input, output, len)));
}

/// Copies a result that did not fit into the (now larger) result buffer.
#[napi(js_name = "everythingRustWireFlush")]
pub fn everything_rust_wire_flush() -> f64 {
    let bytes = PENDING.with(|p| std::mem::take(&mut *p.borrow_mut()));
    WIRE.with(|w| {
        let mut w = w.borrow_mut();
        let Some((_, output, _)) = w.as_mut() else {
            return -1.0;
        };
        // SAFETY: the call is synchronous; JS does not touch `output` until it returns.
        let out = unsafe { output.as_mut() };
        let n = bytes.len().min(out.len());
        out[..n].copy_from_slice(&bytes[..n]);
        n as f64
    })
}

/// Reads the argument bytes (`wLen[0]` of them), runs `f`, and writes its result; a negative
/// return is the size of a result that did not fit, kept for `everythingRustWireFlush`.
fn wire_io(
    f: impl FnOnce(&mut wire::Reader, &mut wire::Writer) -> Result<(), String>,
) -> napi::Result<f64> {
    WIRE.with(|cell| {
        let mut guard = cell.borrow_mut();
        let Some((input, output, len)) = guard.as_mut() else {
            return Err(napi::Error::from_reason(
                "everything-rust: the wire buffers are not registered",
            ));
        };
        let len = (len.first().copied().unwrap_or(0) as usize).min(input.len());
        let mut w = wire::Writer(SCRATCH.with(|s| std::mem::take(&mut *s.borrow_mut())));
        w.0.clear();
        let result = run(|| f(&mut wire::Reader::new(&input[..len]), &mut w));
        let keep = |w: wire::Writer| SCRATCH.with(|s| *s.borrow_mut() = w.0);
        match result {
            // A result of another shape than the TS type: JS runs the call through JSON.
            Err(e) if e.reason.starts_with(wire::MISMATCH) => {
                keep(w);
                return Ok(f64::NAN);
            }
            Err(e) => {
                keep(w);
                let mut m = wire::Writer::default();
                m.string16(&e.reason);
                // SAFETY: the call is synchronous; JS does not touch `output` until it returns.
                let out = unsafe { output.as_mut() };
                if m.0.len() > out.len() {
                    return Err(e);
                }
                out[..m.0.len()].copy_from_slice(&m.0);
                return Ok(f64::INFINITY);
            }
            Ok(()) => {}
        }
        // SAFETY: the call is synchronous; JS does not touch `output` until it returns.
        let out = unsafe { output.as_mut() };
        let n = w.0.len();
        if n > out.len() {
            PENDING.with(|p| *p.borrow_mut() = w.0);
            return Ok(-(n as f64));
        }
        out[..n].copy_from_slice(&w.0);
        keep(w);
        Ok(n as f64)
    })
}

static S_RELEVANCE_2: wire::Schema = wire::Schema::Dyn;

/// `relevance` over the binary wire.
#[napi(js_name = "relevanceWire")]
pub fn relevance_wire() -> napi::Result<f64> {
    wire_io(|r, w| {
        let text: String = r.string16()?;
        let pattern: String = r.string16()?;
        let options: Value = wire::read_value(r, &S_RELEVANCE_2)?;
        let out = &port::relevance(text, pattern, options)?;
        w.f64(*out);
        Ok(())
    })
}
