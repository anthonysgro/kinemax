use pyo3::prelude::*;

/// Dummy placeholder function to verify compilation lanes work.
#[pyfunction]
fn version() -> PyResult<String> {
    Ok("0.0.1-pre-alpha".to_string())
}

/// The Kinemax Core Native Extension Module
#[pymodule]
fn kinemax(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
