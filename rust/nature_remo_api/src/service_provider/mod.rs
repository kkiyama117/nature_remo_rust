mod request_generator;

use crate::http::HTTPMethod;
use http::{
    HeaderMap,
    header::{ACCEPT, AUTHORIZATION, HeaderValue, USER_AGENT},
};
pub use request_generator::*;
#[cfg(feature = "serde")]
use serde::{self, Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;

// TODO: Use &T
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct NatureRemoRequest<T> {
    // #[cfg_attr(feature = "serde", serde(with = "http_serde::uri"))]
    // pub url: Uri,
    pub url_tail: String,
    pub http_method: HTTPMethod,
    #[cfg_attr(feature = "serde", serde(with = "http_serde::header_map"))]
    pub headers: HeaderMap,
    pub data: Option<T>,
}

impl<T> NatureRemoRequest<T> {
    pub fn new<'a, 'b, U: Into<Cow<'a, str>>, V: Into<Cow<'b, str>>>(
        url_tail: U,
        access_token: V,
        http_method: HTTPMethod,
        data: Option<T>,
    ) -> Self {
        let url_tail = url_tail.into();
        Self {
            url_tail: url_tail.into(),
            http_method,
            headers: default_header_map(access_token),
            data,
        }
    }
}

fn default_header_map<'a>(access_token: impl Into<Cow<'a, str>>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token.into()))
            .expect("Failed to create header value"),
    );
    headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str("nature-remo/1 (nature_remo_rust)").unwrap(),
    );
    headers
}
