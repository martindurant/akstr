use numpy::ndarray::{ArrayD, ArrayViewD, ArrayView1, ArrayViewMutD, Array1};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use numpy::PyArray1;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyByteArray};
use pyo3::ffi::PyBuffer_GetPointer;
use std::str;

// https://docs.rs/Inflector/latest/inflector/cases/sentencecase/fn.to_sentence_case.html

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


#[pyfunction]
fn upper(py: Python, ob: &PyAny) -> PyResult<PyObject>
{
    let x: &PyArray1<u8> = ob.extract()?;
    let inner : &[u8]= unsafe {x.as_slice()?};
    let s = unsafe {str::from_utf8_unchecked(inner)};
    Ok(s.to_uppercase().as_bytes().into_py(py))
}


/// A Python module implemented in Rust.
#[pymodule]
fn akstr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(upper, m)?)?;
    Ok(())
}
