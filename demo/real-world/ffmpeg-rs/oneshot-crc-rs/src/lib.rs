//! C ABI for `oneshot_crc.c`: drop-in replacements with the original names and signatures, generated
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

/// `uint32_t oneshot_crc(int id, uint32_t crc, const uint8_t *buf, size_t len)` (`oneshot_crc.c:8`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn oneshot_crc(id: std::ffi::c_int, crc: u32, buf: *const u8, len: usize) -> u32 {
    port::oneshot_crc(
        id,
        crc,
        crate::__everything_rust_c_abi::bytes(
            buf.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
    )
}

/// `oneshot_crc` as `everything_rust_oneshot_crc`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_oneshot_crc(
    id: std::ffi::c_int,
    crc: u32,
    buf: *const u8,
    len: usize,
) -> u32 {
    port::oneshot_crc(
        id,
        crc,
        crate::__everything_rust_c_abi::bytes(
            buf.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
    )
}

/// `int64_t oneshot_crc_custom(int le, int bits, uint32_t poly, int table_size, uint32_t crc, const uint8_t *buf, size_t len)` (`oneshot_crc.c:13`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn oneshot_crc_custom(
    le: std::ffi::c_int,
    bits: std::ffi::c_int,
    poly: u32,
    table_size: std::ffi::c_int,
    crc: u32,
    buf: *const u8,
    len: usize,
) -> i64 {
    port::oneshot_crc_custom(
        le,
        bits,
        poly,
        table_size,
        crc,
        crate::__everything_rust_c_abi::bytes(
            buf.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
    )
}

/// `oneshot_crc_custom` as `everything_rust_oneshot_crc_custom`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_oneshot_crc_custom(
    le: std::ffi::c_int,
    bits: std::ffi::c_int,
    poly: u32,
    table_size: std::ffi::c_int,
    crc: u32,
    buf: *const u8,
    len: usize,
) -> i64 {
    port::oneshot_crc_custom(
        le,
        bits,
        poly,
        table_size,
        crc,
        crate::__everything_rust_c_abi::bytes(
            buf.cast::<u8>(),
            crate::__everything_rust_c_abi::len(len),
        ),
    )
}
