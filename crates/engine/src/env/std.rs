use super::Env;

pub struct StdEnv;

impl Env for StdEnv {
    fn get(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }
}
