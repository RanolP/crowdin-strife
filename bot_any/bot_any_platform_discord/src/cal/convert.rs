use bot_any_cal::{Command, CommandSender};

use crate::sys::types::{InteractionApplicationCommand};

impl From<InteractionApplicationCommand> for Command<()> {
    fn from(command: InteractionApplicationCommand) -> Self {
        Command {
            sender: if let Some(member) = command.rest.member {
                CommandSender::User(member.into())
            } else if let Some(user) = command.rest.user {
                CommandSender::User(user.into())
            } else {
                CommandSender::Unknown
            },
            label: command.data.name,
            arguments: (),
        }
    }
}
