use reqores::{ClientRequest, HttpMethod};

pub struct RefreshToken;

impl ClientRequest for RefreshToken {
    type Response = String;

    fn url(&self) -> String {
        "https://accounts.crowdin.com/auth/token?refresh=true".to_string()
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn deserialize(
        &self,
        response: &impl reqores::ClientResponse,
    ) -> Result<Self::Response, String> {
        if let Some(value) = response.header("set-cookie") {
            let key = value
                .find("CSRF-TOKEN=")
                .ok_or("Failed to find csrf token cookie".to_string())?;
            let semi = value[key..].find(";").unwrap_or(value.len());
            Ok(value[key + "CSRF-TOKEN=".len()..semi].to_string())
        } else {
            Err("Failed to receive set-cookie header".to_string())
        }
    }
}
