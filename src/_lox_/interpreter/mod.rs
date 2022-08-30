use crate::loc;
use crate::parser::{
    statement::{Declaration, Declaration::DStmt, Stmt},
    traits::evaluate::{Evaluate, ValueResult},
    value::Value,
    Parser,
};
use colored::Colorize;

/// Since at this point our program is made of statements, this is perfectly fine
pub struct Interpreter(Vec<Declaration>);
impl Interpreter {
    pub fn new(mut p: Parser) -> Self {
        Self(p.parse())
    }
    pub fn interpret(&mut self) -> () {
        for stmt in self.0.iter() {
            let val: ValueResult = match stmt {
                DStmt(s) => match s {
                    Stmt::ExprStmt(e) | Stmt::Print(e) => e.eval(),
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
                    println!(
                        "var {name} declared to {}",
                        if let Some(expr) = initializer {
                            expr.eval().expect("Unsafe unwrap of ValueResult")
                        } else {
                            Value::Nil
                        }
                    );
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
