pub use self::std::*;
pub use layered::*;
pub use predefined::*;

mod layered;
mod predefined;
mod std;

pub trait Env {
    fn get(&self, name: &str) -> Option<String>;
}
