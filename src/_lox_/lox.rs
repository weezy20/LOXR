#![allow(warnings, unused)]
use crate::_lox_::parser::Parser;

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
    /// Report `message` as error on `line`
    pub fn report_err(line: usize, col: usize, message: String) {
        eprintln!("Syntax Error: {message} at line {line}, column {col}");
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
        let mut parser = Parser::new(tokens);
        match parser.run() {
            Ok(std) => println!("Successfully parsed"),
            Err(_) => {
                self.had_error = true;
            }
        }
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
}
