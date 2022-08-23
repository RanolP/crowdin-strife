use crate::CommandSender;

pub struct Command<Args> {
    pub sender: CommandSender,
    pub label: String,
    pub arguments: Args,
}
