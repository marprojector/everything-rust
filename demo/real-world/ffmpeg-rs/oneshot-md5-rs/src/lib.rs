//! C ABI for `oneshot_md5.c`: drop-in replacements with the original names and signatures, generated
//! by everything-rust from `everything-rust.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

mod __everything_rust_c_abi {

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

/// `int oneshot_md5(uint8_t *dst, int dst_size, const uint8_t *src, size_t len, int chunk)` (`oneshot_md5.c:9`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn oneshot_md5(
    dst: *mut u8,
    dst_size: std::ffi::c_int,
    src: *const u8,
    len: usize,
    chunk: std::ffi::c_int,
) -> std::ffi::c_int {
    port::oneshot_md5(
        crate::__everything_rust_c_abi::bytes_mut(
            dst.cast::<u8>(),
            crate::__everything_rust_c_abi::len(dst_size),
        ),
        crate::__everything_rust_c_abi::bytes(
            src.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
        chunk,
    )
}

/// `oneshot_md5` as `everything_rust_oneshot_md5`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_oneshot_md5(
    dst: *mut u8,
    dst_size: std::ffi::c_int,
    src: *const u8,
    len: usize,
    chunk: std::ffi::c_int,
) -> std::ffi::c_int {
    port::oneshot_md5(
        crate::__everything_rust_c_abi::bytes_mut(
            dst.cast::<u8>(),
            crate::__everything_rust_c_abi::len(dst_size),
        ),
        crate::__everything_rust_c_abi::bytes(
            src.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
        chunk,
    )
}
