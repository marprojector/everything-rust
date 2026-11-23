//! Python bindings for `src/textkit/`: generated from the signatures in `port.rs` by everything-rust.
//! Regenerate with `everything-rust refresh` after changing them.

use pyo3::prelude::*;

pub mod js;
pub mod port;

/// Batches allocate on all cores at once; mimalloc keeps that from serializing.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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

fn slugify_inner(
    text: String,
    sep: Option<String>,
    max_length: Option<i64>,
) -> Result<String, String> {
    let out = port::slugify(text, sep, max_length)?;
    Ok(out)
}

/// `slugify`.
#[pyfunction]
#[pyo3(name = "slugify", signature = (text, sep=String::from("-"), *, max_length=None))]
fn slugify(text: String, sep: Option<String>, max_length: Option<i64>) -> PyResult<String> {
    slugify_inner(text, sep, max_length).map_err(error)
}

fn word_counts_inner(text: String, min_length: i64) -> Result<serde_json::Value, String> {
    let out = port::word_counts(text, min_length)?;
    Ok(out)
}

/// `word_counts`.
#[pyfunction]
#[pyo3(name = "word_counts", signature = (text, min_length=1))]
fn word_counts(py: Python<'_>, text: String, min_length: i64) -> PyResult<Py<PyAny>> {
    word_counts_inner(text, min_length)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn chunk_inner(items: Vec<i64>, size: i64) -> Result<Vec<Vec<i64>>, String> {
    let out = port::chunk(items, size)?;
    Ok(out)
}

/// `chunk`.
#[pyfunction]
#[pyo3(name = "chunk", signature = (items, size))]
fn chunk(py: Python<'_>, items: Vec<i64>, size: i64) -> PyResult<Py<PyAny>> {
    chunk_inner(items, size)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn page_count_inner(total: i64, per_page: i64) -> Result<i64, String> {
    let out = port::page_count(total, per_page)?;
    Ok(out)
}

/// `page_count`.
#[pyfunction]
#[pyo3(name = "page_count", signature = (total, per_page))]
fn page_count(total: i64, per_page: i64) -> PyResult<i64> {
    page_count_inner(total, per_page).map_err(error)
}

fn mean_inner(values: Vec<f64>) -> Result<f64, String> {
    let out = port::mean(values)?;
    Ok(out)
}

/// `mean`.
#[pyfunction]
#[pyo3(name = "mean", signature = (values))]
fn mean(values: Vec<f64>) -> PyResult<f64> {
    mean_inner(values).map_err(error)
}

fn stack_new_inner(capacity: i64) -> Result<port::Stack, String> {
    let out = port::Stack::new(capacity)?;
    Ok(out)
}

fn stack_push_inner(inner: &mut port::Stack, item: String) -> Result<i64, String> {
    let out = inner.push(item)?;
    Ok(out)
}

fn stack_pop_inner(inner: &mut port::Stack) -> Result<Option<String>, String> {
    let out = inner.pop()?;
    Ok(out)
}

fn stack_peek_inner(inner: &port::Stack) -> Result<Option<String>, String> {
    let out = inner.peek()?;
    Ok(out)
}

fn stack_size_inner(inner: &port::Stack) -> Result<i64, String> {
    let out = inner.size()?;
    Ok(out)
}

/// Class `Stack`.
#[pyclass(name = "Stack")]
pub struct Stack {
    inner: port::Stack,
}

#[pymethods]
impl Stack {
    #[new]
    #[pyo3(signature = (capacity=8))]
    fn new(capacity: i64) -> PyResult<Self> {
        stack_new_inner(capacity)
            .map(|inner| Self { inner })
            .map_err(error)
    }

    #[pyo3(name = "push", signature = (item))]
    fn push(&mut self, item: String) -> PyResult<i64> {
        stack_push_inner(&mut self.inner, item).map_err(error)
    }

    #[pyo3(name = "pop", signature = ())]
    fn pop(&mut self) -> PyResult<Option<String>> {
        stack_pop_inner(&mut self.inner).map_err(error)
    }

    #[pyo3(name = "peek", signature = ())]
    fn peek(&self) -> PyResult<Option<String>> {
        stack_peek_inner(&self.inner).map_err(error)
    }

    #[pyo3(name = "size", signature = ())]
    fn size(&self) -> PyResult<i64> {
        stack_size_inner(&self.inner).map_err(error)
    }
}

/// The items of `SlugifyBatch` as the port takes them; the port runs per item. Define
/// `pub struct SlugifyIndex` in `port.rs` (with `new` and `run`) to prepare items once.
pub struct SlugifyDefaultIndex {
    items: Vec<String>,
}

impl SlugifyDefaultIndex {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }

    pub fn run(&self, sep: Option<String>, max_length: Option<i64>) -> Result<Vec<String>, String> {
        use rayon::prelude::*;
        self.items
            .par_iter()
            .map(|item| port::slugify(item.clone(), sep.clone(), max_length))
            .collect::<Vec<_>>()
            .into_iter()
            .collect()
    }
}

/// `slugify` over many `text` values kept in Rust: `run` maps them all in one call, on all
/// cores, without the GIL; `rank` returns the indexes of results >= `min`, best first.
#[pyclass(name = "SlugifyBatch")]
pub struct SlugifyBatch {
    inner: SlugifyDefaultIndex,
}

#[pymethods]
impl SlugifyBatch {
    #[new]
    fn new(items: Vec<String>) -> Self {
        Self {
            inner: SlugifyDefaultIndex::new(items),
        }
    }

    /// The result for every item, in order.
    fn run(
        &self,
        py: Python<'_>,
        sep: Option<String>,
        max_length: Option<i64>,
    ) -> PyResult<Vec<String>> {
        py.detach(|| self.inner.run(sep, max_length)).map_err(error)
    }
}

/// The items of `PageCountBatch` as the port takes them; the port runs per item. Define
/// `pub struct PageCountIndex` in `port.rs` (with `new` and `run`) to prepare items once.
pub struct PageCountDefaultIndex {
    items: Vec<i64>,
}

impl PageCountDefaultIndex {
    pub fn new(items: Vec<i64>) -> Self {
        Self { items }
    }

    pub fn run(&self, per_page: i64) -> Result<Vec<i64>, String> {
        use rayon::prelude::*;
        self.items
            .par_iter()
            .map(|item| port::page_count(*item, per_page))
            .collect::<Vec<_>>()
            .into_iter()
            .collect()
    }
}

/// `page_count` over many `total` values kept in Rust: `run` maps them all in one call, on all
/// cores, without the GIL; `rank` returns the indexes of results >= `min`, best first.
#[pyclass(name = "PageCountBatch")]
pub struct PageCountBatch {
    inner: PageCountDefaultIndex,
}

#[pymethods]
impl PageCountBatch {
    #[new]
    fn new(items: Vec<i64>) -> Self {
        Self {
            inner: PageCountDefaultIndex::new(items),
        }
    }

    /// The result for every item, in order.
    fn run(&self, py: Python<'_>, per_page: i64) -> PyResult<Vec<i64>> {
        py.detach(|| self.inner.run(per_page)).map_err(error)
    }

    /// Indexes of the items whose result is at least `min`, highest result first.
    fn rank(&self, py: Python<'_>, per_page: i64, min: f64) -> PyResult<Vec<u32>> {
        let out = py.detach(|| self.inner.run(per_page)).map_err(error)?;
        let mut hits: Vec<(f64, u32)> = out
            .into_iter()
            .enumerate()
            .map(|(i, r)| (r as f64, u32::try_from(i).unwrap_or(u32::MAX)))
            .filter(|(r, _)| *r >= min)
            .collect();
        // Stable: equal results keep the items' order, as JS `sort` and Python `sorted` do.
        hits.sort_by(|a, b| b.0.total_cmp(&a.0));
        let order: Vec<u32> = hits.into_iter().map(|(_, i)| i).collect();
        Ok(order)
    }
}

#[pymodule]
#[pyo3(name = "_native")]
fn native_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(slugify, m)?)?;
    m.add_function(wrap_pyfunction!(word_counts, m)?)?;
    m.add_function(wrap_pyfunction!(chunk, m)?)?;
    m.add_function(wrap_pyfunction!(page_count, m)?)?;
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_class::<Stack>()?;
    m.add_class::<SlugifyBatch>()?;
    m.add_class::<PageCountBatch>()?;
    Ok(())
}
