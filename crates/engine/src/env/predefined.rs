use std::collections::HashMap;

use super::Env;

pub struct PredefinedEnv(HashMap<String, String>);

impl PredefinedEnv {
    pub fn new() -> Self {
        PredefinedEnv(HashMap::new())
    }

    pub fn with(mut self, key: String, value: String) -> PredefinedEnv {
        self.0.insert(key, value);
        self
    }
}

impl Env for PredefinedEnv {
    fn get(&self, name: &str) -> Option<String> {
        self.0.get(name).cloned()
    }
}
