fn main() {
    let input = r#"(import "io") (io print 123)"#;
    let cur_path = std::path::PathBuf::from(".");
    let abs_path = std::fs::canonicalize(cur_path).unwrap();
    let mut runtime = elem_lit::prelude::Runtime::new(abs_path.to_string_lossy().to_string());
    runtime.run(input)
}
