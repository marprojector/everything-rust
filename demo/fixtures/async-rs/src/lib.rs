//! JS bindings for `src/pipeline.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use napi_derive::napi;

pub mod js;
pub mod port;

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

#[allow(dead_code)]
async fn run_async<T>(f: impl std::future::Future<Output = Result<T, String>>) -> napi::Result<T> {
    use futures::FutureExt as _;
    match std::panic::AssertUnwindSafe(f).catch_unwind().await {
        Ok(result) => result.map_err(napi::Error::from_reason),
        Err(payload) => Err(napi::Error::from_reason(format!(
            "Rust panic: {}",
            panic_message(&*payload)
        ))),
    }
}

/// `normalize`.
#[napi(js_name = "normalize")]
pub async fn normalize(entries: String, prefix: Option<String>) -> napi::Result<String> {
    run_async(async move {
        let entries = crate::js::from_json(&entries)?;
        let out = port::normalize(entries, prefix).await?;
        crate::js::to_json(&out)
    })
    .await
}

/// `countWords`.
#[napi(js_name = "countWords")]
pub async fn count_words(text: String) -> napi::Result<f64> {
    run_async(async move {
        let out = port::count_words(text).await?;
        Ok(out)
    })
    .await
}

/// `upper`.
#[napi(js_name = "upper")]
pub fn upper(text: String) -> napi::Result<String> {
    run(move || {
        let out = port::upper(text)?;
        Ok(out)
    })
}
