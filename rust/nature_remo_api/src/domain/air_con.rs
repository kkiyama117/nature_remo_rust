use super::inner::{AirConRangeModes, StringOrBool, TemperatureUnit};
#[cfg(feature = "pyo3")]
use pyo3;
#[cfg(feature = "pyo3")]
use pyo3::prelude::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirCon {
    pub range: AirConRange,
    #[cfg(feature = "pyo3")]
    #[cfg_attr(feature = "serde", serde(rename = "tempUnit"))]
    #[pyo3(name = "tempUnit")]
    pub temp_unit: TemperatureUnit,
    #[cfg(not(feature = "pyo3"))]
    #[cfg_attr(feature = "serde", serde(rename = "tempUnit"))]
    pub temp_unit: TemperatureUnit,
}

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirConRange {
    pub modes: AirConRangeModes,
    #[cfg_attr(feature = "serde", serde(rename = "fixedButtons"))]
    pub fixed_buttons: Vec<String>,
    // Not in Swagger
    pub extras: Vec<AirConRangeExtra>,
}

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirConRangeExtra {
    // TODO: Use Enum
    availability: String,
    pub description: String,
    pub id: String,
    // TODO: Implement
    options: Vec<HashMap<String, StringOrBool>>,
    pub text: String,
    // TODO: Use Enum, Avoid dirty hack
    #[cfg(feature = "pyo3")]
    #[pyo3(name = "type")]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub _type: String,
    #[cfg(not(feature = "pyo3"))]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub _type: String,
}

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirconSmartEcoMode {
    pub adjusting: bool,
    /// Maximum is 255, minimum: 0
    pub area: u8,
    pub enabled: bool,
}
