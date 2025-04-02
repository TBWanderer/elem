use super::{error, Scopes, Value};
use std::rc::Rc;

pub fn eval(value: Value, scopes: &mut Scopes) -> Value {
    match value {
        Value::Name(name) => scopes.get(name),
        Value::Pair(car, cdr) => match eval((*car).clone(), scopes) {
            Value::Macro(lmacro) => eval_macro(lmacro, (*cdr).clone().into(), scopes),
            Value::Function(lfunc) => eval_function(lfunc, (*cdr).clone().into(), scopes),
            Value::Struct(_) => eval_struct((*car).clone(), (*cdr).clone(), scopes),
            Value::Array(array) => eval_array(array, (*cdr).clone(), scopes),
            _ => Value::Error(format!(
                "[..] <sys> eval: SyntaxError - can't eval this list: {}\nScopes: {:?}",
                Value::Pair((*car).clone().into(), (*cdr).clone().into()),
                scopes
            )),
        },
        _ => value,
    }
}

fn eval_macro(
    lmacro: Rc<dyn Fn(Vec<Value>, &mut Scopes) -> Value>,
    args: Value,
    scopes: &mut Scopes,
) -> Value {
    lmacro(args.clone().into(), scopes)
}

fn eval_function(
    lfunction: Rc<dyn Fn(Vec<Value>, &mut Scopes) -> Value>,
    args: Value,
    scopes: &mut Scopes,
) -> Value {
    if !args.is_list() {
        Value::Error(error::fmt(
            "..",
            "sys",
            "eval",
            "TypeError",
            &format!("this is not list: {}", args),
        ))
    } else {
        let args_vec: Vec<Value> = args.clone().into();
        let evaluated_args = args_vec
            .into_iter()
            .map(|x| eval(x, scopes))
            .collect::<Vec<Value>>();
        lfunction(evaluated_args, scopes)
    }
}

fn eval_struct(lstruct: Value, args: Value, scopes: &mut Scopes) -> Value {
    if let Value::Struct(structure) = eval(lstruct.clone(), scopes) {
        scopes.init_scope();
        scopes.change_from(structure.clone());
        match eval(args.clone(), scopes) {
            Value::Error(err) => Value::Error(error::fmt(
                "..",
                "sys",
                "eval",
                "CatchedError",
                &format!("catched error\n{}", err),
            )),
            any_other => any_other,
        }
    } else {
        Value::Nil
    }
}

fn eval_array(array: Vec<Value>, args: Value, scopes: &mut Scopes) -> Value {
    let args_vec: Vec<Value> = args.into();
    if args_vec.len() == 1 {
        match eval(args_vec[0].clone(), scopes) {
            Value::Number(idx) => {
                if idx < 0 || idx as usize >= array.len() {
                    Value::Error(error::fmt(
                        "..",
                        "sys",
                        "eval",
                        "IndexError",
                        &format!(
                            "index {} is out of bounds of array: {}",
                            idx,
                            Value::Array(array)
                        ),
                    ))
                } else {
                    array[idx as usize].clone()
                }
            }
            _ => Value::Error(
                "[..] <sys> eval: TypeError - array index must be a number".to_string(),
            ),
        }
    } else if args_vec.len() == 2 {
        match (
            eval(args_vec[0].clone(), scopes),
            eval(args_vec[1].clone(), scopes),
        ) {
            (Value::Number(start), Value::Number(end)) => {
                let start = start as usize;
                let end = end as usize;
                if start >= array.len() || end > array.len() || start > end {
                    Value::Error(format!("[..] <sys> eval: IndexError - invalid slice [{}:{}] for array of length {}", start, end, array.len()))
                } else {
                    Value::Array(array[start..end].to_vec())
                }
            }
            _ => Value::Error(
                "[..] <sys> eval: TypeError - array slice bounds must be numbers".to_string(),
            ),
        }
    } else {
        Value::Error(
            "[..] <sys> eval: SyntaxError - array access requires 1 or 2 arguments".to_string(),
        )
    }
}
