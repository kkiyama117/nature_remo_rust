use std::collections::HashMap;

#[cfg(feature = "pyo3")]
use pyo3::{
    self,
    prelude::{FromPyObject, IntoPyObject},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
// TODO: Implement struct and merge
pub struct AirConSettingExtra(HashMap<String, String>);
