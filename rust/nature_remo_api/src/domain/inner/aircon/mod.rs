pub(crate) mod air_con_range_mode;

pub(crate) mod air_con_range;
pub(crate) mod air_con_range_extra;
pub(crate) mod air_con_setting_extra;

use crate::domain::inner::{TemperatureUnit, aircon::air_con_range::AirConRange};
#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
