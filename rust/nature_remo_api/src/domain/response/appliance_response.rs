use crate::domain::air_con::{AirCon, AirconSmartEcoMode};
use crate::domain::inner::{ApplianceId, ApplianceModel, ApplianceType};
use crate::domain::response::{DeviceResponse, Signals};
#[cfg(feature = "pyo3")]
use pyo3::{
    self,
    prelude::{FromPyObject, IntoPyObject, pyclass},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ApplianceResponse {
    // Set Null if called by `get_device_appliances`
    pub aircon: AirCon,
    pub aircon_smart_eco_mode: Option<AirconSmartEcoMode>,
    // TODO: pub ble: BLE
    // Set Null if called by `get_device_appliances`
    pub device: Option<DeviceResponse>,
    // TODO: pub echonetlite: Option<EchoNetLite>
    pub id: ApplianceId,
    pub image: String,
    // TODO: pub light: Option<Light>,
    // TODO: pub light_projector: Option<Light>,
    // Set Null if called by `get_device_appliances`
    pub model: Option<ApplianceModel>,
    // pub mornin_plus: Option<MorninPlus>,
    pub nickname: String,
    // pub qrio_lock: Option<String>,
    // Set Null if called by `get_device_appliances`
    // pub settings: Option<AirConSetting>,
    pub signals: Signals,
    // TODO: pub smart_meter: Option<SmartMeter>,
    // TODO: pub tv: Option<TV>
    // TODO: Avoid dirty hack
    // https://github.com/PyO3/pyo3/issues/1003
    /// Appliance types. AC, TV, LIGHT, etc.
    #[cfg(feature = "pyo3")]
    #[pyo3(name = "type")]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub _type: ApplianceType,
    #[cfg(not(feature = "pyo3"))]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub _type: ApplianceType,
}

impl PartialEq for ApplianceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Display for ApplianceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApplianceResponse(id='{}')", self.id)
    }
}

#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
/// Wrapper of [Vec<ApplianceResponse>]. In python, same as `list[Appliance]`
pub struct ApplianceResponses(Vec<ApplianceResponse>);

nature_response_list_domain!(ApplianceResponse, ApplianceResponses);
