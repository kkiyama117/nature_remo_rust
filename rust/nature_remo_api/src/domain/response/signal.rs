use crate::domain::inner::SignalId;
#[cfg(feature = "pyo3")]
use pyo3::{
    self,
    prelude::{FromPyObject, IntoPyObject, pyclass},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Signal is used in both params and responses. So this is re-exported in both modules
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Signal {
    id: SignalId,
    image: String,
    name: String,
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Appliance(id='{}')", self.id)
    }
}

#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
/// Wrapper of [Vec<Signal>]. In python, same as `list[Signal]`
pub struct Signals(Vec<Signal>);

impl Display for Signals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}
