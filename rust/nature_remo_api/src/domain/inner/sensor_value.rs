use chrono::{DateTime, Utc};
#[cfg(feature = "pyo3")]
use pyo3::{
    prelude::{Bound, FromPyObject, IntoPyObject, PyAny, PyResult, pyclass, pymethods},
    types::PyType,
};
#[cfg(feature = "pyo3")]
use pythonize::depythonize;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct SensorValue {
    pub val: f64,
    pub created_at: DateTime<Utc>,
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl SensorValue {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    #[cfg(feature = "serde")]
    #[classmethod]
    fn from_json(_cls: &Bound<PyType>, json: &Bound<PyAny>) -> PyResult<Self> {
        let result = depythonize(json)?;
        Ok(result)
    }
}

impl Display for SensorValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SensorValue(val={}, created_at={})",
            self.val, self.created_at
        )
    }
}

/// The SensorValue
/// key means 'te' = temperature, 'hu' = humidity, 'il' = illumination, 'mo' = movement.
///
/// The val of 'mo' is always 1 and when movement event is captured created_at is updated.
#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
// TODO: Use enum
pub struct SensorValueDict(HashMap<String, SensorValue>);

impl Display for SensorValueDict {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec = &self.0;
        write!(f, "{{")?;
        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, (key, v)) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", key, v)?;
        }
        write!(f, "}}")
    }
}
