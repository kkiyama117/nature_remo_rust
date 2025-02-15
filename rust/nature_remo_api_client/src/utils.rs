use http::Uri;
use nature_remo_api::http::HTTPUri;

pub(crate) fn merge_string_to_url<T: Into<String>>(base_url: &HTTPUri, tail: T) -> HTTPUri {
    // Maybe this is infallible
    let builder = Uri::builder();
    let builder = if let Some(schema) = base_url.scheme() {
        builder.scheme(schema.as_str())
    } else {
        builder
    };
    let builder = if let Some(authority) = base_url.authority() {
        builder.authority(authority.as_str())
    } else {
        builder
    };
    let p_and_q = match base_url.query() {
        Some(q) => {
            format!("{}/{}?{}", base_url.path(), tail.into(), q)
        }
        None => {
            format!("{}/{}", base_url.path(), tail.into())
        }
    };
    HTTPUri(builder.path_and_query(p_and_q).build().unwrap())
}
