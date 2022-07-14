#![allow(warnings, unused)]
#![feature(let_chains)]
//! This module contains all definitions for the Lox compiler and Lox interpreter
mod tests;

/// ## A module for token definitions, and a lox lexer and scanner
pub mod tokenizer;

/// ## Parser module that defines Lox syntactical grammar and constructs ASTs
pub mod parser;

// use std::borrow::{Cow, Borrow};

use crate::parser::traits::evaluate::Evaluate;
use crate::parser::Parser;
use crate::tokenizer::scanner::Scanner;
use colored::Colorize;
use tokenizer::token::Token;
#[derive(Debug)]
pub struct Lox {
    /// Error encountered?
    pub had_error: bool,
    pub had_runtime_error: bool,
    /// Source string
    pub src: String,
}

impl Lox {
    /// Start a Lox instance for files
    pub fn new(src: String) -> Self {
        Self {
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
    pub fn report_runtime_err(&mut self, line: usize, col: usize, message: String) {
        eprintln!(
            "{runtime_error}: {message} at {line_no}, {col_no}",
            runtime_error = "Runtime Error".bright_red(),
            line_no = format!("line {line}").yellow(),
            col_no = format!("column {col}").yellow()
        );
        if !self.had_runtime_error {
            self.had_runtime_error = true;
        }
    }
    pub fn run(&mut self, line: Option<String>) {
        if let Some(src) = line {
            // Interpret
            self.run_line(src);
        } else {
            // Run file
            let src = self.src.clone(); // TODO: optimize this away
            let mut scanner = Scanner::new(&src, self);
            scanner.scan_tokens();
            let tokens = scanner.tokens;
            let mut parser = Parser::new(tokens);
            // TODO: run the interpreter
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
                    self.had_runtime_error = true;
                }
            }
        }
    }
    /// A REPL function. Interpret `src` as `lox` source and run it
    pub fn run_line(&mut self, src: String) {
        let mut scanner = Scanner::new(&src, self);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens);
        {
            let mut parser = parser.clone();
            // Print statements
            println!("Statements: ");
            for s in parser.parse() {
                println!("-> {s}");
            }
        }
        // match parser.run() {
        //     Ok(exp) => match exp.eval() {
        //         // Ok(result) => println!("{result}"),
        //         Ok(result) => {
        //             println!("{exp}");
        //             println!("{}", result);
        //             // We don't care about runtime errors in REPL mode, yet interesting to note, once a user does
        //             // enter faulty code, self.had_runtime_error stays on. 
        //             // println!("had error {}", self.had_error);
        //             // println!("had runtime error {}", self.had_runtime_error);
        //         }
        //         Err(e) => eprintln!("{e}"),
        //     }, /* println!("Successfully parsed: {std:#?}"), */
        //     Err(e) => {
        //         eprintln!("{e}");
        //         self.had_runtime_error = true;
        //     }
        // }
        return;
    }
}
