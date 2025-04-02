use super::{super::error, Scopes, Value, LIB_NAME};

const VALUE_TYPE: &str = "func";

pub fn lwrite(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "write";

    if args.len() > 0 {
        for arg in args {
            if let Value::Error(err_text) = &arg {
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
                print!("{}", arg.show())
            }
        }
    }

    Value::Nil
}

pub fn lprint(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "print";

    let mut data = String::new().to_owned();

    if args.len() != 0 {
        for arg in args {
            if let Value::Error(err_text) = &arg {
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
                data = data + &arg.show();
            }
        }

        println!("{}", data);
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
