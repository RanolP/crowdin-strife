use std::borrow::Cow;

use serde::de::DeserializeOwned;

use crate::{ClientResponse, HttpMethod};

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

    fn deserialize(&self, response: &impl ClientResponse) -> Result<Self::Response, String> {
        serde_json::from_slice(response.body()).map_err(|e| e.to_string())
    }
}
