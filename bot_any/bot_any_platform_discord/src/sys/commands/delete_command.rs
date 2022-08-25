use std::borrow::Cow;

use reqores::{ClientRequest, HttpMethod, StatusCode};

use crate::sys::types::Snowflake;

pub struct DeleteCommand<'a> {
    pub application_id: &'a str,
    pub token: &'a str,
    pub command_id: Snowflake,
    pub guild_id: Option<Snowflake>,
}

impl ClientRequest for DeleteCommand<'_> {
    type Response = ();

    fn headers(&self) -> Vec<(String, String)> {
        vec![("Authorization".to_string(), format!("Bot {}", self.token))]
    }

    fn url(&self) -> Cow<str> {
        if let Some(guild_id) = &self.guild_id {
            Cow::Owned(format!(
                "https://discord.com/api/v10/applications/{}/guilds/{}/commands/{}",
                self.application_id, guild_id.0, self.command_id.0
            ))
        } else {
            Cow::Owned(format!(
                "https://discord.com/api/v10/applications/{}/commands/{}",
                self.application_id, self.command_id.0
            ))
        }
    }

    fn method(&self) -> &HttpMethod {
        &HttpMethod::Delete
    }

    fn deserialize(
        &self,
        response: &impl reqores::ClientResponse,
    ) -> Result<Self::Response, String> {
        if response.status() == StatusCode::NoContent {
            Ok(())
        } else {
            Err("Failed to delete command".to_string())
        }
    }
}
