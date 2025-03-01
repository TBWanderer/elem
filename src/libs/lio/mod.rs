mod functions;

use super::Library;
use crate::lang::value::Value;

pub fn init() -> Library {
    let key = |name| String::from(name);
    let fun = |function| Value::Function(function);
    let _mac = |macros| Value::Macros(macros);
    Library::from([
        (key("write"), fun(functions::lwrite)),
        (key("print"), fun(functions::lprint)),
    ])
}
