pub mod lfs;
pub mod lio;
pub mod lstd;

use crate::lang::{error, Value};
use std::collections::HashMap;

pub fn builtins() -> HashMap<String, Value> {
    let key = |key: &str| key.to_string();
    HashMap::from([
        (key(lio::LIB_NAME), lio::init()),
        (key(lfs::LIB_NAME), lfs::init()),
    ])
}
