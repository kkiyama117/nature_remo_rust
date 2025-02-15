use crate::domain::inner::StringOrBool;
#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
