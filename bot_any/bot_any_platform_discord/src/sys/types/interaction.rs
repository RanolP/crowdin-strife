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
    kind: RawInteractionKind,
    data: Option<RawInteractionData>,
    guild_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    member: Option<Member>,
    user: Option<User>,
    token: String,
}

impl RawInteraction {
    pub fn process(self) -> Option<Interaction> {
        match (self.kind, self.data) {
            (RawInteractionKind::Ping, _) => Some(Interaction::Ping),
            (
                RawInteractionKind::ApplicationCommand,
                Some(RawInteractionData::ApplicationCommand(data)),
            ) => Some(Interaction::ApplicationCommand(data)),
            _ => None,
        }
    }
}

pub enum Interaction {
    Ping,
    ApplicationCommand(ApplicationCommand),
}
