use super::Value;
use std::collections::HashMap;

pub type Scope = HashMap<String, Value>;
#[derive(Debug)]
pub struct Scopes {
    inner: Vec<Scope>,
    pub public: Scope,
}

impl Scopes {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            public: Scope::new(),
        }
    }

    pub fn init_scope(&mut self) {
        self.inner.push(Scope::new())
    }

    pub fn add_scope(&mut self, scope: Scope) {
        self.inner.insert(self.inner.len() - 1, scope);
    }

    pub fn change_public(&mut self, k: String, v: Value) {
        self.public.insert(k, v);
    }

    pub fn pop(&mut self) -> Option<Scope> {
        self.inner.pop()
    }

    pub fn change(&mut self, k: String, v: Value) {
        self.inner.last_mut().unwrap().insert(k, v);
    }

    pub fn change_from(&mut self, scope: Scope) {
        for key in scope.keys() {
            self.change(key.into(), scope.get(key).unwrap().clone());
        }
    }

    pub fn get(&self, k: String) -> Value {
        for i in (0..self.inner.len()).rev() {
            if self.inner[i].contains_key(&k) {
                return self.inner[i].get(&k).unwrap().clone();
            }
        }
        Value::Error(format!(
            "<sys> get_var: KeyNotFoundError - name <{}> not found in module",
            k
        ))
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
