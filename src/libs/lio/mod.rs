mod functions;

use crate::lang::{Scopes, Value};
use std::rc::Rc;

type Struct = std::collections::HashMap<String, Value>;

pub const LIB_NAME: &str = "io";

pub fn init() -> Value {
    let key = |key_name: &str| String::from(key_name);
    let fun = |lfun| Value::Function(lfun);
    let _mac = |lmac| Value::Macro(lmac);
    Value::Struct(Struct::from([
        (key("write"), fun(Rc::new(functions::lwrite))),
        (key("print"), fun(Rc::new(functions::lprint))),
        (key("read"), fun(Rc::new(functions::lread))),
    ]))
}
