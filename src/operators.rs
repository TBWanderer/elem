use super::scopes::Scopes;
use super::value::*;
use crate::nil;

pub fn ladd(list: Value, _scopes: &mut Scopes) -> Value {
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

pub fn lmul(list: Value, _scopes: &mut Scopes) -> Value {
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

pub fn leq(list: Value, _scopes: &mut Scopes) -> Value {
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

pub fn lgt(list: Value, _scopes: &mut Scopes) -> Value {
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

pub fn llt(list: Value, _scopes: &mut Scopes) -> Value {
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

pub fn ldiv(list: Value, _scopes: &mut Scopes) -> Value {
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

pub fn lsub(list: Value, _scopes: &mut Scopes) -> Value {
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

pub fn lset(list: Value, scopes: &mut Scopes) -> Value {
    use super::scopes;

    let v: Vec<Value> = list.into();
    if v.len() != 2 {
        panic!("Not 2 arguments!")
    }

    if let Value::Name(k) = &v[0] {
        scopes::change(scopes.to_vec(), k.to_string(), leval(v[1].clone(), scopes))
    } else {
        panic!("1st arg isn't name type")
    }
    nil!()
}

pub fn llet(list: Value, scopes: &mut Scopes) -> Value {
    use super::scopes;

    let args: Vec<Value> = list.into();
    if args.len() != 2 {
        panic!("Not 2 arguments!")
    }

    scopes::new(scopes);

    let sets: Vec<Value> = args[0].clone().into();
    for i in 0..sets.len() {
        lset(sets[i].clone(), scopes);
    }

    let result = leval(args[1].clone(), scopes);

    scopes::pop(scopes.to_vec());

    result
}

pub fn leval(expr: Value, scopes: &mut Scopes) -> Value {
    use super::scopes;
    use crate::pair;
    match expr {
        Value::Nil => expr,
        Value::Number(_) => expr,
        Value::Macros(_) => expr,
        Value::Function(_) => expr,
        Value::Name(name) => scopes::get(scopes.to_vec(), name),
        Value::Pair(action, args) => match leval((*action).clone(), scopes) {
            Value::Macros(macros) => macros((*args).clone(), scopes),
            Value::Function(function) => {
                if !args.is_list() {
                    panic!("This is not list: {}", args)
                } else {
                    let args_vec: Vec<Value> = (*args).clone().into();
                    let evaluated_args = Value::from(
                        args_vec
                            .into_iter()
                            .map(|x| leval(x, scopes))
                            .collect::<Vec<Value>>(),
                    );
                    function(evaluated_args, scopes)
                }
            }
            _ => panic!(
                "Can't eval this list: {}",
                pair!((*action).clone(), (*args).clone())
            ),
        },
    }
}
