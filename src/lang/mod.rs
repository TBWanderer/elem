mod scopes;
pub use scopes::{Scope, Scopes};

use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub enum Value {
    Nil,
    Error(String),
    Name(String),
    Number(i128),
    String(String),
    Pair(Rc<Value>, Rc<Value>),
    Function(Rc<dyn Fn(Value, &mut Scopes) -> Value>),
    Macro(Rc<dyn Fn(Value, &mut Scopes) -> Value>),
    Struct(HashMap<String, Value>),
}

pub fn eval(value: Value, scopes: &mut Scopes) -> Value {
    match value {
        Value::Name(name) => scopes.get(name),
        Value::Pair(car, cdr) => match eval((*car).clone(), scopes) {
            Value::Macro(lmacro) => lmacro((*cdr).clone(), scopes),
            Value::Function(lfunc) => {
                if !(*cdr).is_list() {
                    Value::Error(format!("<sys> eval: TypeError - this is not list: {}", cdr))
                } else {
                    let args_vec: Vec<Value> = (*cdr).clone().into();
                    let evaluated_args = Value::from(
                        args_vec
                            .into_iter()
                            .map(|x| eval(x, scopes))
                            .collect::<Vec<Value>>(),
                    );
                    lfunc(evaluated_args, scopes)
                }
            }
            Value::Struct(structure) => {
                scopes.init_scope();
                scopes.change_from(structure);
                match eval((*cdr).clone(), scopes) {
                    Value::Error(err) => {
                        Value::Error(format!("<sys> eval: catched Error\n{}", err))
                    }
                    any_other => {
                        let updated_structure = scopes.pop().unwrap();
                        scopes.change(car.to_string(), Value::Struct(updated_structure));

                        any_other
                    }
                }
            }
            _ => Value::Error(format!(
                "<sys> eval: SyntaxError - can't eval this list: {}\nScopes: {:?}",
                Value::Pair((*car).clone().into(), (*cdr).clone().into()),
                scopes
            )),
        },
        _ => value,
    }
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
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Error(err) => write!(f, "Error({:?})", err),
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Name(n) => write!(f, "Name({:?})", n),
            Value::Pair(car, cdr) => write!(f, "Pair({:?}, {:?})", car, cdr),
            Value::Function(_) => write!(f, "Function(...)"),
            Value::Macro(_) => write!(f, "Macro(...)"),
            Value::Struct(map) => write!(f, "Struct({:?})", map),
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
