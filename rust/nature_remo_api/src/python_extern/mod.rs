#![cfg(all(feature = "serde", feature = "pyo3"))]
//! This module is only loaded if features includes "pyo3"
//! In this module, we can use attributes of pyo3, serde, and some dependencies
//! set in Cargo.toml.
mod client;
#[cfg(feature = "serde")]
mod service_provider;
use log::LevelFilter;
use pyo3::{create_exception, prelude::*};
use pyo3_log::{Caching, Logger};

// NatureRemoError for Python
create_exception!(
    python_binding,
    NatureRemoError,
    pyo3::exceptions::PyException
);

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_rust_api")]
mod nature_remo_api {
    use super::*;

    #[pymodule_export]
    use crate::python_extern::{NatureRemoError, client::PyNatureRemoAPI};

    #[pymodule]
    mod domain {
        use super::*;
        #[pymodule_export]
        use crate::{
            domain::inner::{
                AirConRangeMode,
                // ApplianceId,
                ApplianceType,
                // Country
                // DeviceId,
                // ImageId,
                // SignalId,
                TemperatureUnit,
            },
            domain::params::{
                AirConParams, ApplianceParams, CreateApplianceRequest, CreateSignalParameters,
                DetectApplianceRequest, ReorderAppliancesParams, ReorderSignalsParams,
                UpdateProfileParam,
            },
            domain::response::{
                AirConSettingResponse, ApplianceModelAndParam, ApplianceResponse, DeviceResponse,
                Signal, UserResponse,
            },
            // Some wrapper types are hidden because we use native classes of Python like `list`.
            python_extern::service_provider::PyNatureRemoRequest,
        };

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("nature_remo_api.domain", m)
            })
        }
    }

    /// Wrapper of HTTP Signature
    #[pymodule]
    mod request_generator {
        use super::*;
        #[pymodule_export]
        use crate::python_extern::service_provider::{
            PyNatureRemoRequest, get_user_request, post_user_request,
        };

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("nature_remo_api.request_generator", m)
            })
        }
    }
    /// Wrapper of HTTP Signature
    #[pymodule]
    mod http {
        use super::*;
        #[pymodule_export]
        use crate::http::{HTTPMethod, HTTPStatusCode, RateLimit};

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("nature_remo_api.http", m)
            })
        }
    }

    #[pymodule]
    mod tester {
        use super::super::*;
        #[pymodule_export]
        use super::super::{call_rust_sleep, rust_sleep};
        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("nature_remo_api.tester", m)
            })
        }
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // pyo3_log::init();
        Python::with_gil(|py| {
            // Some time in the future when logging changes, reset the caches:
            let handle = Logger::new(py, Caching::LoggersAndLevels)?
                .filter(LevelFilter::Debug)
                .filter_target("nature_remo_rust".to_owned(), LevelFilter::Debug)
                .install()
                .expect("Someone installed a logger before us :-(");
            handle.reset();
            log::debug!("Initialize Nature Remo PyO3 Rust");
            // import to `sys.modules` for `from` import
            py.import("sys")?
                .getattr("modules")?
                .set_item("nature_remo_api._rust_api", m)
        })?;
        log::debug!("Initialized");
        Ok(())
    }
}

async fn rust_sleep2() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

#[pyfunction]
fn call_rust_sleep(py: Python) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        rust_sleep2().await;
        Ok(())
    })
}

#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    })
}
