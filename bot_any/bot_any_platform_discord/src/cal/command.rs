use bot_any_cal::{CommandArgument, CommandArgumentValue, CommandPreflight, CommandSender};

use crate::sys::types::{
    ApplicationCommandOption, ApplicationCommandOptionKind, ApplicationCommandOptionValue,
    InteractionApplicationCommand,
};

pub fn parse_command(
    command: InteractionApplicationCommand,
) -> (CommandSender, Vec<CommandPreflight>) {
    let sender = if let Some(member) = command.rest.member {
        CommandSender::User(member.into())
    } else if let Some(user) = command.rest.user {
        CommandSender::User(user.into())
    } else {
        CommandSender::Unknown
    };
    let label = command.data.name;

    let mut preflights = Vec::new();

    preflights.push(CommandPreflight::Select(label));
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
                preflights.push(CommandPreflight::Select(name.clone()));
            }
            _ => {
                let mut arguments = Vec::new();
                for option in current_options {
                    match &option.value {
                        Some(ApplicationCommandOptionValue::String(s)) => {
                            arguments.push(CommandArgument {
                                name: option.name.clone(),
                                value: CommandArgumentValue::String(s.clone()),
                            });
                        }
                        Some(ApplicationCommandOptionValue::Int(i)) => {
                            arguments.push(CommandArgument {
                                name: option.name.clone(),
                                value: CommandArgumentValue::I64(i.clone()),
                            });
                        }
                        Some(ApplicationCommandOptionValue::Double(d)) => {
                            arguments.push(CommandArgument {
                                name: option.name.clone(),
                                value: CommandArgumentValue::F64(d.clone()),
                            });
                        }
                        None => {
                            continue;
                        }
                    };
                }
                preflights.push(CommandPreflight::Execute(arguments));
                break;
            }
        }
    }

    (sender, preflights)
}
