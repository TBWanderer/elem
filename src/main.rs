use elem::runtime;

fn main() {
    let mut elem_runtime = runtime::Runtime::new();

    loop {
        let line = runtime::utils::input("lithium@elem >> ");
        elem_runtime.run(&line);
    }
}
