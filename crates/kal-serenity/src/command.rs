use kal::{CommandArgument, CommandArgumentValue, CommandFragment};
use serenity::model::prelude::{
    command::CommandOptionType,
    interaction::application_command::{CommandData, CommandDataOption},
};

pub fn parse_command(data: &CommandData) -> Vec<CommandFragment> {
    let label = data.name.clone();

    let mut fragments = Vec::new();

    fragments.push(CommandFragment::Select(label));
    let mut current_options = data.options.as_slice();
    loop {
        match current_options {
            [CommandDataOption {
                kind: CommandOptionType::SubCommand,
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
                        Some(serenity::json::Value::String(s)) => {
                            CommandArgumentValue::String(s.clone())
                        }
                        Some(serenity::json::Value::Number(number)) => {
                            if let Some(i) = number.as_i64() {
                                CommandArgumentValue::I64(i)
                            } else if let Some(d) = number.as_f64() {
                                CommandArgumentValue::F64(d)
                            } else {
                                continue;
                            }
                        }
                        // some case never be handled
                        _ => continue,
                    };
                    arguments.push(CommandArgument::Named(option.name.clone(), value));
                }
                fragments.push(CommandFragment::Execute(arguments));
                break;
            }
        }
    }

    fragments
}
