use crate::domain::params::UpdateProfileParam;
use nature_remo_api::service_provider::{self as normal, PyNatureRemoRequest};
use pyo3::prelude::*;

#[pyfunction]
pub(crate) fn get_user_request(
    py: Python<'_>,
    access_token: String,
) -> PyResult<Bound<'_, PyNatureRemoRequest>> {
    normal::get_user_request(access_token).into_pyobject(py)
}
#[pyfunction]
pub(crate) fn post_user_request(
    py: Python<'_>,
    access_token: String,
    value: UpdateProfileParam,
) -> PyResult<Bound<'_, PyNatureRemoRequest>> {
    normal::post_user_request(access_token, value.to_owned()).into_pyobject(py)
}
