pub struct CommandSpec {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub options: Vec<CommandOption>,

    pub subcommands: Vec<CommandSpec>,
}

pub struct CommandOption {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub value: CommandOptionValueKind,
}

pub enum CommandOptionValueKind {
    String,
    Integer,
    Double,
}

pub trait GetCommandOptionValueKind {
    fn get_command_option_value_kind() -> CommandOptionValueKind;
}

impl GetCommandOptionValueKind for String {
    fn get_command_option_value_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::String
    }
}

impl GetCommandOptionValueKind for i64 {
    fn get_command_option_value_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Integer
    }
}

impl GetCommandOptionValueKind for f64 {
    fn get_command_option_value_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Double
    }
}
