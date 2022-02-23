use super::tokenizer::scanner::Scanner;
#[derive(Default)]
pub struct Lox {
    /// Error encountered?
    had_error: bool,
    /// Token scanner
    scanner : Scanner,
}

impl Lox {
    /// Start a Lox instance for files
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Scan a file, parse it into tokens and construct an AST using Lox grammer, then run it
    pub fn run_file(&mut self, file: &std::path::Path) {
        let lox_file =
            std::fs::read_to_string(file).expect("Cannot open file path {file:?}");
        self.run(lox_file);
    }
    /// Parse a `lox` string as `lox` tokens and run them
    pub fn run(&mut self, lox: String) {
        let tokens = self.scanner.scan_tokens(lox);
        println!("Scanned \n{tokens:?}");
    }

    /// Report `message` as error on `line`
    pub fn report_err(line: usize, message: String) {
        eprintln!("Error: {message} at line {line}");
    }
}
