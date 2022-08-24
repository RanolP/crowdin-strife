use crate::{CommandSender, CommandSpec, Context};

pub trait Command {
    fn spec() -> CommandSpec;
}
