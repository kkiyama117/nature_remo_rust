#![allow(dead_code)]

use nature_remo_api::domain::params::UpdateDeviceParameters;
use nature_remo_api::service_provider::{self as normal, PyNatureRemoRequest};
use pyo3::prelude::*;

#[pyfunction]
pub(crate) fn get_devices_request(
    py: Python<'_>,
    access_token: String,
) -> PyResult<Bound<'_, PyNatureRemoRequest>> {
    normal::get_devices_request(access_token).into_pyobject(py)
}

#[pyfunction]
pub(crate) fn post_device_request(
    py: Python<'_>,
    access_token: String,
    id: String,
    value: UpdateDeviceParameters,
) -> PyResult<Bound<'_, PyNatureRemoRequest>> {
    normal::update_device_request(access_token, id, value.to_owned()).into_pyobject(py)
}

#[pyfunction]
pub(crate) fn get_device_appliances_request(
    py: Python<'_>,
    access_token: String,
    id: String,
) -> PyResult<Bound<'_, PyNatureRemoRequest>> {
    normal::get_device_appliances_request(access_token, id).into_pyobject(py)
}
