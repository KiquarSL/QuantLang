use crate::runtime::Value;
use std::collections::HashMap;

pub struct Runtime {
    /* var name | (type|:value) */
    pub vars: HashMap<String, Value>,
}

impl Runtime {
    pub fn new() -> Self {
        let vars = HashMap::new();
        Self { vars }
    }

    pub fn new_var(&mut self, name: String, value: Value) {
        if let Some(_) = self.vars.insert(name.clone(), value) {
            panic!("Variable '{}' already exists!", name.clone());
        }
    }

    pub fn set_var(&mut self, name: String, value: Value) {
        if let Some(_) = self.vars.insert(name.clone(), value) {
        } else {
            panic!("Variable '{}' not exists!", name.clone());
        }
    }

    pub fn get_var(&mut self, name: String) -> Option<&Value> {
        if !self.vars.contains_key(&name) {
            panic!("Variable '{}' not exists!", name.clone());
        }
        self.vars.get(&name)
    }
}
