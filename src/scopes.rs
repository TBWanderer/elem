use super::value::Value;
use std::collections::HashMap;

pub type Scope = HashMap<String, Value>;
pub type Scopes = Vec<Scope>;

pub fn new(scopes: &mut Scopes) {
    scopes.push(Scope::new());
}

pub fn pop(mut scopes: Scopes) {
    scopes.pop();
}

pub fn change(mut scopes: Scopes, k: String, v: Value) {
    scopes.last_mut().unwrap().insert(k, v);
}

pub fn get(scopes: Scopes, k: String) -> Value {
    for i in (1..scopes.len()).rev() {
        if scopes[i].contains_key(&k) {
            return scopes[i].get(&k).unwrap().clone();
        }
    }
    panic!(r#"Key not exists: "{}"#, k)
}
