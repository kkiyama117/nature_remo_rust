use nature_remo_api_raw::add as raw_add;
use pyo3::prelude::*;
use pyo3_log;

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok(format!("1 + 1 = {}", raw_add(1, 1)))
}

#[pyfunction]
fn add() -> PyResult<u64> {
    Ok(raw_add(1, 2))
}

/// A Python module implemented in Rust.
#[pymodule]
// #[pyo3(module = "_rust_api")]
mod _rust_api {
    use log::debug;
    use super::*;
    #[pymodule_export]
    use super::{add, hello};
    #[pymodule_export]
    use crate::domain::{Method, RateLimit};
    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>)->PyResult<()>{
        pyo3_log::init();
        debug!("Initialize Nature Remo PyO3 Rust");
        Ok(())
    }
}
