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
            match stmt {
                Stmt::ExprStmt(e) => {
                    let _ = e.eval();
                }
                Stmt::Print(e) => {
                    let val = e.eval();
                    println!("{}", val.expect("DANGER UNWRAP of STATEMENT"));
                }
            }
        }
    }
}
