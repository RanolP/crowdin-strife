use std::{collections::HashMap, str::FromStr};

pub struct CommandArgument {
    map: HashMap<String, String>,
}

impl CommandArgument {
    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.map.get(key).and_then(|s| s.parse().ok())
    }
}
