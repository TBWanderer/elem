use crate::lang::{scopes::Scopes, value::*};

pub fn lwrite(args: Value, _scopes: &mut Scopes) -> Value {
    use crate::nil;

    if args.len() > 1 {
        return Value::Error(
            "<func> write: Incorrect count of arguments! Expected less than 2 args".to_string(),
        );
    } else if args.len() == 1 {
        print!("{}", args.get(0).show())
    }

    nil!()
}

pub fn lprint(args: Value, _scopes: &mut Scopes) -> Value {
    use crate::nil;

    if args.len() > 1 {
        return Value::Error(
            "<func> print: Incorrect count of arguments! Expected less than 2 args".to_string(),
        );
    } else if args.len() == 1 {
        let arg = args.get(0);
        if let Value::Error(err) = arg {
            return Value::Error(format!("<func> print: catched error in argument\n{}", err));
        } else {
            println!("{}", arg.show())
        }
    } else {
        println!()
    }

    nil!()
}

pub fn lread(args: Value, _scopes: &mut Scopes) -> Value {
    if args.len() != 0 {
        return Value::Error(
            "<func> read: Incorrect count of arguments! Expected 0 args".to_string(),
        );
    }

    let input = crate::runtime::utils::io::input("");
    Value::String(input)
}
