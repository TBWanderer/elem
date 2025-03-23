pub mod error;
mod eval;

pub use eval::eval;

mod scopes;
pub use scopes::{Scope, Scopes};

use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub enum Value {
    Nil,
    Error(String),
    Name(String),
    Number(i128),
    Byte(u8),
    String(String),
    Pair(Rc<Value>, Rc<Value>),
    Function(Rc<dyn Fn(Vec<Value>, &mut Scopes) -> Value>),
    Macro(Rc<dyn Fn(Vec<Value>, &mut Scopes) -> Value>),
    Struct(HashMap<String, Value>),
    Array(Vec<Value>),
}

impl Value {
    pub fn is_list(&self) -> bool {
        let mut current = self;
        while let Value::Pair(_, cdr) = current {
            current = cdr;
        }
        matches!(current, Value::Nil)
    }

    pub fn show(&self) -> String {
        match self {
            Value::String(string) => string.to_string(),
            other => format!("{}", other),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "()"),
            Value::Error(err) => write!(f, "Error {}", err),
            Value::Number(n) => write!(f, "{}", n),
            Value::Byte(byte) => write!(f, "{}", byte),
            Value::String(string) => write!(f, r#""{}""#, string),
            Value::Name(name) => write!(f, "<{}>", name),
            Value::Function(_) => write!(f, "<function>"),
            Value::Macro(_) => write!(f, "<macro>"),
            Value::Pair(car, cdr) => {
                write!(f, "(")?;
                write!(f, "{}", car)?;

                let mut current = cdr;
                while let Value::Pair(next_car, next_cdr) = current.as_ref() {
                    write!(f, " {}", next_car)?;
                    current = next_cdr;
                }

                match current.as_ref() {
                    Value::Nil => write!(f, ")"),
                    _ => write!(f, " . {})", current),
                }
            }
            Value::Struct(_) => write!(f, "<struct>"),
            Value::Array(array) => {
                write!(f, "[")?;
                if let Some((first, rest)) = array.split_first() {
                    write!(f, "{}", first)?;
                    for item in rest {
                        write!(f, ", {}", item)?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Error(err) => write!(f, "Error({:?})", err),
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Byte(byte) => write!(f, "Byte({})", byte),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Name(n) => write!(f, "Name({:?})", n),
            Value::Pair(car, cdr) => write!(f, "Pair({:?}, {:?})", car, cdr),
            Value::Function(_) => write!(f, "Function(...)"),
            Value::Macro(_) => write!(f, "Macro(...)"),
            Value::Struct(map) => write!(f, "Struct({:?})", map),
            Value::Array(array) => write!(f, "Array({:?})", array),
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(values: Vec<Value>) -> Self {
        let mut result = Value::Nil;

        for value in values.into_iter().rev() {
            result = Value::Pair(Rc::new(value), Rc::new(result));
        }

        result
    }
}

impl From<Value> for Vec<Value> {
    fn from(value: Value) -> Self {
        let mut elements = Vec::new();
        let mut current = value;

        while let Value::Pair(car, cdr) = current {
            elements.push((*car).clone());
            current = (*cdr).clone();
        }

        elements
    }
}
