use std::borrow::Cow;

use reqores::{ClientRequest, HttpMethod};

use crate::sys::types::{ApplicationCommand, Snowflake};

pub struct ListCommands<'a> {
    pub application_id: &'a str,
    pub token: &'a str,
    pub guild_id: Option<Snowflake>,
}

impl ClientRequest for ListCommands<'_> {
    type Response = Vec<ApplicationCommand>;

    fn headers(&self) -> Vec<(String, String)> {
        vec![("Authorization".to_string(), format!("Bot {}", self.token))]
    }

    fn url(&self) -> Cow<str> {
        if let Some(guild_id) = &self.guild_id {
            Cow::Owned(format!(
                "https://discord.com/api/v10/applications/{}/guilds/{}/commands",
                self.application_id, guild_id.0
            ))
        } else {
            Cow::Owned(format!(
                "https://discord.com/api/v10/applications/{}/commands",
                self.application_id
            ))
        }
    }

    fn method(&self) -> &HttpMethod {
        &HttpMethod::Get
    }
}
