use super::{super::error, Scopes, Value, LIB_NAME};

const VALUE_TYPE: &str = "func";

pub fn lread(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    use std::{fs::read, path::PathBuf};

    const FUNC_NAME: &str = "read";

    if args.len() != 1 {
        return Value::Error(
            "<func> read: Incorrect count of arguments! Expected 1 args".to_string(),
        );
    }

    match &args[0] {
        Value::String(path) => {
            if PathBuf::from(path).exists() {
                match read(path) {
                    Ok(data) => Value::Array(data.iter().map(|&x| Value::Byte(x)).collect()),
                    Err(err_text) => Value::Error(error::fmt(
                        LIB_NAME,
                        VALUE_TYPE,
                        FUNC_NAME,
                        "FileReadError",
                        &err_text.to_string(),
                    )),
                }
            } else {
                Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "FileNotFoundError",
                    &format!("file {} not found!", path),
                ))
            }
        }
        Value::Error(err_text) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "TypeError",
                "expected String type in arguments",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type in arguments",
        )),
    }
}

pub fn lwrite(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    use std::fs::write;

    const FUNC_NAME: &str = "read";

    if args.len() != 2 {
        return Value::Error(
            "<func> read: Incorrect count of arguments! Expected 1 args".to_string(),
        );
    }

    match (&args[0], &args[1]) {
        (Value::String(path), Value::Array(array)) => Value::Nil,
        (Value::Error(err_text), _) | (_, Value::Error(err_text)) => Value::Error(format!(
            "{}\n{}",
            error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "TypeError",
                "expected String type in arguments",
            ),
            err_text
        )),
        _ => Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "TypeError",
            "expected String type in arguments",
        )),
    }
}
