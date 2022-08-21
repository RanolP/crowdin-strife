use std::borrow::Cow;

use reqores::{ClientRequest, HttpMethod};

pub struct RefreshToken;

impl ClientRequest for RefreshToken {
    type Response = String;

    fn url(&self) -> Cow<str> {
        Cow::Borrowed("https://accounts.crowdin.com/auth/token?refresh=true")
    }

    fn method(&self) -> &HttpMethod {
        &HttpMethod::Get
    }

    fn header_processor(&self) -> Option<fn(&str, &str) -> Option<Self::Response>> {
        fn process(key: &str, value: &str) -> Option<String> {
            if key == "set-cookie" && value.starts_with("CSRF-TOKEN=") {
                let key = value.find("CSRF-TOKEN=")?;
                let semi = value[key..].find(";")?;
                Some(value[key + "CSRF-TOKEN=".len()..semi].to_string())
            } else {
                None
            }
        }
        Some(process)
    }
}
