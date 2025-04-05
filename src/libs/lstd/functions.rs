use super::{super::error, Scopes, Value, LIB_NAME};

const VALUE_TYPE: &str = "func";

pub fn levals(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    use crate::prelude::{eval, parse, tokenize};

    const FUNC_NAME: &str = "evals";

    for arg in args {
        match arg {
            Value::String(code) => {
                let tokens = tokenize(&code);
                let values = match parse(tokens) {
                    Ok(result) => result,
                    Err(err) => {
                        println!("{:?}", err);
                        std::process::exit(1);
                    }
                };

                for (index, value) in values.iter().enumerate() {
                    match eval(value.clone(), scopes) {
                        Value::Error(err) => {
                            println!(
                                "[..] <runtime> run: catched Error while evaluating value: {}\n{}",
                                value, err
                            );
                            std::process::exit(1);
                        }
                        result => {
                            if index == values.len() - 1 {
                                return result;
                            } else {
                                continue;
                            }
                        }
                    };
                }
            }
            _ => {
                return Value::Error(error::fmt(
                    LIB_NAME,
                    VALUE_TYPE,
                    FUNC_NAME,
                    "TypeError",
                    "expected String type",
                ))
            }
        }
    }
    Value::Nil
}

pub fn llist(args: Vec<Value>, _scopes: &mut Scopes) -> Value {
    Value::Array(args)
}

pub fn limport(args: Vec<Value>, scopes: &mut Scopes) -> Value {
    use crate::{prelude::Runtime, utils};
    use dirs::cache_dir;
    use reqwest::blocking::get;
    use std::path::{Path, PathBuf};

    fn download_github(import_path: &str) -> Result<PathBuf, String> {
        // Split off branch specification
        let (path_part, branch) = import_path
            .rsplit_once('@')
            .unwrap_or((import_path, "main"));

        // Verify and parse GitHub path
        let path_part = path_part
            .strip_prefix("github:")
            .ok_or("GitHub imports must start with 'github:'")?;

        let mut components = path_part.split('/');
        let user = components.next().ok_or("Missing user in GitHub path")?;
        let repo = components.next().ok_or("Missing repo in GitHub path")?;
        let rest_path = components.collect::<Vec<_>>().join("/");

        // Determine if we're importing a directory or file
        let is_file = rest_path.ends_with(".li");
        let github_path = if is_file {
            rest_path.clone()
        } else {
            format!("{}/mod.li", rest_path)
        };

        // Create cache directory structure
        let cache_base = cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("li-libs")
            .join("github")
            .join(user)
            .join(repo)
            .join(format!("@{}", branch));

        let cache_path = cache_base.join(&rest_path);
        std::fs::create_dir_all(&cache_path)
            .map_err(|e| format!("Failed to create cache dir: {}", e))?;

        // Download target file
        let target_file = if is_file {
            Path::new(&rest_path)
                .file_name()
                .ok_or("Invalid file path")?
                .to_str()
                .unwrap()
        } else {
            "mod.li"
        };

        let target_path = cache_path.join(target_file);

        if !target_path.exists() {
            let url = format!(
                "http://raw.githubusercontent.com/{}/{}/{}/{}",
                user, repo, branch, github_path
            );

            let content = get(&url)
                .map_err(|e| format!("Failed to download {}: {}", url, e))?
                .bytes()
                .map_err(|e| format!("Failed to read response: {}", e))?;

            std::fs::write(&target_path, content)
                .map_err(|e| format!("Failed to write cache file: {}", e))?;
        }

        Ok(if is_file { target_path } else { cache_path })
    }

    for arg in args {
        if let Value::String(module) = arg {
            let builtins = crate::libs::builtins();
            if builtins.contains_key(&module) {
                scopes.change(module.clone(), builtins.get(&module).unwrap().clone())
            } else if module.starts_with("github:") {
                match download_github(&module) {
                    Ok(path) => {
                        let mut runtime = Runtime::new(Some(
                            path.parent().unwrap().to_string_lossy().to_string(),
                        ));

                        if path.is_dir() {
                            let mod_path = path.join("mod.li");
                            if !mod_path.exists() {
                                return Value::Error(format!(
                                    "GitHub module '{}' missing mod.li",
                                    module
                                ));
                            }
                            runtime.run(&utils::code_from_file(&mod_path));
                        } else {
                            runtime.run(&utils::code_from_file(&path));
                        }

                        scopes.change(module.clone(), Value::Struct(runtime.get_public()));
                    }
                    Err(e) => return Value::Error(format!("GitHub import failed: {}", e)),
                }
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
