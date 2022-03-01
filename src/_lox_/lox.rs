use super::tokenizer::scanner::Scanner;
#[derive(Debug)]
pub struct Lox {
    /// Error encountered?
    pub(crate) had_error: bool,
    pub src: String,
}

impl Lox {
    /// Start a Lox instance for files
    pub fn new(src: String) -> Self {
        Self {
            had_error: false,
            src,
        }
    }

    /// Scan a file, parse it into tokens and construct an AST using Lox grammer, then run it
    pub fn run_file(&mut self, file: &std::path::Path) {
        // todo! : refactor this code to check if lox instance already has a source
        let lox_file =
            std::fs::read_to_string(file).expect("Cannot open file path {file:?}");
        self.run(lox_file);
    }
    /// Parse a `lox` string as `lox` tokens and run them
    pub fn run(&mut self, src: String) {
        let mut scanner = Scanner::new(&src, self);
        scanner.scan_tokens();
        todo!(
            "
            let tokens = scanner.tokens;
            let parser = Parser::new(tokens);
            let ast = parser.gen_ast()
            ast.evaluate();
        "
        );
    }

    /// Report `message` as error on `line`
    pub fn report_err(line: usize, message: String) {
        eprintln!("Error: {message} at line {line}");
    }
}
