use super::value::*;

pub fn ladd(list: Value) -> Value {
    let mut sum = 0;
    let v: Vec<Value> = list.into();

    for i in v {
        if let Value::Number(n) = i {
            sum += n;
        } else {
            panic!("Non-number element in list: {}", i);
        }
    }

    Value::Number(sum)
}

pub fn lmul(list: Value) -> Value {
    let mut res = 1;
    let v: Vec<Value> = list.into();

    for i in v {
        if let Value::Number(n) = i {
            res *= n;
        } else {
            panic!("Non-number element in list: {}", i);
        }
    }

    Value::Number(res)
}

pub fn leq(list: Value) -> Value {
    let v: Vec<Value> = list.into();
    if v.len() != 2 {
        panic!("Too many arguments!");
    };

    if v[0] == v[1] {
        Value::Number(1)
    } else {
        Value::Nil
    }
}

pub fn lgt(list: Value) -> Value {
    let v: Vec<Value> = list.into();
    if v.len() != 2 {
        panic!("Too many arguments!");
    };

    if let (Value::Number(a_num), Value::Number(b_num)) = (&v[0], &v[1]) {
        if a_num > b_num {
            Value::Number(1)
        } else {
            Value::Nil
        }
    } else {
        panic!("Arguments must be numbers: {} and {}", v[0], v[1])
    }
}

pub fn llt(list: Value) -> Value {
    let v: Vec<Value> = list.into();
    if v.len() != 2 {
        panic!("Too many arguments!");
    };

    if let (Value::Number(a_num), Value::Number(b_num)) = (&v[0], &v[1]) {
        if a_num < b_num {
            Value::Number(1)
        } else {
            Value::Nil
        }
    } else {
        panic!("Arguments must be numbers: {} and {}", v[0], v[1])
    }
}

pub fn ldiv(list: Value) -> Value {
    let v: Vec<Value> = list.into();
    if v.len() != 2 {
        panic!("Too many arguments!");
    };

    if let (Value::Number(a_num), Value::Number(b_num)) = (v[0].clone(), v[1].clone()) {
        if b_num == 0 {
            panic!("Division by zero");
        }
        Value::Number(a_num / b_num)
    } else {
        panic!("Arguments must be numbers")
    }
}

pub fn lsub(list: Value) -> Value {
    let v: Vec<Value> = list.into();
    if v.len() != 2 {
        panic!("Too many arguments!");
    };

    if let (Value::Number(a_num), Value::Number(b_num)) = (v[0].clone(), v[1].clone()) {
        Value::Number(a_num - b_num)
    } else {
        panic!("Arguments must be numbers")
    }
}
