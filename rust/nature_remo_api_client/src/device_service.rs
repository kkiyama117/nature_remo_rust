use super::*;
use nature_remo_api::domain::params::{
    HumidityOffsetParams, TemperatureOffsetParams, UpdateDeviceParameters,
};
use nature_remo_api::domain::response::{ApplianceResponse, DeviceResponse, DeviceResponses};
use nature_remo_api::service_provider::*;
use std::borrow::Cow;

impl NatureRemoAPIClient {
    #[async_backtrace::framed]
    pub async fn get_devices(&mut self) -> Result<DeviceResponses, NatureRemoCliError> {
        let req = get_devices_request(&self.access_token);
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn update_device<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
        value: &UpdateDeviceParameters,
    ) -> Result<DeviceResponse, NatureRemoCliError> {
        let req = update_device_request(
            &self.access_token,
            Into::<Cow<'_, str>>::into(id),
            value.clone(),
        );
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn get_device_appliances<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
    ) -> Result<ApplianceResponse, NatureRemoCliError> {
        let req = get_device_appliances_request(&self.access_token, Into::<Cow<'_, str>>::into(id));
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn delete_device<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
    ) -> Result<(), NatureRemoCliError> {
        let req = delete_device_request(&self.access_token, Into::<Cow<'_, str>>::into(id));
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn update_device_humidity_offset<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
        value: &HumidityOffsetParams,
    ) -> Result<DeviceResponse, NatureRemoCliError> {
        let req = update_device_humidity_offset(
            &self.access_token,
            Into::<Cow<'_, str>>::into(id),
            value.clone(),
        );
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn update_device_temperature_offset<'a>(
        &mut self,
        id: impl Into<Cow<'a, str>>,
        value: &TemperatureOffsetParams,
    ) -> Result<DeviceResponse, NatureRemoCliError> {
        let req = update_device_temperature_resuest(
            &self.access_token,
            Into::<Cow<'_, str>>::into(id),
            value.clone(),
        );
        self.send_request(req).await
    }
}
