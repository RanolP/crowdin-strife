use serde::Deserialize;
use serde_repr::Deserialize_repr;

use super::{ApplicationCommand, Member, Snowflake, User};

#[derive(Deserialize_repr)]
#[repr(u32)]
pub enum RawInteractionKind {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutoComplete = 4,
    ModalSubmit = 5,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawInteractionData {
    ApplicationCommand(ApplicationCommand),
}

#[derive(Deserialize)]
pub struct RawInteraction {
    #[serde(rename = "type")]
    pub kind: RawInteractionKind,
    pub data: Option<RawInteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<Member>,
    pub user: Option<User>,
}
