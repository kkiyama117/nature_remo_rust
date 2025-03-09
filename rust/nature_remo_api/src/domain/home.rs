#[cfg(feature = "pyo3")]
use pyo3::prelude::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Implementation of Home domain.
///
/// `superuser` is optional.
/// This attribute appears in [crate::domain::response::Device]'s attribute.
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Home {
    pub id: String,
    pub location: String,
    pub nickname: String,
    pub temp_unit: String,
    /// This is not in Swagger
    pub superuser: Option<bool>,
}

impl PartialEq for Home {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
