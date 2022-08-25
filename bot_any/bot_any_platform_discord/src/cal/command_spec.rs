use bot_any_cal::{CommandOption, CommandOptionValueKind, CommandSpec};

use crate::sys::types::{
    ApplicationCommand, ApplicationCommandKind, ApplicationCommandOption,
    ApplicationCommandOptionKind,
};

impl From<CommandSpec> for ApplicationCommand {
    fn from(spec: CommandSpec) -> Self {
        ApplicationCommand {
            id: None,
            kind: Some(ApplicationCommandKind::ChatInput),
            application_id: None,
            guild_id: None,
            name: spec.name.to_string(),
            options: spec
                .subcommands
                .into_iter()
                .map(|s| s.into())
                .chain(spec.options.into_iter().map(|o| o.into()))
                .collect(),
            description: spec.description.map(|s| s.to_string()),
        }
    }
}

impl From<CommandSpec> for ApplicationCommandOption {
    fn from(spec: CommandSpec) -> Self {
        ApplicationCommandOption {
            kind: ApplicationCommandOptionKind::SubCommand,
            name: spec.name.to_string(),
            description: spec.description.map(|s| s.to_string()),
            required: None,
            choices: Vec::new(),
            options: spec
                .subcommands
                .into_iter()
                .map(|s| s.into())
                .chain(spec.options.into_iter().map(|o| o.into()))
                .collect(),
        }
    }
}
impl From<CommandOption> for ApplicationCommandOption {
    fn from(option: CommandOption) -> Self {
        ApplicationCommandOption {
            kind: {
                match option.value.as_primitive() {
                    CommandOptionValueKind::String => ApplicationCommandOptionKind::String,
                    CommandOptionValueKind::Integer => ApplicationCommandOptionKind::Integer,
                    CommandOptionValueKind::Double => ApplicationCommandOptionKind::Number,
                    _ => unreachable!(),
                }
            },
            name: option.name.to_string(),
            description: option.description.map(|s| s.to_string()),
            required: Some(option.value.is_optional()),
            choices: vec![],
            options: vec![],
        }
    }
}
