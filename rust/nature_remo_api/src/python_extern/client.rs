#![allow(dead_code)]
#![cfg(feature = "pyo3")]

// use http_client::{Error, HttpClient, Request, Response};
use crate::http::RateLimit;
use pyo3::prelude::*;
use std::fmt::Debug;

#[derive(Debug)]
struct PyHTTPRequest {
    inner: Py<PyAny>,
}
#[derive(Debug)]
struct PyHTTPClient {
    client: Py<PyAny>,
}

// impl HttpClient for PyHTTPClient {
//     async fn send(&self, req: Request) -> Result<Response, Error> {
//         Python::with_gil(|py| {
//             self.client.bind(py).call_method("")
//         })
//     }
// }

#[pyclass]
#[pyo3(name = "NatureRemoAPIClient")]
pub struct PyNatureRemoAPI {
    // http_client: PyAny,
    access_token: String,
    base_url: String,
    last_rate_limit: Option<RateLimit>,
}
