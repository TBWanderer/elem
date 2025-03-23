pub use ansi_term::Color;
pub const COLOR_RESET: &str = "\u{1b}[0m";
pub use std::{fs::read_to_string, process};

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

pub fn code_from_file(path: &dyn AsRef<std::path::Path>) -> String {
    let path = path.as_ref();
    if path.exists() {
        if !path.is_dir() {
            let data = std::fs::read_to_string(&path)
                .expect(&format!("Couldn't read file: {}", path.to_str().unwrap()));
            let processed_data = data
                .lines()
                .map(|line| line.split(';').collect::<Vec<&str>>()[0].trim())
                .collect::<Vec<&str>>()
                .join(" ");

            processed_data
        } else {
            println!("{} is a directory, not file", path.to_str().unwrap());
            process::exit(1);
        }
    } else {
        println!("File not found: {}", path.to_str().unwrap());
        process::exit(1);
    }
}
