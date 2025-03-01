use crate::operators::*;
use crate::scopes::*;
use crate::value::Value;

pub use crate::operators::leval as eval;
pub use crate::realization::*;
pub use crate::*;

pub fn stdlib() -> Scope {
    Scope::from([
        (String::from("+"), Value::Function(ladd)),
        (String::from("-"), Value::Function(lsub)),
        (String::from("/"), Value::Function(ldiv)),
        (String::from("*"), Value::Function(lmul)),
        (String::from(">"), Value::Function(lgt)),
        (String::from("<"), Value::Function(llt)),
        (String::from("="), Value::Function(leq)),
        (String::from("let"), Value::Function(llet)),
        (String::from("set"), Value::Function(lset)),
    ])
}

pub fn init_scopes() -> Scopes {
    let mut scopes = Scopes::new();
    scopes.push(stdlib());
    scopes
}
