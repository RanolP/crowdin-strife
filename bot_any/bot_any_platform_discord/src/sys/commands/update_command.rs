use reqores::{ClientRequest, HttpMethod, headers};

use crate::sys::types::{ApplicationCommand, Snowflake};

pub struct UpdateCommand<'a> {
    pub application_id: &'a str,
    pub token: &'a str,
    pub guild_id: Option<Snowflake>,
    pub command: ApplicationCommand,
}

impl ClientRequest for UpdateCommand<'_> {
    type Response = ApplicationCommand;

    fn headers(&self) -> Vec<(String, String)> {
        vec![ headers::content_type_json_utf8(), ("Authorization".to_string(), format!("Bot {}", self.token))]
    }

    fn url(&self) -> String {
        if let Some(guild_id) = &self.guild_id {
            format!(
                "https://discord.com/api/v10/applications/{}/guilds/{}/commands",
                self.application_id, guild_id.0
            )
        } else {
            format!(
                "https://discord.com/api/v10/applications/{}/commands",
                self.application_id
            )
        }
    }

    fn body(&self) -> Option<String> {
        Some(serde_json::to_string(&self.command).unwrap())
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }
}
