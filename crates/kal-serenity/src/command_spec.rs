use kal::{CommandOption, CommandOptionValueKind, CommandSpec};
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    model::prelude::command::{CommandOptionType, CommandType},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("You cannot have both sub-command and options due to limitation of Discord slash command system")]
pub struct CommandSpecError;

pub fn try_into_serenity_command(
    spec: CommandSpec,
) -> Result<CreateApplicationCommand, CommandSpecError> {
    let options = match (spec.subcommands.is_empty(), spec.options.is_empty()) {
        (false, false) => return Err(CommandSpecError),
        (false, true) => spec
            .subcommands
            .into_iter()
            .map(try_into_create_application_command_option)
            .collect::<Result<_, _>>()?,
        (true, false) => spec
            .options
            .into_iter()
            .map(into_create_application_command_option)
            .collect(),
        (true, true) => Vec::new(),
    };
    let mut builder = CreateApplicationCommand::default();
    builder
        .kind(CommandType::ChatInput)
        .name(spec.name)
        .set_options(options)
        .description(spec.description);
    Ok(builder)
}

pub fn try_into_create_application_command_option(
    spec: CommandSpec,
) -> Result<CreateApplicationCommandOption, CommandSpecError> {
    let options = match (spec.subcommands.is_empty(), spec.options.is_empty()) {
        (false, false) => return Err(CommandSpecError),
        (false, true) => spec
            .subcommands
            .into_iter()
            .map(try_into_create_application_command_option)
            .collect::<Result<_, _>>()?,
        (true, false) => spec
            .options
            .into_iter()
            .map(into_create_application_command_option)
            .collect(),
        (true, true) => Vec::new(),
    };
    let mut builder = CreateApplicationCommandOption::default();
    builder
        .kind(CommandOptionType::SubCommand)
        .name(spec.name)
        .description(spec.description);
    for option in options {
        builder.add_sub_option(option);
    }
    Ok(builder)
}

pub fn into_create_application_command_option(
    option: CommandOption,
) -> CreateApplicationCommandOption {
    let mut builder = CreateApplicationCommandOption::default();
    builder
        .kind(match option.value.as_primitive() {
            CommandOptionValueKind::String => CommandOptionType::String,
            CommandOptionValueKind::Integer => CommandOptionType::Integer,
            CommandOptionValueKind::Double => CommandOptionType::Number,
            _ => unreachable!(),
        })
        .name(option.name)
        .description(option.description)
        .required(!option.value.is_optional());
    builder
}
