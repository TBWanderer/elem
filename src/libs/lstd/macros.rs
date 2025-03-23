use super::{super::error, Scopes, Value, LIB_NAME};
use crate::lang::eval;

const VALUE_TYPE: &str = "macro";

pub fn leval(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    const _MACRO_NAME: &str = "eval";

    eval(args[0].clone(), scopes)
}

pub fn lset(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    const MACRO_NAME: &str = "set";

    if args.len() != 2 {
        return Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            MACRO_NAME,
            "ArgsCountError",
            "incorrect count of args. Expected 2 args",
        ));
    }

    match &args[0] {
        Value::Name(name) => match eval(args[1].clone(), scopes) {
            Value::Error(err_text) => Value::Error(format!(
                "{}\n{}",
                error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    MACRO_NAME,
                    "CatchedError",
                    "got an error in args"
                ),
                err_text
            )),
            value => {
                scopes.change(name.into(), value);
                Value::Nil
            }
        },
        Value::Error(err_text) => {
            return Value::Error(format!(
                "{}\n{}",
                error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    MACRO_NAME,
                    "CatchedError",
                    "got an error in args"
                ),
                err_text
            ))
        }
        _ => {
            return Value::Error(error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                MACRO_NAME,
                "TypeError",
                "expected Name type in arguments",
            ))
        }
    }
}

pub fn lpub(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    const MACRO_NAME: &str = "pub";

    for arg in args {
        if let Value::Name(name) = arg {
            match scopes.get(name.clone()) {
                Value::Error(err_text) => {
                    return Value::Error(format!(
                        "{}\n{}",
                        error::fmt(
                            LIB_NAME,
                            VALUE_TYPE,
                            MACRO_NAME,
                            "CatchedError",
                            "got an error in args"
                        ),
                        err_text
                    ))
                }
                value => scopes.change_public(name, value),
            }
        } else {
            return Value::Error(error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                MACRO_NAME,
                "TypeError",
                "expected Name type in arguments",
            ));
        }
    }

    Value::Nil
}
