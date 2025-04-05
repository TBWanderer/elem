use clap::{value_parser, Arg, Command};
use elem_lit::prelude::Runtime;
use elem_lit::utils::*;
use std::path::PathBuf;

fn main() {
    let matches = Command::new("elem-lit")
        .version("0.2.1")
        .about("Interpreter for ELEM Lithium lang")
        .arg(Arg::new("file").value_parser(value_parser!(PathBuf)))
        .get_matches();

    match matches.get_one::<PathBuf>("file") {
        Some(path) => run_file(path.to_path_buf()),
        None => repl(),
    }
}

fn repl() {
    let mut runtime = Runtime::new(None);

    let repl_code = r#"
        (import "io")

        (set repl (lambda () (
            (set input (io read "repl@lit: "))
            (io print (evals input))
            (repl)
        )))

        (repl)
    "#;

    runtime.run(&repl_code);
}

fn run_file(path: PathBuf) {
    let parent_path = path
        .parent()
        .map(|p| p.to_str().unwrap_or("."))
        .unwrap_or(".")
        .to_string();
    let mut runtime = Runtime::new(Some(parent_path));

    let code = code_from_file(&path);
    runtime.run(&code);
}
