use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Nil,
    Number(i128),
    String(String),
    Name(String),
    Pair(Rc<Value>, Rc<Value>),
    Function(Rc<dyn Fn(Value, &mut super::scopes::Scopes) -> Value>),
    Macros(Rc<dyn Fn(Value, &mut super::scopes::Scopes) -> Value>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Name(a), Value::Name(b)) => a == b,
            (Value::Pair(car1, cdr1), Value::Pair(car2, cdr2)) => car1 == car2 && cdr1 == cdr2,
            (Value::Function(_), Value::Function(_)) => false,
            (Value::Macros(_), Value::Macros(_)) => false,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Nil, Value::Nil) => Ordering::Equal,
            (Value::Nil, _) => Ordering::Less,
            (_, Value::Nil) => Ordering::Greater,

            (Value::Number(a), Value::Number(b)) => a.cmp(b),
            (Value::Number(_), _) => Ordering::Less,
            (_, Value::Number(_)) => Ordering::Greater,

            (Value::String(a), Value::String(b)) => a.cmp(b),
            (Value::String(_), _) => Ordering::Less,
            (_, Value::String(_)) => Ordering::Greater,

            (Value::Name(a), Value::Name(b)) => a.cmp(b),
            (Value::Name(_), _) => Ordering::Less,
            (_, Value::Name(_)) => Ordering::Greater,

            (Value::Pair(car1, cdr1), Value::Pair(car2, cdr2)) => match car1.cmp(car2) {
                Ordering::Equal => cdr1.cmp(cdr2),
                ord => ord,
            },
            (Value::Pair(_, _), _) => Ordering::Less,
            (_, Value::Pair(_, _)) => Ordering::Greater,

            (Value::Function(_), Value::Function(_)) => Ordering::Equal,
            (Value::Function(_), _) => Ordering::Less,
            (_, Value::Function(_)) => Ordering::Greater,

            (Value::Macros(_), Value::Macros(_)) => Ordering::Equal,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Name(n) => write!(f, "Name({:?})", n),
            Value::Pair(car, cdr) => write!(f, "Pair({:?}, {:?})", car, cdr),
            Value::Function(_) => write!(f, "Function(...)"),
            Value::Macros(_) => write!(f, "Macros(...)"),
        }
    }
}

pub trait List {
    fn is_list(&self) -> bool;
    fn get(&self, index: usize) -> &Value;
    fn len(&self) -> usize;
}

impl List for Value {
    fn is_list(&self) -> bool {
        let mut current = self;
        while let Value::Pair(_, cdr) = current {
            current = cdr;
        }
        matches!(current, Value::Nil)
    }

    fn get(&self, index: usize) -> &Value {
        if self.is_list() {
            let mut current = self;
            let mut i = 0;

            while let Value::Pair(car, cdr) = current {
                if i == index {
                    return car;
                }
                current = cdr;
                i += 1;
            }
        }
        &Value::Nil
    }

    fn len(&self) -> usize {
        let v: Vec<Value> = self.into();
        v.len()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "()"),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(string) => write!(f, r#""{}""#, string),
            Value::Name(name) => write!(f, "<{}>", name),
            Value::Function(_) => write!(f, "<function>"),
            Value::Macros(_) => write!(f, "<macros>"),
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
        }
    }
}

pub trait Show {
    fn show(&self) -> String;
}

impl Show for Value {
    fn show(&self) -> String {
        match self {
            Value::String(string) => string.to_string(),
            other => format!("{}", other),
        }
    }
}

// Rest of the implementations remain the same...

impl From<i128> for Value {
    fn from(n: i128) -> Self {
        Value::Number(n)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Name(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Name(s)
    }
}

impl From<&Value> for Value {
    fn from(v: &Value) -> Self {
        v.clone()
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

impl From<Vec<Value>> for Value {
    fn from(values: Vec<Value>) -> Self {
        let mut result = Value::Nil;

        for value in values.into_iter().rev() {
            result = Value::Pair(Rc::new(value), Rc::new(result));
        }

        result
    }
}
impl From<&Value> for Vec<Value> {
    fn from(value: &Value) -> Self {
        let mut elements = Vec::new();
        let mut current = value;

        while let Value::Pair(car, cdr) = current {
            elements.push((**car).clone());
            current = &**cdr;
        }

        elements
    }
}

#[macro_export]
macro_rules! list {
    () => {
        $crate::lang::value::Value::Nil
    };
    ($elem:expr $(, $rest:expr)*) => {
        $crate::lang::value::Value::Pair(std::rc::Rc::new($elem.into()), std::rc::Rc::new(list!($($rest),*)))
    };
}

#[macro_export]
macro_rules! num {
    ($x:expr) => {
        $crate::lang::value::Value::Number($x)
    };
}

#[macro_export]
macro_rules! name {
    ($x:expr) => {
        $crate::lang::value::Value::Name($x.to_string())
    };
}

#[macro_export]
macro_rules! nil {
    () => {
        $crate::lang::value::Value::Nil
    };
}

#[macro_export]
macro_rules! pair {
    ($car:expr, $cdr:expr) => {
        $crate::lang::value::Value::Pair(
            std::rc::Rc::new($car.into()),
            std::rc::Rc::new($cdr.into()),
        )
    };
}
