use numpy::IntoPyArray;
use numpy::PyArray1;

use pyo3::prelude::*;
use std::str;

// https://docs.rs/Inflector/latest/inflector/cases/sentencecase/fn.to_sentence_case.html

// This is should be a macro
#[inline]
fn to_slice<T>(data: &PyAny) -> &[T]
    where T: numpy::Element
{
    let x: &PyArray1<T> = data.extract().unwrap();
    unsafe {x.as_slice().unwrap()}
}

// This is should be a macro
#[inline]
fn to_slice_mut<T>(data: &PyAny) -> &mut [T]
    where T: numpy::Element
{
    let x: &PyArray1<T> = data.extract().unwrap();
    unsafe {x.as_slice_mut().unwrap()}
}

/// data: uint8 data containing text
/// starts: int64 offsets of string starts
/// stops: int64 offsets of string ends
/// out: int64 offsets array for contiguous output (length 1+ starts)
/// returns: new uint8 np array
#[pyfunction]
fn upper(py: Python, data: &PyAny, starts: &PyAny, stops: &PyAny, out: &PyAny) -> PyResult<PyObject>
{
    let d: &[u8] = to_slice(data);
    let s: &[i64] = to_slice(starts);
    let ss: &[i64] = to_slice(stops);
    let o: &mut [i64] = to_slice_mut(out);
    let mut outdata: Vec<u8> = Vec::with_capacity(d.len());
    o[0] = 0;
    for (i, (start, stop)) in (s).iter().zip(ss).enumerate() {
        let s = unsafe {str::from_utf8_unchecked(&d[*start as usize..*stop as usize])};
        outdata.extend_from_slice(s.to_uppercase().as_bytes());
        o[i+1] = outdata.len() as i64;
    }

    Ok(PyArray1::from_vec(py,outdata).into_py(py))
}

#[pyfunction]
fn len(py: Python, data: &PyAny, offsets: &PyAny, out: &PyAny) -> PyResult<PyObject>
{
    let inner : &[u8] = to_slice(data);
    let offs : &[i64]= to_slice(offsets);
    let ou : &mut [i64]= to_slice_mut(out);
    let mut it = offs.iter();
    let mut pos:usize = *(it.next().unwrap()) as usize;
    for (i, off) in it.enumerate() {
        let s = unsafe {str::from_utf8_unchecked(&inner[pos..*off as usize])};
        ou[i] = s.len() as i64;
        pos = *off as usize;
    }
    Ok(out.into_py(py))
}

#[pyfunction]
fn make_new(py: Python) -> PyResult<&PyArray1<u8>>
{
    let v: Vec<u8> = vec![1, 2, 3];
    Ok(PyArray1::from_vec(py, v))
}

/// A Python module implemented in Rust.
#[pymodule]
fn akstr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(upper, m)?)?;
    m.add_function(wrap_pyfunction!(len, m)?)?;
    Ok(())
}
