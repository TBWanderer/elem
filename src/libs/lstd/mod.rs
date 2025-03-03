mod functions;
mod macroses;

use super::Library;
use crate::lang::value::Value;
use std::rc::Rc;

pub fn init() -> Library {
    let key = |name| String::from(name);
    let fun = |function| Value::Function(function);
    let mac = |macros| Value::Macros(macros);
    Library::from([
        (key("+"), fun(Rc::new(functions::ladd))),
        (key("-"), fun(Rc::new(functions::lsub))),
        (key("*"), fun(Rc::new(functions::lmul))),
        (key("/"), fun(Rc::new(functions::ldiv))),
        (key(">"), fun(Rc::new(functions::lgt))),
        (key("<"), fun(Rc::new(functions::llt))),
        (key("="), fun(Rc::new(functions::leq))),
        (key("load"), fun(Rc::new(functions::lload))),
        (key("lambda"), mac(Rc::new(macroses::llambda))),
        (key("set"), mac(Rc::new(macroses::lset))),
        (key("let"), mac(Rc::new(macroses::llet))),
        (key("cond"), mac(Rc::new(macroses::lcond))),
    ])
}
