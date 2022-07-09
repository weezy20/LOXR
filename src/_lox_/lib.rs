#![allow(warnings, unused)]
#![feature(let_chains)]
//! This module contains all definitions for the Lox compiler and Lox interpreter
mod tests;

/// ## A module for token definitions, and a lox lexer and scanner
pub mod tokenizer;

/// ## Parser module that defines Lox syntactical grammar and constructs ASTs
pub mod parser;

/// ## Interpreter for Parser output
pub mod interpreter;

use crate::parser::traits::evaluate::Evaluate;
use crate::parser::Parser;
use crate::tokenizer::scanner::Scanner;
use colored::Colorize;
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
        eprintln!(
            "{syntax_error}: {message} at {line_no}, {col_no}",
            syntax_error = "Syntax Error".red(),
            line_no = format!("line {line}").yellow(),
            col_no = format!("column {col}").yellow()
        );
    }
    /// Parse a `lox` string as `lox` tokens and run them
    pub fn run(&mut self, src: String) {
        let mut scanner = Scanner::new(&src, self);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        // tokens
        //     .iter()
        //     .map(|t| t.to_string())
        //     .for_each(|tr| println!("{tr}"));
        let mut parser = Parser::new(tokens);
        match parser.run() {
            Ok(exp) => match exp.eval() {
                // Ok(result) => println!("{result}"),
                Ok(result) => {
                    println!("{exp}");
                    println!("{}", result);
                }
                Err(e) => eprintln!("{e}"),
            }, /* println!("Successfully parsed: {std:#?}"), */
            Err(e) => {
                eprintln!("{e}");
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
