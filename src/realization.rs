pub fn tokenize(line: String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut curr = String::from("");
    for i in line.chars().into_iter() {
        match i {
            ' ' => {
                if !curr.is_empty() {
                    tokens.push(curr);
                    curr = String::from("")
                }
            }
            '(' | ')' => {
                if !curr.is_empty() {
                    tokens.push(curr);
                    curr = String::from("")
                }
                tokens.push(String::from(i))
            }
            any => {
                curr += &String::from(any);
            }
        }
    }

    tokens
}

pub fn parse(tokens: Vec<String>) -> Vec<crate::value::Value> {
    use crate::value::Value;
    use crate::*;
    let mut stack: Vec<Value> = vec![];
    let mut tokens = tokens;
    tokens.reverse();

    for token in tokens {
        match token.as_str() {
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
                    let tmp = Value::Name(token);
                    let active = stack.pop().expect("Stack error 4");
                    stack.push(pair!(tmp, active))
                }
            }
        }
    }

    stack.last().expect("Stack error").into()
}
