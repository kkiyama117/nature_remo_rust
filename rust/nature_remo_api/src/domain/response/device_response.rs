use crate::domain::inner::{DeviceId, SensorValueDict};
use crate::domain::response::UserResponses;
use chrono::{DateTime, Utc};
#[cfg(feature = "pyo3")]
use pyo3::prelude::{FromPyObject, IntoPyObject, pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DeviceResponse {
    pub bt_mac_address: String,
    pub created_at: DateTime<Utc>,
    pub firmware_version: String,
    pub humidity_offset: f32,
    pub id: DeviceId,
    pub mac_address: String,
    pub name: String,
    // Set Null if called by `get_device_appliances` etc.
    pub newest_events: Option<SensorValueDict>,
    // Set Null if called by `get_device_appliances` etc.
    pub online: Option<bool>,
    pub serial_number: String,
    pub temperature_offset: f32,
    pub updated_at: DateTime<Utc>,
    /// Deprecated. Do not Use in new code.
    users: Option<UserResponses>,
}

impl PartialEq for DeviceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Display for DeviceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Device(id={})", self.id)
    }
}

#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
/// Wrapper of [Vec<DeviceResponse>]. In python, same as `list[Device]`
pub struct DeviceResponses(Vec<DeviceResponse>);

nature_response_list_domain!(DeviceResponse, DeviceResponses);
