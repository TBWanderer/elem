use crate::lang::{scopes::Scopes, value::*};

pub fn ladd(args: Value, _scopes: &mut Scopes) -> Value {
    let mut sum = 0;
    let v: Vec<Value> = args.into();

    for i in v {
        if let Value::Number(n) = i {
            sum += n;
        } else {
            panic!("Non-number element in args: {}", i);
        }
    }

    Value::Number(sum)
}

pub fn lmul(args: Value, _scopes: &mut Scopes) -> Value {
    let mut res = 1;
    let v: Vec<Value> = args.into();

    for i in v {
        if let Value::Number(n) = i {
            res *= n;
        } else {
            panic!("Non-number element in args: {}", i);
        }
    }

    Value::Number(res)
}

pub fn leq(args: Value, _scopes: &mut Scopes) -> Value {
    let v: Vec<Value> = args.into();
    if v.len() != 2 {
        panic!("Too many arguments!");
    };

    if v[0] == v[1] {
        Value::Number(1)
    } else {
        Value::Nil
    }
}

pub fn lgt(args: Value, _scopes: &mut Scopes) -> Value {
    let v: Vec<Value> = args.into();
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

pub fn llt(args: Value, _scopes: &mut Scopes) -> Value {
    let v: Vec<Value> = args.into();
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

pub fn ldiv(args: Value, _scopes: &mut Scopes) -> Value {
    let v: Vec<Value> = args.into();
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

pub fn lsub(args: Value, _scopes: &mut Scopes) -> Value {
    let v: Vec<Value> = args.into();
    if v.len() != 2 {
        panic!("Too many arguments!");
    };

    if let (Value::Number(a_num), Value::Number(b_num)) = (v[0].clone(), v[1].clone()) {
        Value::Number(a_num - b_num)
    } else {
        panic!("Arguments must be numbers")
    }
}

pub fn lload(args: Value, scopes: &mut Scopes) -> Value {
    if args.len() != 1 {
        panic!("Incorrect count of args for load func")
    } else if let Value::String(module_path) = args.get(0) {
        let module_code = crate::runtime::utils::io::read_file(module_path);
        scopes.init_scope();
        crate::runtime::utils::run_code(module_code, scopes);
    } else {
        panic!("Incorrect type of argument! It should be String")
    }

    crate::nil!()
}
