// wrapped_string!(Country);
#[cfg(feature = "pyo3")]
use pyo3::{
    self,
    prelude::{FromPyObject, IntoPyObject},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Appliance ID String wrapper
#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Country(String);

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl From<String> for Country {
    fn from(value: String) -> Self {
        Country(value)
    }
}
