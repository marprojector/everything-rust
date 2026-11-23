//! C ABI for `parseutils.c`: drop-in replacements with the original names and signatures, generated
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

    /// A writable byte pointer and its size as a slice; empty for `NULL` or a zero size.
    pub(super) fn bytes_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
        if ptr.is_null() || len == 0 {
            return &mut [];
        }

        // SAFETY: the C API passes `len` writable bytes at `ptr`, used by nothing else during the
        // call; NULL is handled above.
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }

    /// A pointer to one value as a reference; `None` for `NULL`.
    pub(super) fn out<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
        // SAFETY: the C API passes NULL or a valid, aligned pointer used by nothing else
        // during the call.
        unsafe { ptr.as_mut() }
    }

    /// A C length as `usize`; a negative length is zero.
    pub(super) fn len<T: TryInto<usize>>(n: T) -> usize {
        n.try_into().unwrap_or(0)
    }
}

/// `int av_parse_video_size(int *width_ptr, int *height_ptr, const char *str)` (`parseutils.c:150`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_parse_video_size(
    width_ptr: *mut std::ffi::c_int,
    height_ptr: *mut std::ffi::c_int,
    str: *const c_char,
) -> std::ffi::c_int {
    port::av_parse_video_size(
        crate::__everything_rust_c_abi::out(width_ptr),
        crate::__everything_rust_c_abi::out(height_ptr),
        crate::__everything_rust_c_abi::c_str(str),
    )
}

/// `av_parse_video_size` as `everything_rust_av_parse_video_size`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_parse_video_size(
    width_ptr: *mut std::ffi::c_int,
    height_ptr: *mut std::ffi::c_int,
    str: *const c_char,
) -> std::ffi::c_int {
    port::av_parse_video_size(
        crate::__everything_rust_c_abi::out(width_ptr),
        crate::__everything_rust_c_abi::out(height_ptr),
        crate::__everything_rust_c_abi::c_str(str),
    )
}

/// `int av_find_info_tag(char *arg, int arg_size, const char *tag1, const char *info)` (`parseutils.c:753`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_find_info_tag(
    arg: *mut std::ffi::c_char,
    arg_size: std::ffi::c_int,
    tag1: *const c_char,
    info: *const c_char,
) -> std::ffi::c_int {
    port::av_find_info_tag(
        crate::__everything_rust_c_abi::bytes_mut(
            arg.cast::<u8>(),
            crate::__everything_rust_c_abi::len(arg_size),
        ),
        crate::__everything_rust_c_abi::c_str(tag1),
        crate::__everything_rust_c_abi::c_str(info),
    )
}

/// `av_find_info_tag` as `everything_rust_av_find_info_tag`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_find_info_tag(
    arg: *mut std::ffi::c_char,
    arg_size: std::ffi::c_int,
    tag1: *const c_char,
    info: *const c_char,
) -> std::ffi::c_int {
    port::av_find_info_tag(
        crate::__everything_rust_c_abi::bytes_mut(
            arg.cast::<u8>(),
            crate::__everything_rust_c_abi::len(arg_size),
        ),
        crate::__everything_rust_c_abi::c_str(tag1),
        crate::__everything_rust_c_abi::c_str(info),
    )
}
