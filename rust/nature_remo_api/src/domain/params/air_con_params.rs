use crate::domain::inner::TemperatureUnit;
#[cfg(feature = "pyo3")]
use pyo3::prelude::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirConParams {
    /// AC air direction. Empty means automatic.
    pub air_direction: Option<String>,
    // TODO: Implement custom class
    /// AC horizontal air direction.
    pub air_direction_h: Option<String>,
    /// AC air volume.
    /// Empty means automatic. Numbers express the amount of volume.
    /// The range of AirVolumes which the air conditioner accepts depends on the air conditioner
    /// model and operation mode. Check the 'AirConRangeMode' information in the response for the
    /// range of the particular air conditioner model and operation mode.
    pub air_volume: Option<String>,
    // TODO: Implement custom class
    /// Button.
    /// Specify 'power-off' always if you want the air conditioner powered off.
    /// Empty means powered on.
    pub button: Option<String>,
    /// AC operation mode.
    /// The range of operation modes which the air conditioner accepts depends on the air
    /// conditioner model. Check the 'AirConRangeMode' information in the response for the range of
    /// the particular air conditioner model.
    pub operation_mode: Option<String>,
    /// Temperature.
    /// The temperature in string format. The unit is described in Aircon object.
    /// The range of Temperatures which the air conditioner accepts depends on the air conditioner
    /// model and operation mode.
    /// Check the 'AirConRangeMode' information in the response for the range
    /// of the particular air conditioner model and operation mode.
    pub temperature: Option<String>,
    /// Temperature unit. 'c' or 'f' or '' for unknown.
    pub temperature_unit: Option<TemperatureUnit>,
}
