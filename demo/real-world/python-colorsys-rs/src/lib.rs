//! Python bindings for `src/colorsys_pkg/`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use pyo3::prelude::*;

pub mod js;
pub mod port;
pub mod pyconv;
pub mod pynum;

#[allow(unused_imports)]
use pynum::Num;

fn error(message: String) -> PyErr {
    use pyo3::exceptions as e;
    let (kind, text) = message.split_once(": ").unwrap_or(("", message.as_str()));
    let text = text.to_string();
    match kind {
        "TypeError" => e::PyTypeError::new_err(text),
        "KeyError" => e::PyKeyError::new_err(text),
        "IndexError" => e::PyIndexError::new_err(text),
        "OverflowError" => e::PyOverflowError::new_err(text),
        "ZeroDivisionError" => e::PyZeroDivisionError::new_err(text),
        "AttributeError" => e::PyAttributeError::new_err(text),
        "RuntimeError" => e::PyRuntimeError::new_err(text),
        "NotImplementedError" => e::PyNotImplementedError::new_err(text),
        "LookupError" => e::PyLookupError::new_err(text),
        "ArithmeticError" => e::PyArithmeticError::new_err(text),
        "UnicodeError" => e::PyUnicodeError::new_err(text),
        "ValueError" => e::PyValueError::new_err(text),
        _ => e::PyValueError::new_err(message),
    }
}

fn rgb_to_yiq_inner(
    r: Bound<'_, PyAny>,
    g: Bound<'_, PyAny>,
    b: Bound<'_, PyAny>,
) -> Result<(Num, Num, Num), String> {
    let r: Num = crate::pyconv::FromPy::from_py(&r)?;
    let g: Num = crate::pyconv::FromPy::from_py(&g)?;
    let b: Num = crate::pyconv::FromPy::from_py(&b)?;
    let out = port::rgb_to_yiq(r, g, b)?;
    Ok(out)
}

/// `rgb_to_yiq`.
#[pyfunction]
#[pyo3(name = "rgb_to_yiq", signature = (r, g, b))]
fn rgb_to_yiq(
    py: Python<'_>,
    r: Bound<'_, PyAny>,
    g: Bound<'_, PyAny>,
    b: Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    rgb_to_yiq_inner(r, g, b)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn yiq_to_rgb_inner(
    y: Bound<'_, PyAny>,
    i: Bound<'_, PyAny>,
    q: Bound<'_, PyAny>,
) -> Result<(Num, Num, Num), String> {
    let y: Num = crate::pyconv::FromPy::from_py(&y)?;
    let i: Num = crate::pyconv::FromPy::from_py(&i)?;
    let q: Num = crate::pyconv::FromPy::from_py(&q)?;
    let out = port::yiq_to_rgb(y, i, q)?;
    Ok(out)
}

/// `yiq_to_rgb`.
#[pyfunction]
#[pyo3(name = "yiq_to_rgb", signature = (y, i, q))]
fn yiq_to_rgb(
    py: Python<'_>,
    y: Bound<'_, PyAny>,
    i: Bound<'_, PyAny>,
    q: Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    yiq_to_rgb_inner(y, i, q)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn rgb_to_hls_inner(
    r: Bound<'_, PyAny>,
    g: Bound<'_, PyAny>,
    b: Bound<'_, PyAny>,
) -> Result<(Num, Num, Num), String> {
    let r: Num = crate::pyconv::FromPy::from_py(&r)?;
    let g: Num = crate::pyconv::FromPy::from_py(&g)?;
    let b: Num = crate::pyconv::FromPy::from_py(&b)?;
    let out = port::rgb_to_hls(r, g, b)?;
    Ok(out)
}

/// `rgb_to_hls`.
#[pyfunction]
#[pyo3(name = "rgb_to_hls", signature = (r, g, b))]
fn rgb_to_hls(
    py: Python<'_>,
    r: Bound<'_, PyAny>,
    g: Bound<'_, PyAny>,
    b: Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    rgb_to_hls_inner(r, g, b)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn hls_to_rgb_inner(
    h: Bound<'_, PyAny>,
    l: Bound<'_, PyAny>,
    s: Bound<'_, PyAny>,
) -> Result<(Num, Num, Num), String> {
    let h: Num = crate::pyconv::FromPy::from_py(&h)?;
    let l: Num = crate::pyconv::FromPy::from_py(&l)?;
    let s: Num = crate::pyconv::FromPy::from_py(&s)?;
    let out = port::hls_to_rgb(h, l, s)?;
    Ok(out)
}

/// `hls_to_rgb`.
#[pyfunction]
#[pyo3(name = "hls_to_rgb", signature = (h, l, s))]
fn hls_to_rgb(
    py: Python<'_>,
    h: Bound<'_, PyAny>,
    l: Bound<'_, PyAny>,
    s: Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    hls_to_rgb_inner(h, l, s)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn rgb_to_hsv_inner(
    r: Bound<'_, PyAny>,
    g: Bound<'_, PyAny>,
    b: Bound<'_, PyAny>,
) -> Result<(Num, Num, Num), String> {
    let r: Num = crate::pyconv::FromPy::from_py(&r)?;
    let g: Num = crate::pyconv::FromPy::from_py(&g)?;
    let b: Num = crate::pyconv::FromPy::from_py(&b)?;
    let out = port::rgb_to_hsv(r, g, b)?;
    Ok(out)
}

/// `rgb_to_hsv`.
#[pyfunction]
#[pyo3(name = "rgb_to_hsv", signature = (r, g, b))]
fn rgb_to_hsv(
    py: Python<'_>,
    r: Bound<'_, PyAny>,
    g: Bound<'_, PyAny>,
    b: Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    rgb_to_hsv_inner(r, g, b)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn hsv_to_rgb_inner(
    h: Bound<'_, PyAny>,
    s: Bound<'_, PyAny>,
    v: Bound<'_, PyAny>,
) -> Result<(Num, Num, Num), String> {
    let h: Num = crate::pyconv::FromPy::from_py(&h)?;
    let s: Num = crate::pyconv::FromPy::from_py(&s)?;
    let v: Num = crate::pyconv::FromPy::from_py(&v)?;
    let out = port::hsv_to_rgb(h, s, v)?;
    Ok(out)
}

/// `hsv_to_rgb`.
#[pyfunction]
#[pyo3(name = "hsv_to_rgb", signature = (h, s, v))]
fn hsv_to_rgb(
    py: Python<'_>,
    h: Bound<'_, PyAny>,
    s: Bound<'_, PyAny>,
    v: Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    hsv_to_rgb_inner(h, s, v)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

#[pymodule]
#[pyo3(name = "_native")]
fn native_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rgb_to_yiq, m)?)?;
    m.add_function(wrap_pyfunction!(yiq_to_rgb, m)?)?;
    m.add_function(wrap_pyfunction!(rgb_to_hls, m)?)?;
    m.add_function(wrap_pyfunction!(hls_to_rgb, m)?)?;
    m.add_function(wrap_pyfunction!(rgb_to_hsv, m)?)?;
    m.add_function(wrap_pyfunction!(hsv_to_rgb, m)?)?;
    Ok(())
}
