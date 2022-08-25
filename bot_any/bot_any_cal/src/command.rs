use crate::{CommandPreflight, CommandSpec};

pub trait Command: Sized {
    const NAME: &'static str;

    fn spec() -> CommandSpec;

    fn parse(preflights: &[CommandPreflight]) -> Option<Self>;
}
