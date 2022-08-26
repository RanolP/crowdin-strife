pub struct CommandSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub options: Vec<CommandOption>,

    pub subcommands: Vec<CommandSpec>,
}

pub struct CommandOption {
    pub name: &'static str,
    pub description: &'static str,
    pub value: CommandOptionValueKind,
}

#[derive(Clone)]
pub enum CommandOptionValueKind {
    Optional(Box<CommandOptionValueKind>),
    String,
    Integer,
    Double,
}

impl CommandOptionValueKind {
    pub fn is_optional(&self) -> bool {
        match self {
            CommandOptionValueKind::Optional(_) => true,
            _ => false,
        }
    }

    pub fn as_primitive(&self) -> CommandOptionValueKind {
        match self {
            CommandOptionValueKind::Optional(t) => t.as_primitive(),
            _ => self.clone(),
        }
    }
}

pub trait GetCommandOptionValueKind {
    fn get_command_option_value_kind() -> CommandOptionValueKind;
}

impl<T: GetCommandOptionValueKind> GetCommandOptionValueKind for Option<T> {
    fn get_command_option_value_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Optional(Box::new(T::get_command_option_value_kind()))
    }
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
