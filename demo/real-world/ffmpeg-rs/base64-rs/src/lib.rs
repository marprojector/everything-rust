//! C ABI for `base64.c`: drop-in replacements with the original names and signatures, generated
//! by everything-rust from `everything-rust.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

use std::ffi::c_char;

mod __everything_rust_c_abi {
    use std::ffi::{CStr, c_char};

    /// A `const char *` argument as bytes; `None` for `NULL`.
    pub(super) fn c_str<'a>(ptr: *const c_char) -> Option<&'a [u8]> {
        if ptr.is_null() {
            return None;
        }

        // SAFETY: the C API takes NUL-terminated strings that stay valid for the call;
        // NULL is handled above.
        Some(unsafe { CStr::from_ptr(ptr) }.to_bytes())
    }

    /// A `const` byte pointer and its length as a slice; empty for `NULL` or a zero length.
    pub(super) fn bytes<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
        if ptr.is_null() || len == 0 {
            return &[];
        }

        // SAFETY: the C API passes `len` readable bytes at `ptr` that stay valid for the call;
        // NULL is handled above.
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    /// A writable byte pointer and its size as a slice; empty for `NULL` or a zero size.
    pub(super) fn bytes_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
        if ptr.is_null() || len == 0 {
            return &mut [];
        }

        // SAFETY: the C API passes `len` writable bytes at `ptr`, used by nothing else during the
        // call; NULL is handled above.
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }

    /// A C length as `usize`; a negative length is zero.
    pub(super) fn len<T: TryInto<usize>>(n: T) -> usize {
        n.try_into().unwrap_or(0)
    }
}

/// `int av_base64_decode(uint8_t *out, const char *in_str, int out_size)` (`base64.c:81`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_base64_decode(
    out: *mut u8,
    in_str: *const c_char,
    out_size: std::ffi::c_int,
) -> std::ffi::c_int {
    port::av_base64_decode(
        crate::__everything_rust_c_abi::bytes_mut(
            out.cast::<u8>(),
            crate::__everything_rust_c_abi::len(out_size),
        ),
        crate::__everything_rust_c_abi::c_str(in_str),
    )
}

/// `av_base64_decode` as `everything_rust_av_base64_decode`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_base64_decode(
    out: *mut u8,
    in_str: *const c_char,
    out_size: std::ffi::c_int,
) -> std::ffi::c_int {
    port::av_base64_decode(
        crate::__everything_rust_c_abi::bytes_mut(
            out.cast::<u8>(),
            crate::__everything_rust_c_abi::len(out_size),
        ),
        crate::__everything_rust_c_abi::c_str(in_str),
    )
}

/// `char *av_base64_encode(char *out, int out_size, const uint8_t *in, int in_size)` (`base64.c:147`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_base64_encode(
    out: *mut std::ffi::c_char,
    out_size: std::ffi::c_int,
    r#in: *const u8,
    in_size: std::ffi::c_int,
) -> *mut std::ffi::c_char {
    match port::av_base64_encode(
        crate::__everything_rust_c_abi::bytes_mut(
            out.cast::<u8>(),
            crate::__everything_rust_c_abi::len(out_size),
        ),
        crate::__everything_rust_c_abi::bytes(
            r#in.cast::<u8>(),
            crate::__everything_rust_c_abi::len(in_size),
        ),
    ) {
        Some(offset) => out.cast::<u8>().wrapping_add(offset).cast(),
        None => std::ptr::null_mut::<std::ffi::c_char>(),
    }
}

/// `av_base64_encode` as `everything_rust_av_base64_encode`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_base64_encode(
    out: *mut std::ffi::c_char,
    out_size: std::ffi::c_int,
    r#in: *const u8,
    in_size: std::ffi::c_int,
) -> *mut std::ffi::c_char {
    match port::av_base64_encode(
        crate::__everything_rust_c_abi::bytes_mut(
            out.cast::<u8>(),
            crate::__everything_rust_c_abi::len(out_size),
        ),
        crate::__everything_rust_c_abi::bytes(
            r#in.cast::<u8>(),
            crate::__everything_rust_c_abi::len(in_size),
        ),
    ) {
        Some(offset) => out.cast::<u8>().wrapping_add(offset).cast(),
        None => std::ptr::null_mut::<std::ffi::c_char>(),
    }
}
