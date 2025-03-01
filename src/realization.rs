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
            _ => {}
        }
    }

    if start_idx <= curr_idx && start_idx < line.len() {
        tokens.push(&line[start_idx..]);
    }

    tokens
}

pub fn parse(tokens: Vec<&str>) -> Vec<crate::value::Value> {
    use crate::value::Value;
    use crate::*;
    let mut stack: Vec<Value> = vec![];
    let mut tokens = tokens;
    tokens.reverse();

    for token in tokens {
        match token {
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
