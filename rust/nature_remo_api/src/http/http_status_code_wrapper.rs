use http::status::StatusCode;
#[cfg(feature = "pyo3")]
use pyo3::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg_attr(feature = "pyo3", pyclass)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct HTTPStatusCode(#[serde(with = "http_serde::status_code")] StatusCode);
impl From<HTTPStatusCode> for StatusCode {
    fn from(value: HTTPStatusCode) -> Self {
        value.0
    }
}
impl From<StatusCode> for HTTPStatusCode {
    fn from(value: StatusCode) -> Self {
        Self(value)
    }
}

impl fmt::Display for HTTPStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
