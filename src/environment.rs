use std::collections::HashMap;

pub struct Environment<V> {
    vars: HashMap<String, V>,
}

impl<V> Environment<V> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: &str, value: V) {
        self.vars.insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&V> {
        self.vars.get(name)
    }

    pub fn assign(&mut self, name: &str, value: V) {
        todo!()
    }
}
