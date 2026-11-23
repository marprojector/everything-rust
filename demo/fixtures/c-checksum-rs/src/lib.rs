//! C ABI for `src/checksum.c`: drop-in replacements with the original names and signatures, generated
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

/// `uint32_t adler32(const char *data)` (`src/checksum.c:5`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn adler32(data: *const c_char) -> u32 {
    port::adler32(c_str(data))
}

/// `adler32` as `everything_rust_adler32`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_adler32(data: *const c_char) -> u32 {
    port::adler32(c_str(data))
}

/// `uint32_t fnv1a(const char *data)` (`src/checksum.c:15`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn fnv1a(data: *const c_char) -> u32 {
    port::fnv1a(c_str(data))
}

/// `fnv1a` as `everything_rust_fnv1a`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_fnv1a(data: *const c_char) -> u32 {
    port::fnv1a(c_str(data))
}

/// `int clamp(int value, int low, int high)` (`src/checksum.c:25`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn clamp(
    value: std::ffi::c_int,
    low: std::ffi::c_int,
    high: std::ffi::c_int,
) -> std::ffi::c_int {
    port::clamp(value, low, high)
}

/// `clamp` as `everything_rust_clamp`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_clamp(
    value: std::ffi::c_int,
    low: std::ffi::c_int,
    high: std::ffi::c_int,
) -> std::ffi::c_int {
    port::clamp(value, low, high)
}

/// `long gcd(long a, long b)` (`src/checksum.c:31`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn gcd(a: std::ffi::c_long, b: std::ffi::c_long) -> std::ffi::c_long {
    port::gcd(a, b)
}

/// `gcd` as `everything_rust_gcd`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_gcd(a: std::ffi::c_long, b: std::ffi::c_long) -> std::ffi::c_long {
    port::gcd(a, b)
}

/// `double mean3(double a, double b, double c)` (`src/checksum.c:42`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn mean3(a: f64, b: f64, c: f64) -> f64 {
    port::mean3(a, b, c)
}

/// `mean3` as `everything_rust_mean3`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_mean3(a: f64, b: f64, c: f64) -> f64 {
    port::mean3(a, b, c)
}
