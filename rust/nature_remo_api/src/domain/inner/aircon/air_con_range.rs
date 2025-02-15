use crate::{
    domain::inner::AirConRangeModes, domain::inner::aircon::air_con_range_extra::AirConRangeExtra,
};
#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirConRange {
    pub modes: AirConRangeModes,
    #[cfg_attr(feature = "serde", serde(rename = "fixedButtons"))]
    pub fixed_buttons: Vec<String>,
    // Not in Swagger.
    pub extras: Vec<AirConRangeExtra>,
}
