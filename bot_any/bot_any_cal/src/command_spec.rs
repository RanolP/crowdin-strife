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

pub trait CommandOptionValueTy: Sized {
    fn spec_kind() -> CommandOptionValueKind;

    fn default() -> Option<Self> {
        None
    }
}

impl<T: CommandOptionValueTy> CommandOptionValueTy for Option<T> {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Optional(Box::new(T::spec_kind()))
    }

    fn default() -> Option<Self> {
        Some(None)
    }
}

impl CommandOptionValueTy for String {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::String
    }
}

impl CommandOptionValueTy for i64 {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Integer
    }
}

impl CommandOptionValueTy for f64 {
    fn spec_kind() -> CommandOptionValueKind {
        CommandOptionValueKind::Double
    }
}
