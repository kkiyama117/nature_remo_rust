use crate::domain::inner::ApplianceModel;
use crate::domain::params::ApplianceParams;
#[cfg(feature = "pyo3")]
use pyo3::{
    self,
    prelude::{FromPyObject, IntoPyObject, pyclass},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct ApplianceModelAndParam {
    /// Basename of the image file included in the app. Ex: 'ico_ac_1'.
    pub model: ApplianceModel,
    pub params: ApplianceParams,
}

impl Display for ApplianceModelAndParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApplianceModelAndParam(id:{},name:{})",
            self.model.name, self.model.id
        )
    }
}

#[cfg_attr(feature = "pyo3", derive(FromPyObject, IntoPyObject))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
/// Wrapper of [Vec<ApplianceModelAndParam>]. In python, same as `list[ApplianceModelAndParam]`
pub struct ApplianceModelAndParams(Vec<ApplianceModelAndParam>);

nature_response_list_domain!(ApplianceModelAndParam, ApplianceModelAndParams);
