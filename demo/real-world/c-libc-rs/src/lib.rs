//! C ABI for `src/facade.c`: drop-in replacements with the original names and signatures, generated
//! by everything-rust from `everything-rust.json`. All of the crate's `unsafe` code lives in this file.

use std::ffi::{CStr, c_char};

pub mod port;

/// A `const char *` argument as bytes; `None` for `NULL`.
fn c_str<'a>(ptr: *const c_char) -> Option<&'a [u8]> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: the C API takes NUL-terminated strings that stay valid for the call;
    // NULL is handled above.
    Some(unsafe { CStr::from_ptr(ptr) }.to_bytes())
}

/// `long parse_long(const char *text, int base)` (`src/facade.c:6`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn parse_long(text: *const c_char, base: std::ffi::c_int) -> std::ffi::c_long {
    port::parse_long(c_str(text), base)
}

/// `parse_long` as `everything_rust_parse_long`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_parse_long(
    text: *const c_char,
    base: std::ffi::c_int,
) -> std::ffi::c_long {
    port::parse_long(c_str(text), base)
}

/// `int parse_int(const char *text)` (`src/facade.c:7`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn parse_int(text: *const c_char) -> std::ffi::c_int {
    port::parse_int(c_str(text))
}

/// `parse_int` as `everything_rust_parse_int`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_parse_int(text: *const c_char) -> std::ffi::c_int {
    port::parse_int(c_str(text))
}

/// `double parse_double(const char *text)` (`src/facade.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn parse_double(text: *const c_char) -> f64 {
    port::parse_double(c_str(text))
}

/// `parse_double` as `everything_rust_parse_double`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_parse_double(text: *const c_char) -> f64 {
    port::parse_double(c_str(text))
}

/// `size_t span(const char *text, const char *accept)` (`src/facade.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn span(text: *const c_char, accept: *const c_char) -> usize {
    port::span(c_str(text), c_str(accept))
}

/// `span` as `everything_rust_span`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_span(text: *const c_char, accept: *const c_char) -> usize {
    port::span(c_str(text), c_str(accept))
}

/// `int compare(const char *a, const char *b)` (`src/facade.c:10`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn compare(a: *const c_char, b: *const c_char) -> std::ffi::c_int {
    port::compare(c_str(a), c_str(b))
}

/// `compare` as `everything_rust_compare`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_compare(a: *const c_char, b: *const c_char) -> std::ffi::c_int {
    port::compare(c_str(a), c_str(b))
}

/// `int is_alpha(int c)` (`src/facade.c:14`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn is_alpha(c: std::ffi::c_int) -> std::ffi::c_int {
    port::is_alpha(c)
}

/// `is_alpha` as `everything_rust_is_alpha`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_is_alpha(c: std::ffi::c_int) -> std::ffi::c_int {
    port::is_alpha(c)
}

/// `int is_space(int c)` (`src/facade.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn is_space(c: std::ffi::c_int) -> std::ffi::c_int {
    port::is_space(c)
}

/// `is_space` as `everything_rust_is_space`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_is_space(c: std::ffi::c_int) -> std::ffi::c_int {
    port::is_space(c)
}

/// `int is_punct(int c)` (`src/facade.c:16`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn is_punct(c: std::ffi::c_int) -> std::ffi::c_int {
    port::is_punct(c)
}

/// `is_punct` as `everything_rust_is_punct`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_is_punct(c: std::ffi::c_int) -> std::ffi::c_int {
    port::is_punct(c)
}

/// `int to_upper(int c)` (`src/facade.c:17`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn to_upper(c: std::ffi::c_int) -> std::ffi::c_int {
    port::to_upper(c)
}

/// `to_upper` as `everything_rust_to_upper`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_to_upper(c: std::ffi::c_int) -> std::ffi::c_int {
    port::to_upper(c)
}
