#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[cfg_attr(feature = "pyo3", pyclass(eq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemperatureUnit {
    #[cfg_attr(feature = "serde", serde(rename = "c"))]
    Celsius,
    #[cfg_attr(feature = "serde", serde(rename = "f"))]
    Fahrenheit,
    #[cfg_attr(feature = "serde", serde(rename = ""))]
    Unknown,
}
