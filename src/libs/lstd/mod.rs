mod functions;
mod macros;

use crate::lang::{Scopes, Value};
use std::rc::Rc;

type Struct = std::collections::HashMap<String, Value>;

pub const LIB_NAME: &str = "std";

pub fn init() -> Value {
    let key = |key_name: &str| String::from(key_name);
    let fun = |lfun| Value::Function(lfun);
    let mac = |lmac| Value::Macro(lmac);
    Value::Struct(Struct::from([
        (key("free"), mac(Rc::new(macros::lfree))),
        (key("pub"), mac(Rc::new(macros::lpub))),
        (key("set"), mac(Rc::new(macros::lset))),
        (key("lambda"), mac(Rc::new(macros::llambda))),
        (key("eval"), mac(Rc::new(macros::leval))),
        (key("evals"), fun(Rc::new(functions::levals))),
        (key("import"), fun(Rc::new(functions::limport))),
        (key("array"), fun(Rc::new(functions::llist))),
        (key("add"), fun(Rc::new(functions::ladd))),
        (key("mul"), fun(Rc::new(functions::lmul))),
        // Adding bool names
        (key("true"), Value::Bool(true)),
        (key("false"), Value::Bool(false)),
    ]))
}
