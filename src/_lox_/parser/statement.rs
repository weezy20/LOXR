use super::*;
use derive_more::{From, Display};
#[derive(Debug, Display)]
/// A statement has side effects that may affect the `state` a lox program is in
/// A statement is always followed by a `;`.
/// A lox program is made up of lox statements
#[display(fmt = "{}")]
pub enum Stmt {
    /// An expression statement lets you place an expression where a statement is expected
    /// They exist to evaluate expressions that may have side effects
    #[display(fmt = "ExprStmt [{}]", "_0")]
    ExprStmt(Box<Expression>),
    /// A print statement evaluaets an expression and prints to console
    #[display(fmt = "PrintStmt : [{}]", "*_0")]
    Print(Box<Expression>),
    /// Represents a syntax error, maybe moved to Declaration
    ErrStmt {
        message : String
    },
    /// Represents a comment
    Empty
}

/// Since var decls don't make sense every where we wrap Stmt inside Declaration such that any place
/// that cannot accept a declaration can still accept a statement
#[derive(Debug, Display, From)]
pub enum Declaration {
    DStmt(Stmt),
    /// var go = "programming language by Google";
    #[display(fmt = "VarDecl")]
    VarDecl {
        name: String,
        initializer: Option<Box<Expression>>,
    },
}
