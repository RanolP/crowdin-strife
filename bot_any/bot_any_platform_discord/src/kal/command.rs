use bot_any::types::{CommandSender, User as BotanyUser};
use kal::{CommandArgument, CommandArgumentValue, CommandFragment};

use crate::{
    sys::types::{
        ApplicationCommandOption, ApplicationCommandOptionKind, ApplicationCommandOptionValue,
    },
    InteractionApplicationCommand,
};

pub fn parse_command(
    command: InteractionApplicationCommand,
) -> (CommandSender, Vec<CommandFragment>) {
    let sender = command
        .common
        .member
        .map(BotanyUser::from)
        .or_else(|| command.common.user.map(BotanyUser::from))
        .map(CommandSender::User)
        .unwrap_or(CommandSender::Unknown);
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
                    let value = match &option.value {
                        Some(ApplicationCommandOptionValue::String(s)) => {
                            CommandArgumentValue::String(s.clone())
                        }
                        Some(ApplicationCommandOptionValue::Int(i)) => {
                            CommandArgumentValue::I64(i.clone())
                        }
                        Some(ApplicationCommandOptionValue::Double(d)) => {
                            CommandArgumentValue::F64(d.clone())
                        }
                        None => continue,
                    };
                    arguments.push(CommandArgument::Named(option.name.clone(), value));
                }
                fragments.push(CommandFragment::Execute(arguments));
                break;
            }
        }
    }

    (sender, fragments)
}
