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
}

pub struct InteractionRest {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<Member>,
    pub user: Option<User>,
}

impl RawInteraction {
    pub fn transform(self) -> Option<Interaction> {
        let rest = InteractionRest {
            guild_id: self.guild_id,
            channel_id: self.channel_id,
            member: self.member,
            user: self.user,
        };

        match (self.kind, self.data) {
            (RawInteractionKind::Ping, _) => Some(Interaction::Ping),
            (
                RawInteractionKind::ApplicationCommand,
                Some(RawInteractionData::ApplicationCommand(data)),
            ) => Some(Interaction::ApplicationCommand(
                InteractionApplicationCommand { data, rest },
            )),
            _ => None,
        }
    }
}

pub enum Interaction {
    Ping,
    ApplicationCommand(InteractionApplicationCommand),
}

pub struct InteractionApplicationCommand {
    pub data: ApplicationCommand,
    pub rest: InteractionRest,
}
