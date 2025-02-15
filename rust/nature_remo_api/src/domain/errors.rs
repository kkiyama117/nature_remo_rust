use crate::http::HTTPStatusCode;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// Wrapper of Error by API
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Error)]
pub struct NatureRemoAPIError {
    pub status_code: HTTPStatusCode,
    pub code: usize,
    pub message: String,
}

impl Display for NatureRemoAPIError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP Status Code: {}, Nature Remo Code: {}, Message: {}",
            self.status_code, self.code, self.message
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Inner {
    pub code: usize,
    pub message: String,
}
