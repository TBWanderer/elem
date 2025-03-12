use clap::{arg, command};
use lithium::runtime;

fn main() {
    let matches = command!()
        .arg(arg!(-f --file <FILE>).required(false))
        .get_matches();

    let mut elem_runtime = runtime::Runtime::new();

    if let Some(path) = matches.get_one::<String>("file") {
        let pathbuf = std::path::PathBuf::from(path);
        if pathbuf.exists() && !pathbuf.is_file() {
            println!("Path is not file!");
            std::process::exit(1);
        }
        let pathbuf = std::path::PathBuf::from(std::fs::canonicalize(pathbuf).unwrap());

        elem_runtime.run(&format!(
            r#"(set __main_dir_path__ "{}")"#,
            pathbuf.parent().unwrap().to_str().unwrap_or("/")
        ));

        let code = runtime::utils::io::read_file(&path);
        elem_runtime.run(&code);
    } else {
        loop {
            let line = runtime::utils::io::input("lithium@elem >> ");
            elem_runtime.run(&line);
        }
    }
}
