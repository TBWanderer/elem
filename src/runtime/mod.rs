pub mod utils;

pub struct Runtime {
    scopes: crate::lang::scopes::Scopes,
}

impl Runtime {
    pub fn new() -> Self {
        use crate::lang::scopes::Scopes;
        use crate::libs;

        let mut scopes = Scopes::new();
        scopes.add_scope(libs::lstd::init());
        scopes.add_scope(libs::lio::init());

        Self { scopes }
    }

    pub fn run(&mut self, line: &str) {
        utils::run_code(line.to_string(), &mut self.scopes);
    }
}
