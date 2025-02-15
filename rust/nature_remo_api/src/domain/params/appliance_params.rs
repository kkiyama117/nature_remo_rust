use crate::domain::inner::ImageId;
#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Update an appliance. Requires basic.write OAuth2 scopes.
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ApplianceParams {
    /// Basename of the image file included in the app. Ex: 'ico_ac_1'.
    pub image: Option<ImageId>,
    /// Appliance name.
    pub nickname: Option<String>,
}
