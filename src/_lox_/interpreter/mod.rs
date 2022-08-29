use crate::parser::{
    statement::{
        Declaration,
        Declaration::{DStmt, VarDecl},
        Stmt,
    },
    traits::{evaluate::Evaluate, printer::ExpressionPrinter},
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
    pub fn interpret(&mut self) {
        for stmt in self.0.iter() {
            // Todo: declutter this code
            let val = match match stmt {
                DStmt(s) => match s {
                    Stmt::ExprStmt(e) | Stmt::Print(e) => e.eval(),
                    Stmt::ErrStmt { message } => {
                        eprintln!(
                            "{}{}{message}",
                            "Interpreter Error: ".red(),
                            "Bad statement ".yellow()
                        );
                        Ok(Value::Nil)
                    }
                    Stmt::Empty => Ok(Value::Nil),
                },
                VarDecl { name, initializer } => {
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
                    eprintln!("ErrDecl");
                    Ok(Value::Nil)
                }
            } {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("{} {e}", "Interpreter error:".red());
                    continue;
                }
            };
            println!("computed -> {}", val);
        }
    }
}
