use super::value::Value;
use std::collections::HashMap;

pub type Scope = HashMap<String, Value>;
pub struct Scopes {
    pub inner: Vec<Scope>,
}

impl Scopes {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn init_scope(&mut self) {
        self.inner.push(Scope::new())
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.inner.push(scope);
    }

    pub fn pop(&mut self) -> Option<Scope> {
        self.inner.pop()
    }

    pub fn change(&mut self, k: String, v: Value) {
        self.inner.last_mut().unwrap().insert(k, v);
    }

    pub fn get(&mut self, k: String) -> Value {
        for i in (0..self.inner.len()).rev() {
            if self.inner[i].contains_key(&k) {
                return self.inner[i].get(&k).unwrap().clone();
            }
        }
        crate::nil!()
    }

    pub fn exists(&mut self, k: String) -> bool {
        for i in (0..self.inner.len()).rev() {
            if self.inner[i].contains_key(&k) {
                return true;
            }
        }

        false
    }
}
