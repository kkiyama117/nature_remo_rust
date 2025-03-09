use std::collections::HashMap;
use std::fmt::Debug;

use crate::domain::inner::NumberOrString;
#[cfg(feature = "pyo3")]
use pyo3::{
    self,
    prelude::{FromPyObject, IntoPyObject, pyclass},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// AirCon in [`crate::domain::response::ApplianceResponse`]
#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AirConRangeMode {
    pub temp: Vec<NumberOrString>,
    pub dir: Vec<NumberOrString>,
    pub dirh: Vec<NumberOrString>,
    pub vol: Vec<NumberOrString>,
}

#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
// TODO: Implement Key enum
pub struct AirConRangeModes(HashMap<String, AirConRangeMode>);
