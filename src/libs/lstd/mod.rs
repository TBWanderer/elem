mod functions;
mod macroses;

use super::Library;
use crate::lang::value::Value;

pub fn init() -> Library {
    let key = |name| String::from(name);
    let fun = |function| Value::Function(function);
    let mac = |macros| Value::Macros(macros);
    Library::from([
        (key("+"), fun(functions::ladd)),
        (key("-"), fun(functions::lsub)),
        (key("*"), fun(functions::lmul)),
        (key("/"), fun(functions::ldiv)),
        (key(">"), fun(functions::lgt)),
        (key("<"), fun(functions::llt)),
        (key("="), fun(functions::leq)),
        (key("load"), fun(functions::lload)),
        (key("set"), mac(macroses::lset)),
        (key("let"), mac(macroses::llet)),
        (key("cond"), mac(macroses::lcond)),
    ])
}
