pub mod lio;
pub mod lstd;

use crate::lang::Value;
use std::collections::HashMap;

pub fn builtins() -> HashMap<String, Value> {
    let key = |key: &str| key.to_string();
    HashMap::from([(key("io"), lio::init())])
}
