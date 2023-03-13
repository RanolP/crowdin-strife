use std::collections::{
    hash_map::{IntoIter, Iter, Values},
    HashMap,
};

pub struct LocresNamespace {
    name: String,
    map: HashMap<String, String>,
}

impl LocresNamespace {
    pub fn new(name: String) -> Self {
        LocresNamespace {
            name,
            map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn into_iter(self) -> IntoIter<String, String> {
        self.map.into_iter()
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.map.get(key)
    }
}
