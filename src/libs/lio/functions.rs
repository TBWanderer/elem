use super::{Scopes, Value};

pub fn lwrite(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    if args.len() > 1 {
        return Value::Error(
            "<func> write: Incorrect count of arguments! Expected less than 2 args".to_string(),
        );
    } else if args.len() == 1 {
        print!("{}", args[0].show())
    }

    Value::Nil
}

pub fn lprint(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    if args.len() > 1 {
        return Value::Error(
            "<func> print: Incorrect count of arguments! Expected less than 2 args".to_string(),
        );
    } else if args.len() == 1 {
        if let Value::Error(err) = &args[0] {
            return Value::Error(format!("<func> print: catched error in argument\n{}", err));
        } else {
            println!("{}", args[0].show())
        }
    } else {
        println!()
    }

    Value::Nil
}

pub fn lread(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    if args.len() != 0 {
        return Value::Error(
            "<func> read: Incorrect count of arguments! Expected 0 args".to_string(),
        );
    }

    let input = crate::utils::input("");
    Value::String(input)
}
