use crate::lang::leval;
use crate::lang::scopes::Scopes;
use crate::lang::value::*;
use crate::{nil, num};

pub fn lset(list: Value, scopes: &mut Scopes) -> Value {
    let v: Vec<Value> = list.into();
    if v.len() != 2 {
        panic!("Not 2 arguments!")
    }

    if let Value::Name(k) = &v[0] {
        let v = leval(v[1].clone(), scopes);
        scopes.change(k.to_string(), v)
    } else {
        panic!("1st arg isn't name type")
    }
    nil!()
}

pub fn llet(list: Value, scopes: &mut Scopes) -> Value {
    let args: Vec<Value> = list.into();
    if args.len() != 2 {
        panic!("Not 2 arguments!")
    }

    scopes.init_scope();

    let sets: Vec<Value> = args[0].clone().into();
    for i in 0..sets.len() {
        lset(sets[i].clone(), scopes);
    }

    let result = leval(args[1].clone(), scopes);

    scopes.pop();

    result
}

pub fn lcond(states: Value, scopes: &mut Scopes) -> Value {
    if states.is_list() {
        let states: Vec<Value> = states.into();
        for state in states.clone() {
            if state.len() == 2 {
                if leval(state.get(0).clone(), scopes) == num!(1) {
                    return leval(state.get(1).clone(), scopes);
                }
            } else {
                panic!("cond state error!")
            }
        }
    } else {
        panic!("cond syntax error!")
    }

    Value::Nil
}

pub fn llambda(list: Value, scopes: &mut Scopes) -> Value {
    use std::rc::Rc;
    let args: Vec<Value> = list.into();
    if args.len() != 2 {
        panic!("Lambda requires exactly 2 arguments: parameters and body!");
    }

    let params = args[0].clone();
    let body = args[1].clone();

    Value::Function(Rc::new(move |args: Value, eval_scopes: &mut Scopes| {
        eval_scopes.init_scope();
        let param_list: Vec<Value> = params.clone().into();
        let arg_list: Vec<Value> = args.into();

        if param_list.len() != arg_list.len() {
            panic!("Lambda called with wrong number of arguments!");
        }

        for i in 0..param_list.len() {
            if let Value::Name(param_name) = &param_list[i] {
                eval_scopes.change(param_name.clone(), arg_list[i].clone());
            } else {
                panic!("Lambda parameter must be a name!");
            }
        }

        let result = leval(body.clone(), eval_scopes);
        eval_scopes.pop();
        result
    }))
}
