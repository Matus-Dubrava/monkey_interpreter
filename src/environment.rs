use std::collections::HashMap;

use crate::object::Object;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<Object> {
        self.get(name)
    }

    pub fn set(&mut self, name: &str, obj: Object) -> Object {
        self.set(name, obj)
    }

    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }
}
