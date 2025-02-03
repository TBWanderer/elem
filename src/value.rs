#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Nil,
    Number(i128),
    Name(String),
    Pair(Box<(Value, Value)>),
}

pub trait Show {
    fn show(&self) -> String;
}

impl Show for Value {
    fn show(&self) -> String {
        match self {
            Value::Nil => String::from("()"),
            Value::Number(number) => format!("{}", number),
            Value::Name(name) => format!("{}", name),
            Value::Pair(pair) => {
                let mut result = String::from("(");
                result.push_str(&pair.0.show());
                let mut current = &pair.1;

                while let Value::Pair(next_pair) = current {
                    result.push(' ');
                    result.push_str(&next_pair.0.show());
                    current = &next_pair.1;
                }

                match current {
                    Value::Nil => result.push(')'),
                    _ => {
                        result.push_str(" . ");
                        result.push_str(&current.show());
                        result.push(')');
                    }
                }

                result
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}

impl std::ops::Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(a) => {
                let Value::Number(b) = rhs else { panic!() };

                return Value::Number(a + b);
            }
            _ => panic!(),
        }
    }
}

impl std::ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(a) => {
                let Value::Number(b) = rhs else { panic!() };

                *self = Value::Number(*a + b);
            }
            _ => panic!(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Number(a) => {
                let Value::Number(b) = rhs else { panic!() };

                return Value::Number(a * b);
            }
            _ => panic!(),
        }
    }
}

impl std::ops::MulAssign for Value {
    fn mul_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(a) => {
                let Value::Number(b) = rhs else { panic!() };

                *self = Value::Number(*a * b);
            }
            _ => panic!(),
        }
    }
}

pub fn get_two(list: Value) -> (Value, Value) {
    let save = list.clone();

    let Value::Pair(pair) = list else {
        panic!("{} is not a list!", list)
    };
    let a = pair.0;

    let Value::Pair(pair) = pair.1 else {
        panic!("{} is not a list!", pair.1)
    };
    let b = pair.0;

    if pair.1 != Value::Nil {
        panic!("{} is not a list!", save)
    }

    (a, b)
}
