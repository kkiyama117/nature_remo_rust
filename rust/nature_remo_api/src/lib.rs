/// *api* is a library that abstracts a HTTP API
/// and separates the client from the API definition.
/// This allows you to change the underlying HTTP
/// client easily.
use std::collections::BTreeMap;


/// Type for the request/response headers.
pub type Headers = BTreeMap<String, Vec<String>>;
/// Type for the URL query.
pub type Query<'s> = Vec<(String, String)>;


#[cfg(feature = "python")]
mod python_extern;
pub mod domain;
