use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn ntoa() -> PyResult<String> {
    Ok("Hello, world!".to_string())
}

#[pymodule]
// This is necessary to define the ffi
fn test_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(ntoa))?;

    Ok(())
}
