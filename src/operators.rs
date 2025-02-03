use super::value::*;

pub fn ladd(mut list: Value) -> Value {
    let mut sum = Value::Number(0);

    let save = list.clone();

    while let Value::Pair(pair) = &list {
        sum += pair.0.clone();
        list = pair.1.clone();
    }

    if list != Value::Nil {
        panic!("{} is not a list!", save)
    }

    sum
}

pub fn lmul(mut list: Value) -> Value {
    let mut res = Value::Number(1);

    let save = list.clone();

    while let Value::Pair(pair) = &list {
        res *= pair.0.clone();
        list = pair.1.clone();
    }

    if list != Value::Nil {
        panic!("{} is not a list!", save)
    }

    res
}

pub fn leq(list: Value) -> Value {
    let mut res = Value::Nil;

    let (a, b) = get_two(list.clone());

    if a == b {
        res = Value::Number(1);
    }

    res
}

pub fn lgt(list: Value) -> Value {
    let mut res = Value::Nil;

    let (Value::Number(a), Value::Number(b)) = get_two(list.clone()) else {
        panic!("{} is not list of 2 numbers!", list)
    };

    if a > b {
        res = Value::Number(1);
    }

    res
}

pub fn llt(list: Value) -> Value {
    let mut res = Value::Nil;

    let (Value::Number(a), Value::Number(b)) = get_two(list.clone()) else {
        panic!("{} is not list of 2 numbers!", list)
    };

    if a < b {
        res = Value::Number(1);
    }

    res
}

pub fn ldiv(list: Value) -> Value {
    let (Value::Number(a), Value::Number(b)) = get_two(list.clone()) else {
        panic!("{} is not list of 2 numbers!", list)
    };

    Value::Number(a / b)
}

pub fn lsub(list: Value) -> Value {
    let (Value::Number(a), Value::Number(b)) = get_two(list.clone()) else {
        panic!("{} is not list of 2 numbers!", list)
    };

    Value::Number(a - b)
}
