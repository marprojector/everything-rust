//! C ABI for `rational.c`: drop-in replacements with the original names and signatures, generated
//! by everything-rust from `everything-rust.json`. All of the crate's `unsafe` code lives in this file.

pub mod port;

/// `AVRational` as C lays it out: passed and returned by value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AVRational {
    pub num: std::ffi::c_int,
    pub den: std::ffi::c_int,
}

mod __everything_rust_c_abi {

    /// A pointer to one value as a reference; `None` for `NULL`.
    pub(super) fn out<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
        // SAFETY: the C API passes NULL or a valid, aligned pointer used by nothing else
        // during the call.
        unsafe { ptr.as_mut() }
    }
}

/// `int av_reduce(int *dst_num, int *dst_den, int64_t num, int64_t den, int64_t max)` (`rational.c:35`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_reduce(
    dst_num: *mut std::ffi::c_int,
    dst_den: *mut std::ffi::c_int,
    num: i64,
    den: i64,
    max: i64,
) -> std::ffi::c_int {
    port::av_reduce(
        crate::__everything_rust_c_abi::out(dst_num),
        crate::__everything_rust_c_abi::out(dst_den),
        num,
        den,
        max,
    )
}

/// `av_reduce` as `everything_rust_av_reduce`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_reduce(
    dst_num: *mut std::ffi::c_int,
    dst_den: *mut std::ffi::c_int,
    num: i64,
    den: i64,
    max: i64,
) -> std::ffi::c_int {
    port::av_reduce(
        crate::__everything_rust_c_abi::out(dst_num),
        crate::__everything_rust_c_abi::out(dst_den),
        num,
        den,
        max,
    )
}

/// `AVRational av_mul_q(AVRational b, AVRational c)` (`rational.c:80`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_mul_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_mul_q(b, c)
}

/// `av_mul_q` as `everything_rust_av_mul_q`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_mul_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_mul_q(b, c)
}

/// `AVRational av_div_q(AVRational b, AVRational c)` (`rational.c:88`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_div_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_div_q(b, c)
}

/// `av_div_q` as `everything_rust_av_div_q`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_div_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_div_q(b, c)
}

/// `AVRational av_add_q(AVRational b, AVRational c)` (`rational.c:93`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_add_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_add_q(b, c)
}

/// `av_add_q` as `everything_rust_av_add_q`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_add_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_add_q(b, c)
}

/// `AVRational av_sub_q(AVRational b, AVRational c)` (`rational.c:101`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_sub_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_sub_q(b, c)
}

/// `av_sub_q` as `everything_rust_av_sub_q`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_sub_q(b: AVRational, c: AVRational) -> crate::AVRational {
    port::av_sub_q(b, c)
}

/// `AVRational av_d2q(double d, int max)` (`rational.c:106`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_d2q(d: f64, max: std::ffi::c_int) -> crate::AVRational {
    port::av_d2q(d, max)
}

/// `av_d2q` as `everything_rust_av_d2q`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_d2q(d: f64, max: std::ffi::c_int) -> crate::AVRational {
    port::av_d2q(d, max)
}

/// `int av_nearer_q(AVRational q, AVRational q1, AVRational q2)` (`rational.c:127`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_nearer_q(q: AVRational, q1: AVRational, q2: AVRational) -> std::ffi::c_int {
    port::av_nearer_q(q, q1, q2)
}

/// `av_nearer_q` as `everything_rust_av_nearer_q`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_nearer_q(
    q: AVRational,
    q1: AVRational,
    q2: AVRational,
) -> std::ffi::c_int {
    port::av_nearer_q(q, q1, q2)
}

/// `uint32_t av_q2intfloat(AVRational q)` (`rational.c:152`).
#[cfg(not(feature = "shadow"))]
// SAFETY: exported under the C name it replaces: link it instead of the original object.
#[unsafe(no_mangle)]
pub extern "C" fn av_q2intfloat(q: AVRational) -> u32 {
    port::av_q2intfloat(q)
}

/// `av_q2intfloat` as `everything_rust_av_q2intfloat`, for shadow builds beside the original.
// SAFETY: a name no C code defines.
#[unsafe(no_mangle)]
pub extern "C" fn everything_rust_av_q2intfloat(q: AVRational) -> u32 {
    port::av_q2intfloat(q)
}
