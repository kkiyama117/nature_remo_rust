mod device_service;
mod user_service;

use crate::{http::HTTPMethod, service_provider as normal};
use http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use pyo3::prelude::*;
use pythonize::pythonize;
use serde::{self, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
// Re-export each services
pub(crate) use user_service::*;

#[pyclass(name = "NatureRemoRequest", get_all)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct PyNatureRemoRequest {
    endpoint: String,
    http_method: HTTPMethod,
    headers: HashMap<String, String>,
    data: Option<PyObject>,
}

#[pymethods]
impl PyNatureRemoRequest {
    #[new]
    #[pyo3(signature = (endpoint, access_token, http_method, data=None))]
    fn new(
        endpoint: String,
        access_token: String,
        http_method: HTTPMethod,
        data: Option<PyObject>,
    ) -> Self {
        Self {
            endpoint,
            http_method,
            headers: default_header_map(access_token),
            data,
        }
    }
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl<'py, T: Serialize> IntoPyObject<'py> for normal::NatureRemoRequest<T>
where
    T: Serialize,
{
    type Target = PyNatureRemoRequest;
    // in most cases this will be `Bound`
    type Output = Bound<'py, Self::Target>;
    // the conversion error type, has to be convertable to `PyErr`
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Bound::new(
            py,
            PyNatureRemoRequest {
                endpoint: self.url_tail.to_string(),
                http_method: self.http_method,
                // headers: http_serde::header_map::serialize(&self.headers, Pythonizer::new(py))?
                //     .unbind(),
                headers: self
                    .headers
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                    .collect(),
                data: match self.data {
                    None => None,
                    Some(data_value) => Some(pythonize(py, &data_value)?.into()),
                },
            },
        )
    }
}

fn default_header_map<'a>(access_token: impl Into<Cow<'a, str>>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert(
        AUTHORIZATION.to_string(),
        format!("Bearer {}", access_token.into()),
    );
    headers.insert(ACCEPT.to_string(), "application/json".to_string());
    headers.insert(
        USER_AGENT.to_string(),
        "nature-remo/1 (nature_remo_rust)".to_string(),
    );
    headers
}
