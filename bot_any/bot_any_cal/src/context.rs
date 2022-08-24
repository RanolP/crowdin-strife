use crate::CommandSender;

pub trait Context {
    fn var(&self, key: &str) -> Option<String>;
}
