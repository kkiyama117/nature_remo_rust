use http::uri::Uri;
#[cfg(feature = "pyo3")]
use pyo3::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[cfg_attr(feature = "pyo3", pyclass)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HTTPUri(#[serde(with = "http_serde::uri")] pub Uri);
impl From<HTTPUri> for Uri {
    fn from(value: HTTPUri) -> Self {
        value.0
    }
}
impl From<Uri> for HTTPUri {
    fn from(value: Uri) -> Self {
        Self(value)
    }
}

impl fmt::Display for HTTPUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
        // write!(f, "{}", self.0)
    }
}

impl Deref for HTTPUri {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
