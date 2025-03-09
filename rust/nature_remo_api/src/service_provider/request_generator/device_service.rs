use super::*;
use crate::domain::params::{
    HumidityOffsetParams, TemperatureOffsetParams, UpdateDeviceParameters,
};
use crate::http::HTTPMethod;
use std::borrow::Cow;

/// Generate the metadata of "GET: 1/devices"
pub fn get_devices_request<'a>(access_token: impl Into<Cow<'a, str>>) -> NatureRemoRequest<()> {
    NatureRemoRequest::new("devices", access_token, HTTPMethod::GET, None)
}

/// Generate the metadata of "POST: 1/devices/{:id}"
///
/// Now We can only update name of device.
pub fn update_device_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'a, str>>,
    value: UpdateDeviceParameters,
) -> NatureRemoRequest<UpdateDeviceParameters> {
    NatureRemoRequest::new(
        format!("devices/{}", id.into()),
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}

/// Generate the metadata of "GET: 1/devices/{id}/appliances"
pub fn get_device_appliances_request<'a, 'b>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'b, str>>,
) -> NatureRemoRequest<()> {
    NatureRemoRequest::new(
        format!("devices/{}/appliances", id.into()),
        access_token,
        HTTPMethod::GET,
        None,
    )
}

/// Generate the metadata of "POST: 1/devices/{id}/delete"
pub fn delete_device_request<'a, 'b>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'b, str>>,
) -> NatureRemoRequest<()> {
    NatureRemoRequest::new(
        format!("devices/{}/delete", id.into()),
        access_token,
        HTTPMethod::POST,
        None,
    )
}

/// Generate the metadata of "POST: 1/devices/{id}/humidity_offset"
pub fn update_device_humidity_offset<'a, 'b>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'b, str>>,
    value: HumidityOffsetParams,
) -> NatureRemoRequest<HumidityOffsetParams> {
    NatureRemoRequest::new(
        format!("devices/{}/humidity_offset", id.into()),
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}
/// Generate the metadata of "POST: 1/devices/{id}/temperature_offset"
pub fn update_device_temperature_resuest<'a, 'b>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'b, str>>,
    value: TemperatureOffsetParams,
) -> NatureRemoRequest<TemperatureOffsetParams> {
    NatureRemoRequest::new(
        format!("devices/{}/temperature_offset", id.into()),
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}
