mod operators;
mod value;

use operators::*;
use value::*;

fn main() {
    let values = vec![
        Value::Pair(Box::new((
            Value::Number(153),
            Value::Pair(Box::new((
                Value::Number(200),
                Value::Pair(Box::new((Value::Number(20), Value::Nil))),
            ))),
        ))),
        Value::Pair(Box::new((
            Value::Number(32),
            Value::Pair(Box::new((Value::Number(8), Value::Nil))),
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
        Value::Pair(Box::new((Value::Name("aboba".to_string()), Value::Nil))),
    ];

    for value in &values {
        println!("Show test: {}", value.show());
    }
    println!();

    println!("Add test: {}", ladd(values[0].clone()));
    println!("Mul test: {}", lmul(values[0].clone()));
    println!("Greater test: {}", lgt(values[1].clone()));
    println!("Lesser test: {}", llt(values[1].clone()));
    println!("Equal test: {}", leq(values[1].clone()));
    println!("Divide test: {}", ldiv(values[1].clone()));
    println!("Substract test: {}", lsub(values[1].clone()))
}
