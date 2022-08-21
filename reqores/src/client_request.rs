use std::borrow::Cow;

use serde::de::DeserializeOwned;

use crate::HttpMethod;

pub trait ClientRequest {
    type Response: DeserializeOwned;

    fn headers(&self) -> Vec<(String, String)> {
        Default::default()
    }

    fn url(&self) -> Cow<str>;

    fn body(&self) -> Option<String> {
        None
    }

    fn method(&self) -> &HttpMethod;

    fn header_processor(&self) -> Option<fn(&str, &str) -> Option<Self::Response>> {
        None
    }
}
