mod appliance_service;
mod device_service;
mod errors;
mod user_service;
mod utils;

use crate::errors::NatureRemoCliError;
use crate::{errors::responce_to_api_error, utils::merge_string_to_url};
use http::Uri;

use nature_remo_api::http::HTTPUri;
use nature_remo_api::{
    // domain::errors::NatureRemoAPIError, http::RateLimit, service_provider::NatureRemoRequest,
    http::RateLimit,
    service_provider::NatureRemoRequest,
};
use reqwest::Client;
use serde::{self, Serialize, de::DeserializeOwned};
use std::fmt::Debug;

const BASE_URL: &str = "https://api.nature.global/1";

/// Sample Implementation for Demo.
///
/// This client uses [reqwest::Client] for http connection.
/// Each Implementation for each domain is in `xx_service.rs` like [user_service]
#[derive(Debug, Clone)]
pub struct NatureRemoAPIClient {
    /// HTTP Client
    http_client: Client,
    // Nature Remo API
    access_token: String,
    pub base_url: HTTPUri,
    // Rate Limit
    pub last_rate_limit: Option<RateLimit>,
}

impl NatureRemoAPIClient {
    /// NewClient creates new client with access token of Nature Remo API.
    /// You can get access token from https://home.nature.global/.
    pub fn new(access_token: &'_ str) -> Self {
        let http_client = Client::new();
        let base_uri = Uri::from_static(BASE_URL);
        Self {
            http_client,
            access_token: access_token.to_string(),
            base_url: HTTPUri::from(base_uri),
            last_rate_limit: Default::default(),
        }
    }

    #[async_backtrace::framed]
    pub async fn send_request<T: Serialize, U: DeserializeOwned>(
        &mut self,
        request: NatureRemoRequest<T>,
    ) -> Result<U, NatureRemoCliError> {
        let mut request_builder = self
            .http_client
            .request(
                request.http_method.into(),
                merge_string_to_url(&self.base_url, &request.url_tail).to_string(),
            )
            .headers(request.headers.clone());
        request_builder = if let Some(inner) = request.data {
            request_builder.form(&inner)
            // request_builder.json(&inner)
        } else {
            request_builder
        };
        let response = request_builder
            .send()
            .await
            .map_err(NatureRemoCliError::from)?;

        // Parse error returned by Nature Remo API
        if !response.status().is_success() {
            // Update rate limit information from response headers
            let rate_limit = RateLimit::try_from(response.headers()).ok();
            self.last_rate_limit = rate_limit;
            let api_error = responce_to_api_error(response).await;
            // let api_error = NatureRemoAPIError::from_response(response).await;
            Err(NatureRemoCliError::API(api_error))
        } else {
            // Update rate limit information from response headers
            let rate_limit = RateLimit::try_from(response.headers()).ok();
            self.last_rate_limit = rate_limit;
            // FOR LOGGING
            #[cfg(feature = "debug")]
            {
                use log::debug;
                let body = response.bytes().await.map_err(NatureRemoCliError::from)?;
                match serde_json::from_slice::<U>(&body) {
                    Ok(inner) => {
                        let json_value = serde_json::from_slice::<serde_json::Value>(&body)
                            .map_err(NatureRemoCliError::from)?;
                        debug!("{}", json_value);
                        Ok(inner)
                    }
                    Err(e) => {
                        let json_value = serde_json::from_slice::<serde_json::Value>(&body)
                            .map_err(NatureRemoCliError::from)?;
                        debug!("{}", json_value);
                        Err(NatureRemoCliError::from(e))
                    }
                }
            }
            #[cfg(not(feature = "debug"))]
            response.json::<U>().await.map_err(NatureRemoCliError::from)
        }
    }
}
