pub use std::fs::read_to_string;

pub fn input(prompt: &str) -> String {
    use std::io::{self, BufRead, Write};
    print!("{}", prompt);
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
