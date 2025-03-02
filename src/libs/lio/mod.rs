mod functions;

use super::Library;
use crate::lang::value::Value;
use std::rc::Rc;

pub fn init() -> Library {
    let key = |name| String::from(name);
    let fun = |function| Value::Function(Rc::new(function));
    let _mac = |macros| Value::Macros(Rc::new(macros));
    Library::from([
        (key("write"), fun(functions::lwrite)),
        (key("print"), fun(functions::lprint)),
    ])
}
