use crate::domain::inner::{ApplianceId, ApplianceIds};
#[cfg(feature = "pyo3")]
use pyo3::prelude::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// List of all appliance IDs, comma separated.
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ReorderAppliancesParams {
    appliances: Option<ApplianceIds>,
}

impl<'a, S> From<S> for ReorderAppliancesParams
where
    S: Iterator<Item = &'a ApplianceId>,
{
    fn from(value: S) -> Self {
        Self {
            appliances: Some(ApplianceIds::from(value)),
        }
    }
}
