use crate::http::HTTPUri;
#[cfg(feature = "pyo3")]
use pyo3::prelude::{IntoPyObject, pyclass};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PrivacyPolicy {
    pub body: String,
    pub links: HashMap<String, LinkAndText>,
    pub title: String,
}

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct LinkAndText {
    pub link: HTTPUri,
    pub text: String,
}
