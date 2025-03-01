pub mod scopes;
pub mod value;

pub fn leval(expr: value::Value, scopes: &mut scopes::Scopes) -> value::Value {
    use crate::pair;
    use value::List;

    match expr {
        value::Value::Nil => expr,
        value::Value::Number(_) => expr,
        value::Value::Macros(_) => expr,
        value::Value::Function(_) => expr,
        value::Value::Name(name) => scopes.get(name),
        value::Value::Pair(action, args) => match leval((*action).clone(), scopes) {
            value::Value::Macros(macros) => macros((*args).clone(), scopes),
            value::Value::Function(function) => {
                if !(*args).is_list() {
                    panic!("This is not list: {}", args)
                } else {
                    let args_vec: Vec<value::Value> = (*args).clone().into();
                    let evaluated_args = value::Value::from(
                        args_vec
                            .into_iter()
                            .map(|x| leval(x, scopes))
                            .collect::<Vec<value::Value>>(),
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
