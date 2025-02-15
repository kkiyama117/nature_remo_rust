use super::*;
use crate::domain::params::{
    AirConParams, ApplianceParams, CreateApplianceRequest, CreateSignalParameters,
    DetectApplianceRequest, ReorderAppliancesParams, ReorderSignalsParams,
};
use crate::http::HTTPMethod;
use std::borrow::Cow;

/// Generate the metadata of "POST: 1/appliance_order"
pub fn update_appliance_orders_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    value: ReorderAppliancesParams,
) -> NatureRemoRequest<ReorderAppliancesParams> {
    NatureRemoRequest::new(
        "appliance_orders",
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}

/// Generate the metadata of "GET: 1/appliances"
pub fn get_appliances_request<'a>(access_token: impl Into<Cow<'a, str>>) -> NatureRemoRequest<()> {
    NatureRemoRequest::new("appliances", access_token, HTTPMethod::GET, None)
}

/// Generate the metadata of "POST: 1/appliances"
pub fn create_appliance_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    value: CreateApplianceRequest,
) -> NatureRemoRequest<CreateApplianceRequest> {
    NatureRemoRequest::new("appliances", access_token, HTTPMethod::GET, Some(value))
}

/// Generate the metadata of "POST: /1/appliances/{id}"
pub fn update_appliance_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'a, str>>,
    value: ApplianceParams,
) -> NatureRemoRequest<ApplianceParams> {
    NatureRemoRequest::new(
        format!("appliances/{}", id.into()),
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}

/// Generate the metadata of "POST: /1/appliances/{id}/delete"
pub fn delete_appliance_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'a, str>>,
) -> NatureRemoRequest<()> {
    NatureRemoRequest::new(
        format!("appliances/{}", id.into()),
        access_token,
        HTTPMethod::POST,
        None,
    )
}
pub fn update_appliance_aircon_settings_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'a, str>>,
    value: AirConParams,
) -> NatureRemoRequest<AirConParams> {
    NatureRemoRequest::new(
        format!("appliances/{}/aircon_settings", id.into()),
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}

pub fn update_appliance_signal_orders_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    value: ReorderSignalsParams,
) -> NatureRemoRequest<ReorderSignalsParams> {
    NatureRemoRequest::new(
        "appliance_orders",
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}

pub fn get_appliance_signals_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'a, str>>,
) -> NatureRemoRequest<()> {
    NatureRemoRequest::new(
        format!("appliances/{}/signals", id.into()),
        access_token,
        HTTPMethod::GET,
        None,
    )
}
pub fn create_appliance_signals_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    id: impl Into<Cow<'a, str>>,
    value: CreateSignalParameters,
) -> NatureRemoRequest<CreateSignalParameters> {
    NatureRemoRequest::new(
        format!("appliances/{}/signals", id.into()),
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}

/// Find the air conditioner best matching the provided infrared signal. Requires detectappliance OAuth2 scopes.
pub fn detect_appliance_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    value: DetectApplianceRequest,
) -> NatureRemoRequest<DetectApplianceRequest> {
    NatureRemoRequest::new(
        "detectappliance",
        access_token,
        HTTPMethod::POST,
        Some(value),
    )
}
