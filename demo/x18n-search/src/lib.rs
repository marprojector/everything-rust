//! JS bindings for `packages/core/src/fuzzy.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use napi_derive::napi;

pub mod port;

/// Batches allocate on all cores at once; mimalloc keeps that from serializing.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod utf16;

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

/// `fuzzyScore`.
#[napi(js_name = "fuzzyScore")]
pub fn fuzzy_score(query: String, text: String) -> napi::Result<f64> {
    run(move || {
        let out = port::fuzzy_score(query, text);
        Ok(out)
    })
}

/// `fuzzyScore` over many `text` values kept in Rust: `run` maps them all in one call, on all cores;
/// `rank` returns the indexes of results >= `min`, best first.
#[napi(js_name = "FuzzyScoreBatch")]
pub struct FuzzyScoreBatch {
    inner: port::FuzzyScoreIndex,
}

#[napi]
impl FuzzyScoreBatch {
    #[napi(constructor)]
    pub fn new(items: Vec<String>) -> Self {
        Self {
            inner: port::FuzzyScoreIndex::new(items),
        }
    }

    /// The result for every item, in order.
    #[napi]
    pub fn run(&self, query: String) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let out = run(|| self.inner.run(query))?;
        Ok(napi::bindgen_prelude::Float64Array::new(out))
    }

    /// Indexes of the items whose result is at least `min`, highest result first.
    #[napi]
    pub fn rank(
        &self,
        query: String,
        min: f64,
    ) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        let out = run(|| self.inner.rank(query, min))?;
        let order: Vec<u32> = out;
        Ok(napi::bindgen_prelude::Uint32Array::new(order))
    }
}
