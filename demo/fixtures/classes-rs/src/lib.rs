//! JS bindings for `src/stats.ts`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;

fn text_stats_new_inner(separator: Option<String>) -> Result<port::TextStats, String> {
    let out = port::TextStats::new(separator)?;
    Ok(out)
}

fn text_stats_add_inner(inner: &mut port::TextStats, text: String) -> Result<f64, String> {
    let out = inner.add(text)?;
    Ok(out)
}

fn text_stats_stats_inner(inner: &mut port::TextStats) -> Result<String, String> {
    let out = inner.stats()?;
    crate::js::to_json(&out)
}

fn text_stats_longest_word_inner(inner: &port::TextStats) -> Result<Vec<u16>, String> {
    let out = inner.longest_word()?;
    Ok(out)
}

fn text_stats_reset_inner(inner: &mut port::TextStats) -> Result<(), String> {
    inner.reset()?;
    Ok(())
}

/// Class `TextStats`.
#[wasm_bindgen(js_name = "TextStats")]
pub struct TextStats {
    inner: port::TextStats,
}

#[wasm_bindgen(js_class = "TextStats")]
impl TextStats {
    #[wasm_bindgen(constructor)]
    pub fn new(separator: Option<String>) -> Result<TextStats, JsError> {
        let inner = text_stats_new_inner(separator).map_err(|e| JsError::new(&e))?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = "add")]
    pub fn add(&mut self, text: String) -> Result<f64, JsError> {
        text_stats_add_inner(&mut self.inner, text).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "stats")]
    pub fn stats(&mut self) -> Result<String, JsError> {
        text_stats_stats_inner(&mut self.inner).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "longestWord")]
    pub fn longest_word(&self) -> Result<Vec<u16>, JsError> {
        text_stats_longest_word_inner(&self.inner).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "reset")]
    pub fn reset(&mut self) -> Result<(), JsError> {
        text_stats_reset_inner(&mut self.inner).map_err(|e| JsError::new(&e))
    }
}

fn lru_cache_new_inner(capacity: f64) -> Result<port::LruCache, String> {
    let out = port::LruCache::new(capacity)?;
    Ok(out)
}

fn lru_cache_get_inner(inner: &mut port::LruCache, key: String) -> Result<Option<String>, String> {
    let out = inner.get(key)?;
    Ok(out)
}

fn lru_cache_set_inner(
    inner: &mut port::LruCache,
    key: String,
    value: String,
) -> Result<Vec<String>, String> {
    let out = inner.set(key, value)?;
    Ok(out)
}

fn lru_cache_keys_inner(inner: &port::LruCache) -> Result<Vec<String>, String> {
    let out = inner.keys()?;
    Ok(out)
}

/// Class `LruCache`.
#[wasm_bindgen(js_name = "LruCache")]
pub struct LruCache {
    inner: port::LruCache,
}

#[wasm_bindgen(js_class = "LruCache")]
impl LruCache {
    #[wasm_bindgen(constructor)]
    pub fn new(capacity: f64) -> Result<LruCache, JsError> {
        let inner = lru_cache_new_inner(capacity).map_err(|e| JsError::new(&e))?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = "get")]
    pub fn get(&mut self, key: String) -> Result<Option<String>, JsError> {
        lru_cache_get_inner(&mut self.inner, key).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "set")]
    pub fn set(&mut self, key: String, value: String) -> Result<Vec<String>, JsError> {
        lru_cache_set_inner(&mut self.inner, key, value).map_err(|e| JsError::new(&e))
    }

    #[wasm_bindgen(js_name = "keys")]
    pub fn keys(&self) -> Result<Vec<String>, JsError> {
        lru_cache_keys_inner(&self.inner).map_err(|e| JsError::new(&e))
    }
}
