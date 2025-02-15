use chrono::{DateTime, Utc};
use http::header::{HeaderMap, HeaderValue};
#[cfg(feature = "pyo3")]
use pyo3::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(Clone, Debug, Default)]
pub struct RateLimit {
    pub limit: isize,
    pub remaining: isize,
    pub reset: DateTime<Utc>,
}

impl TryFrom<&RateLimit> for HeaderMap {
    type Error = std::convert::Infallible;

    fn try_from(value: &RateLimit) -> Result<Self, Self::Error> {
        let mut result = HeaderMap::new();
        result.insert("X-Rate-Limit-Limit", HeaderValue::from(value.limit));
        result.insert("X-Rate-Limit-Remaining", HeaderValue::from(value.remaining));
        result.insert(
            "X-Rate-Limit-Reset",
            HeaderValue::from(value.reset.timestamp()),
        );
        Ok(result)
    }
}
impl TryFrom<&HeaderMap> for RateLimit {
    type Error = std::convert::Infallible;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let limit = headers
            .get("X-Rate-Limit-Limit")
            .unwrap()
            .to_str()
            .ok()
            .unwrap()
            .parse()
            .ok()
            .unwrap();
        let remaining = headers
            .get("X-Rate-Limit-Remaining")
            .unwrap()
            .to_str()
            .ok()
            .unwrap()
            .parse()
            .ok()
            .unwrap();
        let reset_timestamp = headers
            .get("X-Rate-Limit-Reset")
            .unwrap()
            .to_str()
            .ok()
            .unwrap()
            .parse::<i64>()
            .ok()
            .unwrap();

        let reset = DateTime::from_timestamp(reset_timestamp, 0).unwrap();
        Ok(Self {
            limit,
            remaining,
            reset,
        })
    }
}
