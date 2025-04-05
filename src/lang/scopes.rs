use super::Value;
use std::collections::HashMap;

pub type Scope = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct Scopes {
    inner: Vec<Scope>,
    pub public: Scope,
    captured_scopes: Option<Box<Scopes>>,
}

impl Scopes {
    // Existing methods (unchanged)
    pub fn new() -> Self {
        Self {
            inner: vec![],
            public: Scope::new(),
            captured_scopes: None,
        }
    }

    pub fn init_scope(&mut self) {
        self.inner.push(Scope::new())
    }

    pub fn init_scope_with_parent(&mut self, parent_scopes: &Scopes) {
        self.inner.push(Scope::new());
        self.captured_scopes = Some(Box::new(parent_scopes.clone()));
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
        if let Some(parent) = &self.captured_scopes {
            return parent.get(k);
        }
        Value::Error(format!(
            "<sys> get_var: KeyNotFoundError - name <{}> not found in module",
            k
        ))
    }

    pub fn exists(&self, k: String) -> bool {
        for i in (0..self.inner.len()).rev() {
            if self.inner[i].contains_key(&k) {
                return true;
            }
        }
        if let Some(parent) = &self.captured_scopes {
            return parent.exists(k);
        }
        false
    }

    // New methods
    pub fn remove(&mut self, k: &str) {
        for scope in self.inner.iter_mut().rev() {
            if scope.remove(k).is_some() {
                return;
            }
        }
    }

    pub fn remove_all(&mut self, k: &str) {
        for scope in self.inner.iter_mut() {
            scope.remove(k);
        }
        if let Some(captured) = &mut self.captured_scopes {
            captured.remove_all(k);
        }
    }
}
