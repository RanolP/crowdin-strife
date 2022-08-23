use bot_any::types::User;

pub enum CommandSender {
    User(User),
    Unknown,
    System,
}
