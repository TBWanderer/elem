use super::value::*;
use crate::num;

pub fn leval(expr: Value) -> Value {
    expr
}

pub fn lcond(states: Value) -> Value {
    if states.is_list() && states.len() > 0 && states.get(0).is_list() && states.get(1).is_list() {
        let states: Vec<Value> = states.into();
        for state in states.clone() {
            if leval(state.get(0).clone()) == num!(1) {
                return leval(state.get(1).clone());
            }
        }
    } else {
        panic!("cond syntax error!")
    }

    Value::Nil
}
