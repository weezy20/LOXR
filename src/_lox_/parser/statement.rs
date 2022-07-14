use super::*;
#[derive(Debug, derive_more::Display)]
/// A statement is always followed by a `;`.
/// A lox program is made up of lox statements
#[display(fmt = "{}", )]
pub enum Stmt {
    /// An expression statement lets you place an expression where a statement is expected
    /// They exist to evaluate expressions that may have side effects
    #[display(fmt = "ExprStmt [{}]", "_0")]
    ExprStmt(Box<Expression>),
    /// A print statement just evaluates the expression and prints to console
    #[display(fmt = "PrintStmt : [{}]", "*_0")]
    Print(Box<Expression>),
}

