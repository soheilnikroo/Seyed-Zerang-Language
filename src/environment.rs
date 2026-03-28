use std::collections::HashMap;

struct Environment<V> {
    vars: HashMap<String, V>,
}

impl<V> Environment<V> {
    pub fn new() -> Self<V> {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn extend(&mut self, name: &str, value: V) {
        todo!()
    }

    pub fn lookup(&self, name: &str) -> V {
        todo!()
    }

    pub fn assign(&mut self, name: &str, value: V) {
        todo!()
    }
}
