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
mod environment;
pub use environment::Environment;
/// Since at this point our program is made of statements, this is perfectly fine
#[derive(Default, Debug)]
pub struct Interpreter {
    stmts: Vec<Declaration>,
    // TODO: Can be made generic over environment
    env: Environment,
}

pub trait Memory {
    fn define(&mut self, name: &str, value: Value);
    fn get(&self, name: &Token) -> Result<Option<&Value>, RuntimeError>;
    fn put(&mut self, name: &str, value: Value) -> Result<(), RuntimeError>;
}

impl Interpreter {
    pub fn new(mut p: Parser) -> Self {
        Self {
            stmts: p.parse(),
            ..Default::default()
        }
    }
    /// Add a Stmt to a repl interpreter
    pub fn new_parser(&mut self, mut p: Parser) {
        self.stmts.append(&mut p.parse());
        loc!(format!("Interpreter modified -> {self:?}"));
    }
    pub fn interpret(&mut self) -> () {
        for stmt in self.stmts.iter() {
            let val: ValueResult = match stmt {
                DStmt(dstmt) => match dstmt {
                    Stmt::ExprStmt(assignment)
                        if matches!(**assignment, Expression::Assignment(_)) =>
                    {
                        loc!("Assignment interpreter block entered");
                        let assignment_expr = &**assignment;
                        if let Expression::Assignment(AssignmentExpr { name, right }) =
                            assignment_expr
                        {
                            match right.eval(&mut self.env) {
                                Ok(rval) => {
                                    self.env.put(&name.lexeme, rval.clone());
                                    Ok(rval)
                                }
                                Err(err) => Err(err),
                            }
                        } else {
                            Err(EvalError::InvalidExpr(
                                assignment_expr.to_owned(),
                                Some("Cannot perform this assignment".into()),
                            ))
                        }
                    }
                    Stmt::ExprStmt(e) | Stmt::Print(e) => e.eval(&mut self.env),
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
                },
                Declaration::VarDecl { name, initializer } => {
                    let val = if let Some(expr) = initializer {
                        expr.eval(&mut self.env)
                            .expect("Unsafe unwrap of ValueResult") // TODO: Deal with this unwrap
                    } else {
                        Value::Nil
                    };
                    println!("var {name} declared to {}", val);
                    self.env.define(name, val);
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
                    continue;
                }
            };
        }
    }
}
