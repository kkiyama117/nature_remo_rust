#![allow(unused_imports)]
//! Define models used in other models between multi models.

// #[cfg(feature = "macros")]
#[macro_use]
mod macros;

mod air_con_smart_eco_mode;
mod aircon;
mod appliance;
mod country;
mod device;
mod image_id;
mod mixed_types;
mod privacy_policy;
mod sensor_value;
mod signal_id;
mod temperature_unit;

pub mod prelude {
    pub(crate) use super::appliance::appliance_id::ApplianceIds;
    pub(crate) use super::signal_id::SignalIds;
    pub use super::{
        air_con_smart_eco_mode::AirconSmartEcoMode,
        aircon::{
            AirCon, air_con_range::AirConRange, air_con_range_extra::AirConRangeExtra,
            air_con_range_mode::AirConRangeMode, air_con_range_mode::AirConRangeModes,
            air_con_setting_extra::AirConSettingExtra,
        },
        appliance::{
            appliance_id::ApplianceId, appliance_model::ApplianceModel,
            appliance_type::ApplianceType,
        },
        country::Country,
        device::device_id::DeviceId,
        image_id::ImageId,
        mixed_types::*,
        privacy_policy::PrivacyPolicy,
        sensor_value::{SensorValue, SensorValueDict},
        signal_id::SignalId,
        temperature_unit::TemperatureUnit,
    };
}
// Re-export
pub(crate) use prelude::*;
