#![allow(warnings, unused)]
use super::tokenizer::scanner::Scanner;
#[derive(Debug)]
pub struct Lox {
    /// Error encountered?
    pub(crate) had_error: bool,
    /// Source string
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

    /// Parse a `lox` string as `lox` tokens and run them
    pub fn run(&mut self, src: String) {
        let mut scanner = Scanner::new(&src, self);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        tokens
            .iter()
            .map(|t| t.to_string())
            .for_each(|tr| println!("{tr}"));
        return;
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
    pub fn report_err(line: usize, message: String, col: usize) {
        eprintln!("Syntax Error: {message} at line {line}, column {col}");
    }
}
