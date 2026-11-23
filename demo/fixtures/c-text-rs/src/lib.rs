//! C ABI for `src/text.c`: drop-in replacements with the original names and signatures, generated
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

/// `size_t count_words(const char *s)` (`src/text.c:6`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn count_words(s: *const c_char) -> usize {
    port::count_words(c_str(s))
}

/// `count_words` as `everything_rust_count_words`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_count_words(s: *const c_char) -> usize {
    port::count_words(c_str(s))
}

/// `int vowels(const char *s)` (`src/text.c:21`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn vowels(s: *const c_char) -> std::ffi::c_int {
    port::vowels(c_str(s))
}

/// `vowels` as `everything_rust_vowels`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_vowels(s: *const c_char) -> std::ffi::c_int {
    port::vowels(c_str(s))
}

/// `bool is_palindrome(const char *s)` (`src/text.c:30`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn is_palindrome(s: *const c_char) -> bool {
    port::is_palindrome(c_str(s))
}

/// `is_palindrome` as `everything_rust_is_palindrome`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_is_palindrome(s: *const c_char) -> bool {
    port::is_palindrome(c_str(s))
}
