use crate::domain::inner::{ApplianceId, Country, ImageId};
#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ApplianceModel {
    pub country: Country,
    pub id: ApplianceId,
    pub image: ImageId,
    pub manufacturer: String,
    pub name: String,
    pub remote_name: String,
    pub series: String,
}
