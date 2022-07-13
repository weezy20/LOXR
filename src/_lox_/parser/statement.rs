use super::*;
#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Box<Expression>),
    Print(Box<Expression>)
}
