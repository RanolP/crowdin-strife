use bot_any_cal::{Command, CommandHandler, CommandSender};

use crate::sys::types::{
    ApplicationCommandOption, ApplicationCommandOptionChoiceValue, ApplicationCommandOptionKind,
    InteractionApplicationCommand,
};

fn parse_command<T: Command>(
    mut handler: impl CommandHandler<T>,
    command: InteractionApplicationCommand,
) -> Option<(T, CommandSender)> {
    let sender = if let Some(member) = command.rest.member {
        CommandSender::User(member.into())
    } else if let Some(user) = command.rest.user {
        CommandSender::User(user.into())
    } else {
        CommandSender::Unknown
    };
    let label = command.data.name;

    handler = handler.visit_subcommand(&label)?;
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
                handler = handler.visit_subcommand(&name)?;
            }
            _ => {
                for choice in current_options.iter().flat_map(|opt| opt.choices.iter()) {
                    handler = match &choice.value {
                        ApplicationCommandOptionChoiceValue::String(s) => {
                            handler.visit_argument_str(choice.name.clone(), s.clone())
                        }
                        ApplicationCommandOptionChoiceValue::Int(i) => {
                            handler.visit_argument_i64(choice.name.clone(), i.clone())
                        }
                        ApplicationCommandOptionChoiceValue::Double(d) => {
                            handler.visit_argument_f64(choice.name.clone(), d.clone())
                        }
                    };
                }
                break Some((handler.parse(), sender));
            }
        }
    }
}
