#[cfg(feature = "pyo3")]
use pyo3::{
    prelude::{Bound, PyAny, PyResult, pyclass, pymethods},
    types::PyType,
};
#[cfg(feature = "pyo3")]
use pythonize::depythonize;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct IRSignal {
    pub freq: isize,
    pub data: Vec<isize>,
    pub format: String,
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl IRSignal {
    #[new]
    fn new(freq: isize, data: Vec<isize>, format: String) -> Self {
        Self { freq, data, format }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    #[classmethod]
    fn from_json(_cls: &Bound<PyType>, json: &Bound<PyAny>) -> PyResult<Self> {
        let result = depythonize(json)?;
        Ok(result)
    }
}
