use crate::domain::inner::ImageId;
#[cfg(feature = "pyo3")]
use pyo3::{self, prelude::pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct CreateSignalParameters {
    /// Basename of the image file included in the app. Ex: 'ico_io'.
    image: Option<ImageId>,
    // TODO: Implement custom class
    /// JSON serialized object describing infrared signals.
    /// Includes 'data', 'freq' and 'format' keys.
    message: Option<String>,
    /// Signal name.
    name: Option<String>,
}
