#![allow(unused_imports)]
use crate::loc;
use crate::parser::error::{EvalError, RuntimeError};
use crate::parser::expressions::{AssignmentExpr, Expression};
use crate::parser::{
    statement::{Declaration, Declaration::DStmt, Stmt},
    traits::evaluate::{Evaluate, ValueResult},
    value::Value,
    Parser,
};
use crate::tokenizer::token::Token;
use colored::Colorize;
use std::borrow::BorrowMut;
use std::rc::Rc;
mod environment;
pub use environment::Environment;
/// Since at this point our program is made of statements, this is perfectly fine
#[derive(Default, Debug)]
pub struct Interpreter {
    stmts: Vec<Declaration>,
    // TODO: Can be turned into Rc<Environment>
    env: Rc<Environment>,
    // Default false
    pub(crate) repl: bool,
    // index for repl mode
    previous: usize,
}

pub trait Memory {
    fn define(&mut self, name: &str, value: Value);
    fn get(&self, name: &Token) -> Result<&Value, RuntimeError>;
    fn put(&mut self, name: &str, value: Value) -> Result<(), RuntimeError>;
}

impl Interpreter {
    pub fn new(mut p: Parser) -> Self {
        Self {
            stmts: p.parse(),
            ..Default::default()
        }
    }
    /// Extend a repl interpreter and interpret the added stmts
    pub fn extend(&mut self, mut p: Parser) {
        assert!(
            self.repl,
            "ICE : Extend can only be called on repl mode, call interpret() instead"
        );
        self.previous = self.stmts.len();
        self.stmts.append(&mut p.parse());
        loc!(format!("Interpreter modified -> {self:?}"));
        self.interpret();
        // if self.is_repl_mode ? then for stmt in self.stmts[self.previous..].iter() { .. }
    }
    /// Execute a block of statements inside new environment
    fn execute_block(
        &self,
        statements: &Vec<Declaration>,
        sub_env: Rc<Environment>,
    ) -> ValueResult {
        // let sub_env = ;
        for stmt in statements.iter() {}
        Ok(Value::Nil)
    }
    pub fn interpret(&mut self) -> () {
        for stmt in self.stmts[self.previous..].iter() {
            let val: ValueResult = match stmt {
                DStmt(d) => match d {
                    Stmt::ExprStmt(e) | Stmt::Print(e) => {
                        e.eval(Rc::get_mut(&mut self.env).expect("ICE: Failed to obtain &mut Env"))
                    }
                    Stmt::ErrStmt { message } => {
                        loc!();
                        eprintln!(
                            "{}{}{message}",
                            "Interpreter Error: ".red(),
                            "Bad statement ".yellow()
                        );
                        Ok(Value::Nil)
                    }
                    Stmt::Empty => Ok(Value::Nil),
                    Stmt::Block(scoped_stmts) => self.execute_block(
                        scoped_stmts,
                        Rc::new(Environment::new(Rc::clone(&self.env))),
                    ),
                },
                // Declarations should produce no values
                Declaration::VarDecl { name, initializer } => {
                    // let init_err : Option<EvalError> = None;
                    let val = if let Some(expr) = initializer {
                        match expr.eval(
                            Rc::get_mut(&mut self.env).expect("ICE: Failed to obtain &mut Env"),
                        ) {
                            Ok(v) => v,
                            Err(eval_err) => {
                                loc!();
                                eprintln!("{} {eval_err}", "Interpreter Error:".red());
                                continue;
                            }
                        }
                    } else {
                        Value::Nil
                    };
                    println!("var {name} declared to {}", val);
                    Rc::get_mut(&mut self.env).expect("ICE: obtain &mut env").define(name, val);
                    crate::loc!(format!("{:?}", self.env.values));
                    Ok(Value::Nil)
                }
                Declaration::ErrDecl => {
                    loc!();
                    eprintln!(
                        "{}{}",
                        "Interpreter Error: ".red(),
                        "Variable declaration error".yellow()
                    );
                    Ok(Value::Nil)
                }
            };
            match val {
                Ok(val) => {
                    println!(">> {}", val);
                }
                Err(e) => {
                    loc!();
                    eprintln!("{} {e}", "Interpreter Error:".red());
                }
            };
        }
    }
}
