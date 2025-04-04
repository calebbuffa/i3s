extern crate i3s as i3s_rs;

use pyo3::prelude::*;

#[pymodule]
fn i3s(m: &Bound<PyModule>) -> PyResult<()> {}
