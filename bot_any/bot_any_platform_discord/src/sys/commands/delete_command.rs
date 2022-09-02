use reqores::{headers, ClientRequest, HttpMethod, HttpStatusCode};

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
        vec![
            headers::content_type_json_utf8(),
            ("Authorization".to_string(), format!("Bot {}", self.token)),
        ]
    }

    fn url(&self) -> String {
        if let Some(guild_id) = &self.guild_id {
            format!(
                "https://discord.com/api/v10/applications/{}/guilds/{}/commands/{}",
                self.application_id, guild_id.0, self.command_id.0
            )
        } else {
            format!(
                "https://discord.com/api/v10/applications/{}/commands/{}",
                self.application_id, self.command_id.0
            )
        }
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn deserialize(
        &self,
        response: &dyn reqores::ClientResponse,
    ) -> Result<Self::Response, String> {
        if response.status() == HttpStatusCode::NoContent {
            Ok(())
        } else {
            Err("Failed to delete command".to_string())
        }
    }
}
