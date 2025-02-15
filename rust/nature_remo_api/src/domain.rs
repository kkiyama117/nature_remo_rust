use chrono::{DateTime, Local};
#[cfg(feature = "python")]
use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Enum with all the standard HTTP methods. It also has
/// a variant `Custom` to support non-standard methods.
#[cfg_attr(feature = "python", pyclass(eq, eq_int))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Trace,
    Connect,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match *self {
            Method::Get => "GET".to_string(),
            Method::Head => "HEAD".to_string(),
            Method::Post => "POST".to_string(),
            Method::Put => "PUT".to_string(),
            Method::Delete => "DELETE".to_string(),
            Method::Patch => "PATCH".to_string(),
            Method::Options => "OPTIONS".to_string(),
            Method::Trace => "TRACE".to_string(),
            Method::Connect => "CONNECT".to_string(),
            // Method::Custom(ref s) => s.clone(),
        };
        write!(f, "{}", str)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Clone, Debug, Default)]
pub struct RateLimit {
    pub checked_at: Option<DateTime<Local>>,
    pub limit: Option<isize>,
    pub remaining: Option<isize>,
    pub reset: Option<DateTime<Local>>,
}
