use super::{super::error, Scopes, Value, LIB_NAME};
use crate::lang::eval;

const VALUE_TYPE: &str = "macro";

pub fn leval(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    const _MACRO_NAME: &str = "eval";

    eval(args[0].clone(), scopes)
}

pub fn lfree(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    const MACRO_NAME: &str = "free";

    for arg in args {
        match arg {
            Value::Name(name) => scopes.remove_all(&name),
            Value::Error(err) => {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    MACRO_NAME,
                    "CatchedError",
                    &format!("catched error\n{}", err),
                ))
            }
            _ => {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    MACRO_NAME,
                    "TypeError",
                    "expected Name",
                ))
            }
        }
    }

    Value::Nil
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

pub fn llambda(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    use std::rc::Rc;
    const MACRO_NAME: &str = "lambda";

    if args.len() != 2 {
        return Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            MACRO_NAME,
            "ArgsCountError",
            "lambda requires exactly 2 arguments: parameters and body",
        ));
    }

    let params = args[0].clone();
    let body = args[1].clone();

    // Clone the current scopes to capture the lexical environment
    let captured_scopes = scopes.clone();

    Value::Function(Rc::new(
        move |lambda_args: Vec<Value>, lambda_scopes: &mut Scopes| {
            // Create a new scope that inherits from the captured scope
            lambda_scopes.init_scope_with_parent(&captured_scopes);

            let param_list: Vec<Value> = params.clone().into();

            if param_list.len() != lambda_args.len() {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    MACRO_NAME,
                    "ArgsCountError",
                    &format!(
                        "expected {} arguments, got {}",
                        param_list.len(),
                        lambda_args.len()
                    ),
                ));
            }

            for i in 0..param_list.len() {
                if let Value::Name(param_name) = &param_list[i] {
                    lambda_scopes.change(param_name.clone(), lambda_args[i].clone());
                } else {
                    return Value::Error(error::fmt(
                        LIB_NAME,
                        VALUE_TYPE,
                        MACRO_NAME,
                        "TypeError",
                        "lambda parameter must be a name",
                    ));
                }
            }

            let actions: Vec<Value> = body.clone().into();
            let result = if actions.is_empty() {
                Value::Nil
            } else {
                for action in actions.iter().take(actions.len() - 1) {
                    match leval(vec![action.clone()], lambda_scopes) {
                        Value::Error(err) => return Value::Error(err),
                        _ => {}
                    }
                }

                leval(vec![actions.last().unwrap().clone()], lambda_scopes)
            };

            lambda_scopes.pop();
            result
        },
    ))
}
