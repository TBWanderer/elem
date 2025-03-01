use std::fmt;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Value {
    Nil,
    Number(i128),
    Name(String),
    Pair(Rc<Value>, Rc<Value>),
    Function(fn(Value, &mut super::scopes::Scopes) -> Value),
    Macros(fn(Value, &mut super::scopes::Scopes) -> Value),
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
            Value::Name(name) => write!(f, "{}", name),
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
