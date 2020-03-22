use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn ntoa() -> PyResult<String> {
    Ok("Hello, world!".to_string())
}
