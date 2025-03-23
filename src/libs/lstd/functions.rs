use super::{super::error, Scopes, Value, LIB_NAME};
use crate::{prelude::Runtime, utils};

use std::path::PathBuf;

const VALUE_TYPE: &str = "func";

pub fn llist(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    Value::Array(args)
}

pub fn limport(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    for arg in args {
        if let Value::String(module) = arg {
            let builtins = crate::libs::builtins();
            if builtins.contains_key(&module) {
                scopes.change(module.clone(), builtins.get(&module).unwrap().clone())
            } else {
                let parent_path = match scopes.get("__parent_path__".into()) {
                    Value::Error(err) => {
                        return Value::Error(format!("<func> import: catched error\n{}", err))
                    }
                    Value::String(path) => PathBuf::from(path),
                    _ => {
                        return Value::Error(
                            "<func> import: TypeError - '__parent_path__' should be String".into(),
                        )
                    }
                };

                let path = parent_path.join(&module);
                let file_path = parent_path.join(module.clone() + ".li");

                if path.exists() && path.is_dir() {
                    if path.join("mod.li").exists() {
                        // TODO
                    } else {
                        return Value::Error(format!(
                                "<func> import: FileNotFoundError - can't find mod.li inside directory ({})", path.to_str().unwrap()
                        ));
                    }
                } else if file_path.exists() {
                    let code = utils::code_from_file(&file_path);
                    let mut runtime =
                        Runtime::new(Some(file_path.parent().unwrap().to_str().unwrap().into()));
                    runtime.run(&code);
                    scopes.change(module, Value::Struct(runtime.get_public()))
                } else {
                    let module_as_path = PathBuf::from(&module);
                    if module_as_path.is_absolute() && module_as_path.exists() {
                        // TODO
                    } else {
                        return Value::Error(format!(
                            "<func> import: FileNotFoundError - given path ('{}' and '{}') of module not found", module_as_path.to_str().unwrap(), file_path.to_str().unwrap()
                        ));
                    }
                }
            }
        } else {
            return Value::Error("<func> import: TypeError - expected String type".to_string());
        }
    }

    Value::Nil
}

pub fn ladd(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "add";

    let mut res = 0;

    for arg in args {
        if let Value::Number(num) = arg {
            res += num;
        } else {
            return Value::Error(error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "TypeError",
                "expected Number type in arguments",
            ));
        }
    }

    Value::Number(res)
}

pub fn lmul(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    const FUNC_NAME: &str = "mul";

    let mut res = 1;

    for arg in args {
        if let Value::Number(num) = arg {
            res *= num;
        } else {
            return Value::Error(error::fmt(
                LIB_NAME,
                VALUE_TYPE,
                FUNC_NAME,
                "TypeError",
                "expected Number type in arguments",
            ));
        }
    }

    Value::Number(res)
}
