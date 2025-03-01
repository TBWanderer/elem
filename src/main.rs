use elem::prelude::*;

fn input(prompt: &str) -> String {
    use std::io::{self, BufRead, Write};
    print!("{} {}", "[>]", prompt);
    match io::stdout().flush() {
        Ok(_) => (),
        Err(_) => panic!(),
    };

    match io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
    {
        Ok(input) => input,
        Err(_) => {
            panic!()
        }
    }
}

fn main() {
    let mut scopes = init_scopes();
    loop {
        let line = input("lithium@elem >> ");
        let tokens = tokenize(&line);
        let values = parse(tokens);

        for value in values {
            println!("{}", eval(value, &mut scopes));
        }
    }
}
