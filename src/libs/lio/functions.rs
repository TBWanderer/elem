use crate::lang::{scopes::Scopes, value::*};

pub fn lwrite(args: Value, _scopes: &mut Scopes) -> Value {
    use crate::nil;

    print!("{}", args);

    nil!()
}

pub fn lprint(args: Value, _scopes: &mut Scopes) -> Value {
    use crate::nil;

    println!("{}", args);

    nil!()
}
