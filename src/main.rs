use clap::{arg, command};
use elem::runtime;

fn main() {
    let matches = command!()
        .arg(arg!(-f --file <FILE>).required(false))
        .get_matches();

    let mut elem_runtime = runtime::Runtime::new();

    if let Some(path) = matches.get_one::<String>("file") {
        let code = runtime::utils::io::read_file(&path);
        elem_runtime.run(&code);
    } else {
        loop {
            let line = runtime::utils::io::input("lithium@elem >> ");
            elem_runtime.run(&line);
        }
    }
}
