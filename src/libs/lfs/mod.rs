mod functions;

use crate::lang::{Scopes, Value};
use std::rc::Rc;

type Struct = std::collections::HashMap<String, Value>;

pub const LIB_NAME: &str = "fs";

pub fn init() -> Value {
    let key = |key_name: &str| String::from(key_name);
    let fun = |lfun| Value::Function(lfun);
    let _mac = |lmac| Value::Macro(lmac);

    Value::Struct(Struct::from([
        // --- Original functions ---
        (key("read"), fun(Rc::new(functions::lread))),
        (key("write"), fun(Rc::new(functions::lwrite))),
        // --- Directory operations ---
        (key("listdir"), fun(Rc::new(functions::lls))),
        (key("mkdir"), fun(Rc::new(functions::lmkdir))),
        (key("rmdir"), fun(Rc::new(functions::lrmdir))),
        // --- File operations ---
        (key("rm"), fun(Rc::new(functions::lrm))),
        (key("cp"), fun(Rc::new(functions::lcp))),
        (key("mv"), fun(Rc::new(functions::lmv))),
        (key("exists"), fun(Rc::new(functions::lexists))),
        // --- System information ---
        (key("pwd"), fun(Rc::new(functions::lpwd))),
        (key("stat"), fun(Rc::new(functions::lstat))),
        // --- Symlink handling ---
        (key("symlink"), fun(Rc::new(functions::lsymlink))),
        (key("readlink"), fun(Rc::new(functions::lreadlink))),
    ]))
}
