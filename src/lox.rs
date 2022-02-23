pub struct Lox {
    had_error: bool,
}

impl Lox {
    /// Start a Lox instance
    pub fn new() -> Self {
        let had_error = false;
        Self { had_error }
    }
    /// Start an interpreter
    pub fn init_interpreter() -> Self {
        let had_error = false;
        Self { had_error }
    }
    /// Interpret a line of text as Lox syntax, and make any state changes if pending
    pub fn interpret(&mut self, cmd: &str) {
        todo!();
    }
    /// Scan a file, parse it into tokens and construct an AST using Lox grammer, then run it
    pub fn run_file(&mut self, file: String) {
        todo!();
    }

    /// Report `message` as error on `line`
    pub fn report_err(line: usize, message: String) {
        eprintln!("Error: {message} at line {line}");
    }
}
