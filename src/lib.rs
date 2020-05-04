use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn print_this(string: &str) -> PyResult<String> {
    // Print with rust code
    println!("{}", string);

    // Send the following string back to the calling 
    // python code
    Ok("Hello, world".to_string())
}

#[pymodule]
// This is necessary to define the ffi
fn template_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(print_this))?;

    Ok(())
}
