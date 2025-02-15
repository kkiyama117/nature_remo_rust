#[cfg(feature = "pyo3")]
use pyo3::prelude::{pyclass, pymethods};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// Implementation of domain for [crate::service_provider::post_device_request].
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all, str))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct TemperatureOffsetParams {
    /// Temperature offset value added to the measured temperature.
    pub offset: f32,
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl TemperatureOffsetParams {
    #[new]
    fn new(offset: f32) -> Self {
        Self { offset }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl Display for TemperatureOffsetParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device Offset Request( offset:{:?})", self.offset,)
    }
}
