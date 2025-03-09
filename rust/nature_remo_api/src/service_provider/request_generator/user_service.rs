use super::*;
use crate::domain::params::UpdateProfileParam;
use crate::http::HTTPMethod;
use std::borrow::Cow;

/// Generate the metadata of "GET: 1/users/me"
pub fn get_user_request<'a>(access_token: impl Into<Cow<'a, str>>) -> NatureRemoRequest<()> {
    NatureRemoRequest::new("users/me", access_token, HTTPMethod::GET, None)
}

/// Generate the metadata of "POST: 1/users/me"
pub fn post_user_request<'a>(
    access_token: impl Into<Cow<'a, str>>,
    value: UpdateProfileParam,
) -> NatureRemoRequest<UpdateProfileParam> {
    NatureRemoRequest::new("users/me", access_token, HTTPMethod::POST, Some(value))
}
