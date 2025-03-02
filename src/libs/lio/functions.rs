use crate::lang::{scopes::Scopes, value::*};

pub fn lwrite(args: Value, _scopes: &mut Scopes) -> Value {
    use crate::nil;

    if args.len() > 1 {
        panic!("Too many args for print func")
    } else if args.len() == 1 {
        print!("{}", args.get(0).show())
    }

    nil!()
}

pub fn lprint(args: Value, _scopes: &mut Scopes) -> Value {
    use crate::nil;

    if args.len() > 1 {
        panic!("Too many args for print func")
    } else if args.len() == 1 {
        println!("{}", args.get(0).show())
    } else {
        println!()
    }

    nil!()
}
