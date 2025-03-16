use super::Token;
use crate::lang::Value;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnmatchedClosingParen,
    UnmatchedOpeningParen,
    InvalidStringLiteral,
    UnexpectedToken(String),
    EmptyExpression,
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Value>, ParseError> {
    let mut stack: Vec<Vec<Value>> = vec![vec![]];
    let mut results = vec![];

    for token in tokens {
        match token {
            Token::LeftParen => stack.push(vec![]),
            Token::RightParen => {
                let current = stack.pop().ok_or(ParseError::UnmatchedClosingParen)?;
                let list = build_list(current)?;

                if stack.is_empty() {
                    results.push(list);
                } else {
                    stack
                        .last_mut()
                        .ok_or(ParseError::UnmatchedClosingParen)?
                        .push(list);
                }
            }
            Token::String(s) => {
                let value = parse_string(s)?;
                stack.last_mut().unwrap().push(value);
            }
            Token::Name(s) => {
                let value = parse_symbol(s)?;
                stack.last_mut().unwrap().push(value);
            }
        }
    }

    let root = stack.pop().ok_or(ParseError::EmptyExpression)?;
    if !stack.is_empty() {
        return Err(ParseError::UnmatchedOpeningParen);
    }

    results.extend(build_root_expressions(root)?);

    Ok(results)
}

fn build_root_expressions(elements: Vec<Value>) -> Result<Vec<Value>, ParseError> {
    let mut current = Value::Nil;
    for element in elements.into_iter().rev() {
        current = Value::Pair(Rc::new(element), Rc::new(current));
    }

    let mut expressions = vec![];
    let mut current = current;
    while let Value::Pair(car, cdr) = current {
        expressions.push((*car).clone());
        current = (*cdr).clone();
    }

    Ok(expressions)
}

fn build_list(elements: Vec<Value>) -> Result<Value, ParseError> {
    elements
        .into_iter()
        .rfold(Ok(Value::Nil), |acc, item| match acc {
            Ok(list) => Ok(Value::Pair(Rc::new(item), Rc::new(list))),
            err => err,
        })
}

fn parse_symbol(s: String) -> Result<Value, ParseError> {
    s.parse().map(Value::Number).or_else(|_| Ok(Value::Name(s)))
}

fn parse_string(s: String) -> Result<Value, ParseError> {
    if s.starts_with('"') && s.ends_with('"') {
        Ok(Value::String(s[1..s.len() - 1].to_string()))
    } else {
        Err(ParseError::InvalidStringLiteral)
    }
}
