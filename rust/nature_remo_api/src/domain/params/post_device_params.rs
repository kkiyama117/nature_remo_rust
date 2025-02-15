#[cfg(feature = "pyo3")]
use pyo3::prelude::{pyclass, pymethods};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all, str))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateDeviceParameters {
    pub name: Option<String>,
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl UpdateDeviceParameters {
    #[new]
    #[pyo3(signature = (name=None))]
    fn new(name: Option<String>) -> Self {
        Self { name }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl Display for UpdateDeviceParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PostDeviceParam( name:{:?})", self.name,)
    }
}
