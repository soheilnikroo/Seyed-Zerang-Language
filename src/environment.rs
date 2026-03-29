use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment<V: Clone> {
    parent: Option<Rc<Environment<V>>>,
    vars: RefCell<HashMap<String, V>>,
}

impl<V: Clone> Environment<V> {
    pub fn new(parent: Option<Rc<Environment<V>>>) -> Rc<Environment<V>> {
        Rc::new(Self {
            parent,
            vars: HashMap::new().into(),
        })
    }

    pub fn declare(&self, name: &str, value: V) {
        self.vars.borrow_mut().insert(name.into(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<V> {
        if let Some(value) = self.vars.borrow().get(name) {
            Some(value.clone())
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }

    pub fn assign(&self, name: &str, value: V) -> Option<V> {
        if self.vars.borrow().contains_key(name) {
            self.vars.borrow_mut().insert(name.into(), value.clone());
            Some(value)
        } else if let Some(ref parent) = self.parent {
            parent.assign(name, value)
        } else {
            None
        }
    }
}
