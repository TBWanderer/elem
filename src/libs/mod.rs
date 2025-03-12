mod lio;
pub mod lstd;

type Library = crate::lang::scopes::Scope;

use std::collections::HashMap;
pub fn builtins() -> HashMap<String, Library> {
    HashMap::from([("io".to_string(), lio::init())])
}
