use crate::lang::*;
use crate::prelude::{parse, tokenize};

pub struct Runtime {
    scopes: Scopes,
}

impl Runtime {
    pub fn new(parent_path: String) -> Self {
        let mut scopes = Scopes::new();
        scopes.init_scope();

        scopes.change("__parent_path__".into(), Value::String(parent_path));

        scopes.init_scope();
        if let Value::Struct(std_lib) = crate::libs::lstd::init() {
            scopes.change_from(std_lib);
            scopes.init_scope();
        }

        Self { scopes }
    }

    pub fn run(&mut self, code: &str) {
        let tokens = tokenize(code);
        let values = match parse(tokens) {
            Ok(result) => result,
            Err(err) => {
                println!("{:?}", err);
                std::process::exit(1);
            }
        };

        for value in values {
            match eval(value.clone(), &mut self.scopes) {
                Value::Error(err) => {
                    println!(
                        "<runtime> run: catched Error while evaluating value: {}\n{}",
                        value, err
                    );
                    std::process::exit(1);
                }
                _ => continue,
            };
        }
    }

    pub fn get_public(&self) -> Scope {
        self.scopes.public.clone()
    }
}
