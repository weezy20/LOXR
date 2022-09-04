#![feature(let_chains)]
#![feature(box_syntax)]
#![forbid(unsafe_code)]
//! This module contains all definitions for the Lox interpreter
//! # Lox grammer: 
//! *program*          → `declaration`* EOF;
//! 
//! *declaration*      → `variableDecl` | statement;
//! 
//! *variableDecl*     → `"var" IDENTIFIER ("=" expression)? ";"` ;
//! 
//! *statement*        → `exprStmt` | `printStmt` | `block` | `ifStmt` ;
//! 
//! *exprStmt*         → `expression` ";" ;
//! 
//! *printStmt*        → print `expression` ";" ;
//! 
//! *block*            → `"{" (declaration)* "}"` ;
//! 
//! *ifStmt*           → `"if" "(" expression ")"  statement ("else" statement)?` ;
//! 
//! A comma expression evaluates to the final expression
//! 
//! *comma expr*     → `expression , (expression)* | "(" expression ")"`;
//!
//! *ternary*        → `expression` ? `expression` : `expression`;
//!
//! *expression*     → `assignment
//!                   | literal
//!                   | unary
//!                   | binary
//!                   | grouping ;`
//!
//! *assignment*  → `ternary` | IDENTIFIER "=" `assignment`
//! 
//! *ternary*     → `logic_or` | `logic_or` ? : `logic_or`;
//! 
//! *logic_or*    → `logic_and` ( "or" `logic_and`)* ;
//! 
//! *logic_and*   → `equality` ("and" `equality`)* ; 
//!
//! *equality*    → `comparsion ("==" | "!=" comparison)*;`
//!
//! *comparison*  → `term ("<="|"<"|">"|">=" term)*;`
//!
//! *term*        → `factor ("+"|"-" factor)*;`
//!
//! *factor*      → `unary (( "%" | "/" | "*" ) unary )*;`
//!
//! *unary*       → `("-" | "!") unary | primary;`
//!
//! *primary*     → `literal | identifier | "(" expression ")";`

//! *literal*        → `NUMBER | STRING | "true" | "false" | "nil" ;`
//!
//! *grouping*       → `"(" expression ")" ;`
//!
//! *unary*          → `( "-" | "!" ) expression ;`
//!
//! *binary*         → `expression operator expression ;`
//!
//! *operator*       → `"==" | "!=" | "<" | "<=" | ">" | ">="
//!                  | "+"  | "-"  | "*" | "/" | "%";`

mod tests;

/// ## A module for token definitions, and a lox lexer and scanner
pub mod tokenizer;

/// ## Parser module that defines Lox syntactical grammar and constructs ASTs
pub mod parser;

/// ## Interpreter
pub mod interpreter;

/// ## Macros
pub mod macros;

// use std::rc::Rc;

use crate::parser::Parser;
use crate::tokenizer::scanner::Scanner;
use colored::Colorize;
use interpreter::Interpreter;
use tokenizer::token::Token;
#[derive(Debug)]
pub struct Lox {
    /// Error encountered?
    pub had_error: bool,
    pub had_runtime_error: bool,
    /// Source string
    pub src: String,
    /// Repl interpreter
    pub repl_interpreter: Interpreter,
}

impl Lox {
    /// Start a Lox instance for files
    pub fn new(src: String) -> Self {
        Self {
            repl_interpreter: Interpreter::default(),
            had_error: false,
            had_runtime_error: false,
            src,
        }
    }
    pub fn print_all_tokens(tokens: Vec<Token>) {
        tokens
            .iter()
            .map(|t| t.to_string())
            .for_each(|tr| print!("{tr} "));
        println!("");
    }
    /// Report `message` as error on `line`
    pub fn report_syntax_err(line: usize, col: usize, message: String) {
        eprintln!(
            "{syntax_error}: {message} at {line_no}, {col_no}",
            syntax_error = "Syntax Error".red(),
            line_no = format!("line {line}").yellow(),
            col_no = format!("column {col}").yellow()
        );
    }
    /// Handler for errors that are thrown by the interpreter
    pub fn report_runtime_err(message: String) {
        eprintln!(
            "{runtime_error}: {message}",
            runtime_error = "Runtime Error".bright_red(),
            // line_no = format!("line {line}").yellow(),
            // col_no = format!("column {col}").yellow()
        );
        // if !self.had_runtime_error {
        //     self.had_runtime_error = true;
        // }
    }
    pub fn run(&mut self, line: Option<String>) {
        if let Some(src) = line {
            // Interpret
            self.run_line(src);
        } else {
            // Run file
            let src = self.src.clone();
            let mut scanner = Scanner::new(&src, self);
            scanner.scan_tokens();
            let tokens = scanner.tokens;
            let parser = Parser::new(tokens);
            let mut interpreter = Interpreter::new(parser);
            interpreter.interpret();
        }
    }
    /// A REPL function. Interpret `src` as `lox` source and run it
    pub fn run_line(&mut self, src: String) {
        let mut scanner = Scanner::new(&src, self);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        let parser = Parser::new(tokens);
        // let parser = parser.clone();
        self.repl_interpreter.repl = true;
        self.repl_interpreter.extend(parser);
        // let mut interpreter = Interpreter::new_parser(interpreter, parser);
        // self.repl_interpreter.interpret(); // This will run the entire interpreter 
        return;
    }
}
