use std::collections::HashMap;

use crate::object::Object;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: &str, obj: Object) -> Option<Object> {
        self.store.insert(name.to_string(), obj)
    }

    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn show(&self) {
        for (key, value) in &self.store {
            println!("{} = {}", key, value.to_string());
        }
    }
}
