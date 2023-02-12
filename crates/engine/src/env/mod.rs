pub use self::std::*;

mod std;

pub trait Env {
    fn get(&self, name: &str) -> Option<String>;
}
