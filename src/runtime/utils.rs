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

pub fn tokenize<'a>(line: &'a str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut current_token = String::new();
    let mut is_string = false;
    let mut is_escaped = false;

    for c in line.chars() {
        if is_string {
            if is_escaped {
                match c {
                    'n' => current_token.push_str("\n"),
                    't' => current_token.push_str("\t"),
                    'r' => current_token.push_str("\r"),
                    '\\' => current_token.push_str("\\"),
                    '"' => current_token.push_str("\""),
                    _ => {
                        current_token.push('\\');
                        current_token.push(c);
                    }
                }
                is_escaped = false;
            } else if c == '\\' {
                is_escaped = true;
            } else if c == '"' {
                current_token.push(c);
                tokens.push(current_token);
                current_token = String::new();
                is_string = false;
            } else {
                current_token.push(c);
            }
        } else {
            match c {
                '"' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                    current_token.push(c);
                    is_string = true;
                }
                ' ' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                }
                '(' | ')' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                    tokens.push(c.to_string());
                }
                ';' => {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                    }
                    return tokens;
                }
                _ => {
                    current_token.push(c);
                }
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

pub fn parse(tokens: Vec<String>) -> Vec<crate::lang::value::Value> {
    use crate::lang::value::Value;
    use crate::*;
    let mut stack: Vec<Value> = vec![];

    for token in tokens.iter().rev() {
        match token.as_str() {
            "(" => {
                let tmp = stack.pop().expect("Stack Error");
                let active = stack.pop().unwrap_or(nil!());
                stack.push(pair!(tmp, active))
            }
            ")" => stack.push(nil!()),
            _ => {
                if token.parse::<i128>().is_ok() {
                    let tmp = Value::Number(token.parse().unwrap());
                    let active = stack
                        .pop()
                        .expect("Syntax error! Couldn't add number to stack");
                    stack.push(pair!(tmp, active))
                } else if token.chars().next().unwrap() == '"' {
                    let tmp = Value::String(token[1..token.len() - 1].to_string());
                    let active = stack
                        .pop()
                        .expect("Syntax error! Couldn't add string to stack");
                    stack.push(pair!(tmp, active))
                } else {
                    let tmp = Value::Name(token.to_string());
                    let active = stack
                        .pop()
                        .expect("Syntax error! Couldn't add name to stack");
                    stack.push(pair!(tmp, active))
                }
            }
        }
    }

    stack.last().expect("Stack error").into()
}

pub fn run_code(code: String, scopes: &mut crate::lang::scopes::Scopes) {
    let tokens = tokenize(&code);
    let values = parse(tokens);

    for value in values {
        crate::lang::leval(value, scopes);
    }
}
