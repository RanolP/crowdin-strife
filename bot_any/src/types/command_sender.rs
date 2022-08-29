use super::User;

pub enum CommandSender {
    User(User),
    Unknown,
    System,
}
