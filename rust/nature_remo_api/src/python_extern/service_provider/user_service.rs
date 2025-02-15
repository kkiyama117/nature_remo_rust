use super::*;
use crate::domain::params::UpdateProfileParam;

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
