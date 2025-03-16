use super::{Scopes, Value};
use crate::lang::eval;

pub fn leval(args: Value, scopes: &mut Scopes) -> Value {
    let args: Vec<Value> = args.into();
    eval(args[0].clone(), scopes)
}

pub fn lset(args: Value, scopes: &mut Scopes) -> Value {
    let args: Vec<Value> = args.into();
    if args.len() != 2 {
        return Value::Error("<macro> set: ArgsCountError - incorrect count of args".to_string());
    }

    if let Value::Name(name) = &args[0] {
        let value = eval(args[1].clone(), scopes);
        scopes.change(name.into(), value);
    } else {
        return Value::Error("<macro> set: TypeError".to_string());
    }

    Value::Nil
}

pub fn lpub(args: Value, scopes: &mut Scopes) -> Value {
    let args: Vec<Value> = args.into();
    for arg in args {
        if let Value::Name(name) = arg {
            match scopes.get(name.clone()) {
                Value::Error(err) => return Value::Error(format!("<macro> pub\n{}", err)),
                value => scopes.change_public(name, value),
            }
        } else {
            return Value::Error(
                "<macro> pub: TypeError - expected Name type for arguments".to_string(),
            );
        }
    }

    Value::Nil
}
