use serde::Deserialize;
pub use url::Url;

pub use client_request::*;
pub use client_response::*;
pub use http_method::*;
pub use server_request::*;
pub use server_response::*;
pub use status_code::*;

mod client_request;
mod client_response;
mod http_method;
mod server_request;
mod server_response;
mod status_code;

#[derive(Deserialize)]
pub struct IgnoreObject {}
