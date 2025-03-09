use nature_remo_api::domain::errors::NatureRemoAPIError;
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error;

pub(crate) async fn responce_to_api_error(response: reqwest::Response) -> NatureRemoAPIError {
    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct Inner {
        pub code: usize,
        pub message: String,
    }
    let status_code = response.status().into();
    // Maybe This is infallible
    let inner = response.json::<Inner>().await.unwrap();
    NatureRemoAPIError {
        status_code,
        code: inner.code,
        message: inner.message,
    }
}

/// Wrapper of Error by Client
#[derive(Debug, Error)]
pub enum NatureRemoCliError {
    #[allow(clippy::upper_case_acronyms)]
    #[error("Nature Remo Error: {0}")]
    API(#[from] NatureRemoAPIError),
    #[allow(clippy::upper_case_acronyms)]
    #[error(transparent)]
    HTTP(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

impl Serialize for NatureRemoCliError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NatureRemoCliError::API(inner) => inner.serialize(serializer),
            NatureRemoCliError::HTTP(inner) => serializer.serialize_str(&inner.to_string()),
            NatureRemoCliError::Serde(inner) => serializer.serialize_str(&inner.to_string()),
        }
    }
}
