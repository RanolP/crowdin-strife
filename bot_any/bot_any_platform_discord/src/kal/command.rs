use bot_any::types::CommandSender;
use kal::{CommandArgumentValue, CommandFragment};

use crate::sys::types::{
    ApplicationCommandOption, ApplicationCommandOptionKind, ApplicationCommandOptionValue,
    InteractionApplicationCommand,
};

pub fn parse_command(
    command: InteractionApplicationCommand,
) -> (CommandSender, Vec<CommandFragment>) {
    let sender = if let Some(member) = command.rest.member {
        CommandSender::User(member.into())
    } else if let Some(user) = command.rest.user {
        CommandSender::User(user.into())
    } else {
        CommandSender::Unknown
    };
    let label = command.data.name;

    let mut fragments = Vec::new();

    fragments.push(CommandFragment::Select(label));
    let mut current_options = command.data.options.as_slice();
    loop {
        match current_options {
            [ApplicationCommandOption {
                kind: ApplicationCommandOptionKind::SubCommand,
                options,
                name,
                ..
            }] => {
                current_options = options.as_slice();
                fragments.push(CommandFragment::Select(name.clone()));
            }
            _ => {
                let mut arguments = Vec::new();
                for option in current_options {
                    match &option.value {
                        Some(ApplicationCommandOptionValue::String(s)) => {
                            arguments.push((
                                option.name.clone(),
                                CommandArgumentValue::String(s.clone()),
                            ));
                        }
                        Some(ApplicationCommandOptionValue::Int(i)) => {
                            arguments
                                .push((option.name.clone(), CommandArgumentValue::I64(i.clone())));
                        }
                        Some(ApplicationCommandOptionValue::Double(d)) => {
                            arguments
                                .push((option.name.clone(), CommandArgumentValue::F64(d.clone())));
                        }
                        None => {
                            continue;
                        }
                    };
                }
                fragments.push(CommandFragment::Execute(arguments));
                break;
            }
        }
    }

    (sender, fragments)
}
