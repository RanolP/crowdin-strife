use bot_any_cal::{
    Command, CommandArgument, CommandArgumentValue, CommandPreflight, CommandSender,
};

use crate::sys::types::{
    ApplicationCommandOption, ApplicationCommandOptionChoiceValue, ApplicationCommandOptionKind,
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
                for choice in current_options.iter().flat_map(|opt| opt.choices.iter()) {
                    match &choice.value {
                        ApplicationCommandOptionChoiceValue::String(s) => {
                            arguments.push(CommandArgument {
                                name: choice.name.clone(),
                                value: CommandArgumentValue::String(s.clone()),
                            })
                        }
                        ApplicationCommandOptionChoiceValue::Int(i) => {
                            arguments.push(CommandArgument {
                                name: choice.name.clone(),
                                value: CommandArgumentValue::I64(i.clone()),
                            })
                        }
                        ApplicationCommandOptionChoiceValue::Double(d) => {
                            arguments.push(CommandArgument {
                                name: choice.name.clone(),
                                value: CommandArgumentValue::F64(d.clone()),
                            })
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
