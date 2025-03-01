use elem::prelude::*;

fn main() {
    let mut scopes = init_scopes();
    let line = "(+ (1 2))";
    let tokens = tokenize(line.to_string());
    let values = parse(tokens);

    for value in values {
        println!("{}", eval(value, &mut scopes));
    }
}
