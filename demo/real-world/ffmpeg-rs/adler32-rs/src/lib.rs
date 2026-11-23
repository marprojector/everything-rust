//! C ABI for `adler32.c`: drop-in replacements with the original names and signatures, generated
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

    /// A C length as `usize`; a negative length is zero.
    pub(super) fn len<T: TryInto<usize>>(n: T) -> usize {
        n.try_into().unwrap_or(0)
    }
}

/// `AVAdler av_adler32_update(AVAdler adler, const uint8_t *buf, size_t len)` (`adler32.c:44`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_adler32_update(adler: u32, buf: *const u8, len: usize) -> u32 {
    port::av_adler32_update(
        adler,
        crate::__everything_rust_c_abi::bytes(
            buf.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
    )
}

/// `av_adler32_update` as `everything_rust_av_adler32_update`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_adler32_update(adler: u32, buf: *const u8, len: usize) -> u32 {
    port::av_adler32_update(
        adler,
        crate::__everything_rust_c_abi::bytes(
            buf.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
    )
}
