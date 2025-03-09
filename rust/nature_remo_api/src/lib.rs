// pub(crate) mod sealed;
pub mod domain;
/// *http* is a library that abstracts a HTTP API
/// and separates the client from the API definition.
/// This allows you to change the underlying HTTP
/// client easily.
pub mod http;

#[cfg(feature = "pyo3")]
mod python_extern;

pub mod service_provider;
