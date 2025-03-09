use super::*;
use nature_remo_api::domain::params::UpdateProfileParam;
use nature_remo_api::domain::response::UserResponse;
use nature_remo_api::service_provider::{get_user_request, post_user_request};

impl NatureRemoAPIClient {
    #[async_backtrace::framed]
    pub async fn get_me(&mut self) -> Result<UserResponse, NatureRemoCliError> {
        let req = get_user_request(&self.access_token);
        self.send_request(req).await
    }

    #[async_backtrace::framed]
    pub async fn update_user(
        &mut self,
        value: &UpdateProfileParam,
    ) -> Result<UserResponse, NatureRemoCliError> {
        let req = post_user_request(&self.access_token, value.clone());
        self.send_request(req).await
    }
}
