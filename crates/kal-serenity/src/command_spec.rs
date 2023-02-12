use kal::{CommandOption, CommandOptionValueKind, CommandSpec};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("You cannot have both sub-command and options due to limitation of Discord slash command system")]
pub struct CommandSpecError;

impl TryFrom<CommandSpec> for ApplicationCommand {
    type Error = CommandSpecError;

    fn try_from(spec: CommandSpec) -> Result<Self, Self::Error> {
        let mut options = match (spec.subcommands.is_empty(), spec.options.is_empty()) {
            (false, false) => return Err(CommandSpecError),
            (false, true) => spec
                .subcommands
                .into_iter()
                .map(ApplicationCommandOption::try_from)
                .collect::<Result<_, _>>()?,
            (true, false) => spec
                .options
                .into_iter()
                .map(ApplicationCommandOption::from)
                .collect(),
            (true, true) => Vec::new(),
        };
        options.sort_by_key(|opt| match &opt.required {
            Some(true) => 0,
            None | Some(false) => 1,
        });
        Ok(ApplicationCommand {
            id: None,
            kind: Some(ApplicationCommandKind::ChatInput),
            application_id: None,
            guild_id: None,
            name: spec.name.to_string(),
            options,
            description: Some(spec.description.to_string()),
        })
    }
}

impl TryFrom<CommandSpec> for ApplicationCommandOption {
    type Error = CommandSpecError;

    fn try_from(spec: CommandSpec) -> Result<Self, Self::Error> {
        let mut options = match (spec.subcommands.is_empty(), spec.options.is_empty()) {
            (false, false) => return Err(CommandSpecError),
            (false, true) => spec
                .subcommands
                .into_iter()
                .map(ApplicationCommandOption::try_from)
                .collect::<Result<_, _>>()?,
            (true, false) => spec
                .options
                .into_iter()
                .map(ApplicationCommandOption::from)
                .collect(),
            (true, true) => Vec::new(),
        };
        options.sort_by_key(|opt| match &opt.required {
            Some(true) => 0,
            None | Some(false) => 1,
        });
        Ok(ApplicationCommandOption {
            kind: ApplicationCommandOptionKind::SubCommand,
            name: spec.name.to_string(),
            value: None,
            description: Some(spec.description.to_string()),
            required: None,
            choices: Vec::new(),
            options,
        })
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
            value: None,
            description: Some(option.description.to_string()),
            required: Some(!option.value.is_optional()),
            choices: vec![],
            options: vec![],
        }
    }
}
