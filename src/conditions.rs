use super::operators::leval;
use super::value::*;
use crate::num;

pub fn lcond(states: Value, scopes: &mut super::scopes::Scopes) -> Value {
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
