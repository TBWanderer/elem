pub mod io {
    pub fn input(prompt: &str) -> String {
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

    pub fn read_file(path: &str) -> String {
        use std::path::PathBuf;
        let path = PathBuf::from(path);
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
                panic!("{} is a directory, not file", path.to_str().unwrap())
            }
        } else {
            panic!("File not found: {}", path.to_str().unwrap())
        }
    }
}
pub fn tokenize<'a>(line: &'a str) -> Vec<&'a str> {
    let mut tokens: Vec<&'a str> = vec![];
    let mut start_idx = 0;
    let mut curr_idx = 0;

    for (idx, c) in line.char_indices() {
        curr_idx = idx;
        match c {
            ' ' => {
                if start_idx < idx {
                    tokens.push(&line[start_idx..idx]);
                }
                start_idx = idx + 1;
            }
            '(' | ')' => {
                if start_idx < idx {
                    tokens.push(&line[start_idx..idx]);
                }
                tokens.push(&line[idx..idx + 1]);
                start_idx = idx + 1;
            }
            ';' => break,
            _ => {}
        }
    }

    if start_idx <= curr_idx && start_idx < line.len() {
        tokens.push(&line[start_idx..]);
    }

    tokens
}

pub fn parse(tokens: Vec<&str>) -> Vec<crate::lang::value::Value> {
    use crate::lang::value::Value;
    use crate::*;
    let mut stack: Vec<Value> = vec![];

    for token in tokens.iter().rev() {
        match *token {
            "(" => {
                let tmp = stack.pop().expect("Stack Error 1");
                let active = stack.pop().unwrap_or(nil!());
                stack.push(pair!(tmp, active))
            }
            ")" => stack.push(nil!()),
            _ => {
                if token.parse::<i128>().is_ok() {
                    let tmp = Value::Number(token.parse().unwrap());
                    let active = stack.pop().expect("Stack error 3");
                    stack.push(pair!(tmp, active))
                } else {
                    let tmp = Value::Name(token.to_string());
                    let active = stack.pop().expect("Stack error 4");
                    stack.push(pair!(tmp, active))
                }
            }
        }
    }

    stack.last().expect("Stack error").into()
}
