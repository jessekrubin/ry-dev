use pyo3::types::PyModule;
use pyo3::{PyResult, Python};

#[cfg(feature = "jiter")]
mod jiter_ry;

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "jiter")]
    jiter_ry::madd(_py, m)?;

    Ok(())
}
