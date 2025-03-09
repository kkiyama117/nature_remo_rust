use crate::domain::inner::{AirConRangeModes, AirConSettingExtra, NumberOrString, TemperatureUnit};
use chrono::{DateTime, Local};

#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirConSettingResponse {
    /// Button.
    /// Specify 'power-off' always if you want the air conditioner powered off.
    /// Empty means powered on.
    pub button: String,
    /// AC air direction. Empty means automatic.
    pub dir: NumberOrString,
    /// AC horizontal air direction.
    pub dirh: NumberOrString,
    /// AC operation mode.
    /// The range of operation modes which the air conditioner accepts depends on the air conditioner model.
    /// Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model.
    // TODO: Use enum
    pub mode: AirConRangeModes,
    /// Temperature. The temperature in string format.
    /// The unit is described in Aircon object. The range of Temperatures which the air conditioner accepts
    /// depends on the air conditioner model and operation mode.
    /// Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model
    /// and operation mode.
    pub temp: NumberOrString,
    pub temp_unit: TemperatureUnit,
    pub updated_at: DateTime<Local>,
    /// AC air volume. Empty means automatic. Numbers express the amount of volume. The range of AirVolumes
    /// which the air conditioner accepts depends on the air conditioner model and operation mode.
    /// Check the 'AirConRangeMode' information in the response for the range of the particular
    /// air conditioner model and operation mode.
    pub vol: NumberOrString,
    /// Not in Swagger.
    pub extra: AirConSettingExtra,
}
