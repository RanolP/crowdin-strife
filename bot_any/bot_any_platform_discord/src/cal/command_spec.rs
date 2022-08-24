use bot_any_cal::CommandSpec;

use crate::sys::types::{ApplicationCommand, ApplicationCommandOption};

impl From<CommandSpec> for ApplicationCommand {
    fn from(spec: CommandSpec) -> Self {
        todo!()
    }
}

impl From<CommandSpec> for ApplicationCommandOption {
    fn from(spec: CommandSpec) -> Self {
        todo!()
    }
}
