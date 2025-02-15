use super::*;
use nature_remo_api::domain::params::{
    AirConParams, ApplianceParams, CreateApplianceRequest, CreateSignalParameters,
    DetectApplianceRequest, ReorderAppliancesParams,
};
use nature_remo_api::domain::response::{
    AirConSettingResponse, ApplianceResponse, ApplianceResponses, Signals,
};
use nature_remo_api::service_provider::{
    create_appliance_request, create_appliance_signals_request, delete_appliance_request,
    detect_appliance_request, get_appliance_signals_request, get_appliances_request,
    update_appliance_aircon_settings_request, update_appliance_orders_request,
    update_appliance_request,
};
use std::borrow::Cow;

impl NatureRemoAPIClient {
    #[async_backtrace::framed]
    pub async fn update_appliance_orders(
        &mut self,
        value: ReorderAppliancesParams,
    ) -> Result<(), NatureRemoCliError> {
        let req = update_appliance_orders_request(&self.access_token, value);
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn get_appliances(&mut self) -> Result<ApplianceResponses, NatureRemoCliError> {
        let req = get_appliances_request(&self.access_token);
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn create_appliance<'a>(
        &mut self,
        value: &CreateApplianceRequest,
    ) -> Result<ApplianceResponse, NatureRemoCliError> {
        let req = create_appliance_request(&self.access_token, value.clone());
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn update_appliance<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
        value: &ApplianceParams,
    ) -> Result<ApplianceResponse, NatureRemoCliError> {
        let req = update_appliance_request(
            &self.access_token,
            Into::<Cow<'_, str>>::into(id),
            value.clone(),
        );
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn delete_appliance<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
    ) -> Result<ApplianceResponse, NatureRemoCliError> {
        let req = delete_appliance_request(&self.access_token, Into::<Cow<'_, str>>::into(id));
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn update_appliance_aircon_settings<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
        value: &AirConParams,
    ) -> Result<AirConSettingResponse, NatureRemoCliError> {
        let req = update_appliance_aircon_settings_request(
            &self.access_token,
            Into::<Cow<'_, str>>::into(id),
            value.clone(),
        );
        self.send_request(req).await
    }

    // TODO: LIGHT~SESAME_BOT

    /// Fetch the list of signals. Requires basic.read OAuth2 scopes.
    #[async_backtrace::framed]
    pub async fn get_appliance_signals<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
    ) -> Result<Signals, NatureRemoCliError> {
        let req = get_appliance_signals_request(&self.access_token, Into::<Cow<'_, str>>::into(id));
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn create_appliance_signals<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
        value: &CreateSignalParameters,
    ) -> Result<Signals, NatureRemoCliError> {
        let req = create_appliance_signals_request(
            &self.access_token,
            Into::<Cow<'_, str>>::into(id),
            value.clone(),
        );
        self.send_request(req).await
    }
    // TODO: TV

    #[async_backtrace::framed]
    pub async fn detect_appliance<'a>(
        &mut self,
        value: &DetectApplianceRequest,
    ) -> Result<Signals, NatureRemoCliError> {
        let req = detect_appliance_request(&self.access_token, value.clone());
        self.send_request(req).await
    }
}
