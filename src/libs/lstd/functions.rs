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
        panic!("Incorrect count of args for load func");
    } else if let Value::String(module_name) = args.get(0) {
        let builtins = crate::libs::builtins();
        if builtins.contains_key(module_name) {
            let mut namespaced_scope = std::collections::HashMap::new();
            let builtin_scope = builtins.get(module_name).unwrap();
            for (key, value) in builtin_scope.iter() {
                let namespaced_key = format!("{}::{}", module_name, key);
                namespaced_scope.insert(namespaced_key, value.clone());
            }
            scopes.add_scope(namespaced_scope);
        } else {
            let module_path = if module_name.ends_with(".li") {
                module_name.clone()
            } else {
                format!("{}.li", module_name)
            };

            let file_path = if scopes.exists("__main_dir_path__".to_string()) {
                if let Value::String(main_dir_path) = scopes.get("__main_dir_path__".to_string()) {
                    let full_path = std::path::PathBuf::from(&main_dir_path).join(&module_path);
                    if full_path.exists() && full_path.is_file() {
                        full_path
                    } else {
                        let alt_path = std::path::PathBuf::from(&main_dir_path).join(module_name);
                        if alt_path.exists() && alt_path.is_file() {
                            alt_path
                        } else {
                            std::path::PathBuf::from(&module_path)
                        }
                    }
                } else {
                    std::path::PathBuf::from(&module_path)
                }
            } else {
                std::path::PathBuf::from(&module_path)
            };

            if file_path.exists() && file_path.is_file() {
                let module_name = file_path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unnamed_module".to_string());

                let module_code = crate::runtime::utils::io::read_file(&file_path);

                if module_code.trim().is_empty() {
                    println!("Module {} is empty, skipping", module_name);
                    return crate::nil!();
                }

                let module_dir = if let Some(parent) = file_path.parent() {
                    parent.to_string_lossy().to_string()
                } else {
                    ".".to_string()
                };

                let mut module_scopes = Scopes::new();
                module_scopes.change("__main_dir_path__".to_string(), Value::String(module_dir));
                module_scopes.add_scope(crate::libs::lstd::init());
                module_scopes.init_scope();

                crate::runtime::utils::run_code(module_code, &mut module_scopes);

                if module_scopes.inner.is_empty() {
                    println!("Warning: Module {} has empty scope", module_name);
                    return crate::nil!();
                }

                let module_global_scope = module_scopes.inner.last_mut().unwrap().clone();
                let mut namespaced_scope = std::collections::HashMap::new();

                for (key, value) in module_global_scope.iter() {
                    if !key.starts_with("__") || !key.ends_with("__") {
                        let namespaced_key = format!("{}::{}", module_name, key);
                        namespaced_scope.insert(namespaced_key, value.clone());
                    }
                }

                scopes.add_scope(namespaced_scope.clone());
                println!("{:?}", namespaced_scope.clone())
            } else {
                panic!("File not found: {:?}", file_path);
            }
        }
    } else {
        return Value::Error(
            "<func> load: Incorrect type of argument! It should be String".to_string(),
        );
    }
    crate::nil!()
}
