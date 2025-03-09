#[cfg(feature = "pyo3")]
use pyo3::prelude::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Appliance types. AC, TV, LIGHT, etc.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass(eq, eq_int))]
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ApplianceType {
    #[serde(rename = "AC")]
    AirCon,
    #[serde(rename = "TV")]
    TV,
    #[serde(rename = "LIGHT")]
    Light,
    #[serde(rename = "IR")]
    IR,
}
