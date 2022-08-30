use crate::sys::types::{
    ApplicationCommand, Member, RawInteraction, RawInteractionData, RawInteractionKind, Snowflake,
    User,
};

pub enum Interaction {
    Ping,
    ApplicationCommand(InteractionApplicationCommand),
}
pub struct InteractionCommon {
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    pub member: Option<Member>,
    pub user: Option<User>,
}

pub struct InteractionApplicationCommand {
    pub data: ApplicationCommand,
    pub common: InteractionCommon,
}
impl Interaction {
    pub fn from(raw_interaction: RawInteraction) -> Option<Interaction> {
        let rest = InteractionCommon {
            guild_id: raw_interaction.guild_id,
            channel_id: raw_interaction.channel_id,
            member: raw_interaction.member,
            user: raw_interaction.user,
        };

        match (raw_interaction.kind, raw_interaction.data) {
            (RawInteractionKind::Ping, _) => Some(Interaction::Ping),
            (
                RawInteractionKind::ApplicationCommand,
                Some(RawInteractionData::ApplicationCommand(data)),
            ) => Some(Interaction::ApplicationCommand(
                InteractionApplicationCommand { data, common: rest },
            )),
            _ => None,
        }
    }
}
