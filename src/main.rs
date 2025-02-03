enum Value {
    Nil,
    Number(i128),
    Name(String),
    Pair(Box<(Value, Value)>),
}

trait Show {
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

fn main() {
    let values = vec![
        Value::Pair(Box::new((
            Value::Number(1),
            Value::Pair(Box::new((
                Value::Number(2),
                Value::Pair(Box::new((Value::Number(3), Value::Nil))),
            ))),
        ))),
        Value::Pair(Box::new((
            Value::Number(1),
            Value::Pair(Box::new((Value::Number(2), Value::Number(3)))),
        ))),
        Value::Pair(Box::new((
            Value::Pair(Box::new((Value::Number(1), Value::Number(2)))),
            Value::Number(3),
        ))),
        Value::Pair(Box::new((
            Value::Pair(Box::new((Value::Number(1), Value::Number(2)))),
            Value::Pair(Box::new((Value::Number(3), Value::Nil))),
        ))),
        Value::Pair(Box::new((Value::Nil, Value::Number(1)))),
        Value::Pair(Box::new((Value::Number(228), Value::Nil))),
    ];

    for value in values {
        println!("{}", value.show());
    }
}
