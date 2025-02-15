use crate::domain::inner::{ApplianceType, DeviceId, ImageId};
#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CreateApplianceRequest {
    pub device: Option<DeviceId>,
    /// Basename of the image file included in the app. Ex: 'ico_ac_1'.
    pub image: Option<ImageId>,
    /// ApplianceModel ID if the appliance we're trying to create is included in IRDB.
    pub model: Option<String>,
    /// Enum of 'AC', 'TV', 'Light'
    pub model_type: Option<ApplianceType>,
    /// Appliance name.
    pub nickname: Option<String>,
}
