use super::{super::error, Scopes, Value, LIB_NAME};

const VALUE_TYPE: &str = "func";

pub fn lwrite(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "write";

    if args.len() > 1 {
        return Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "ArgsCountError",
            "expected 0..1 args",
        ));
    } else if args.len() == 1 {
        print!("{}", args[0].show())
    }

    Value::Nil
}

pub fn lprint(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "print";

    if args.len() > 1 {
        return Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "ArgsCountError",
            "expected 0..1 args",
        ));
    } else if args.len() == 1 {
        if let Value::Error(err_text) = &args[0] {
            return Value::Error(format!(
                "{}\n{}",
                error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "CatchedError",
                    "got an error in args"
                ),
                err_text
            ));
        } else {
            println!("{}", args[0].show())
        }
    } else {
        println!()
    }

    Value::Nil
}

pub fn lread(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "read";

    if args.len() == 0 {
        let input = crate::utils::input("");
        Value::String(input)
    } else if args.len() == 1 {
        if let Value::String(prompt) = &args[0] {
            let input = crate::utils::input(&prompt);
            Value::String(input)
        } else {
            return Value::Error(error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "TypeError",
                "expected String type in arguments",
            ));
        }
    } else {
        return Value::Error(error::fmt(
            LIB_NAME,
            VALUE_TYPE,
            FUNC_NAME,
            "ArgsCountError",
            "expected 0..1 args",
        ));
    }
}
