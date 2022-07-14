use colored::Colorize;

use crate::parser::{
    statement::Stmt,
    traits::{evaluate::Evaluate, printer::ExpressionPrinter},
    Parser,
};

/// Since at this point our program is made of statements, this is perfectly fine
pub struct Interpreter(Vec<Stmt>);
impl Interpreter {
    pub fn new(mut p: Parser) -> Self {
        Self(p.parse())
    }
    pub fn interpret(&mut self) {
        for stmt in self.0.iter() {
            let val = match match stmt {
                Stmt::ExprStmt(e) | Stmt::Print(e) => e.eval(),
            } {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("{} {e}", "Interpreter error:".red());
                    continue;
                }
            };

            match stmt {
                Stmt::ExprStmt(e) => {
                    // let _ = e.eval();
                }
                Stmt::Print(e) => {
                    println!("{}", val);
                }
            }
        }
    }
}
