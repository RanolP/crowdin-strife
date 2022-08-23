use std::borrow::Cow;

use reqores::{ClientRequest, HttpMethod};

use crate::sys::types::Snowflake;

pub struct DeleteCommand<'a> {
    pub application_id: &'a str,
    pub token: &'a str,
    pub command_id: Snowflake,
}

impl ClientRequest for DeleteCommand<'_> {
    type Response = ();

    fn headers(&self) -> Vec<(String, String)> {
        vec![("Authorization".to_string(), format!("Bot {}", self.token))]
    }

    fn url(&self) -> Cow<str> {
        Cow::Owned(format!(
            "https://discord.com/api/v10/applications/{}/commands/{}",
            self.application_id, self.command_id.0
        ))
    }

    fn method(&self) -> &HttpMethod {
        &HttpMethod::Delete
    }

    fn deserialize(
        &self,
        _: &impl reqores::ClientResponse,
    ) -> Result<Self::Response, String> {
        Ok(())
    }
}
