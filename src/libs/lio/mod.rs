mod functions;

use super::Library;
use crate::lang::value::Value;
use std::rc::Rc;

pub fn init() -> Library {
    let key = |name| String::from(name);
    let fun = |function| Value::Function(function);
    let _mac = |macros| Value::Macros(macros);
    Library::from([
        (key("write"), fun(Rc::new(functions::lwrite))),
        (key("print"), fun(Rc::new(functions::lprint))),
    ])
}
