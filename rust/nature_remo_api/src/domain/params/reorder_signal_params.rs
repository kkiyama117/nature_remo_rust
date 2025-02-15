use crate::domain::inner::{SignalId, SignalIds};
#[cfg(feature = "pyo3")]
use pyo3::prelude::pyclass;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// List of all appliance IDs, comma separated.
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ReorderSignalsParams {
    signals: Option<SignalIds>,
}

impl<'a, S> From<S> for ReorderSignalsParams
where
    S: Iterator<Item = &'a SignalId>,
{
    fn from(value: S) -> Self {
        Self {
            signals: Some(SignalIds::from(value)),
        }
    }
}
