pub struct CommandSpec {
    pub name: String,
    pub options: Vec<CommandOption>,
    pub subcommands: Vec<CommandSpec>,
}

pub struct CommandOption {
    pub name: String,
    pub value: CommandOptionValueKind
}

pub enum CommandOptionValueKind {
    String,
    Integer,
    Double,
}
