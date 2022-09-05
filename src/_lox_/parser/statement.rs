use super::*;
use derive_more::Display;
#[derive(Debug, Display, Clone)]
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
    #[display(fmt = r#"PrintStmt "{}""#, "*_0")]
    Print(Box<Expression>),
    /// Represents a syntax error, maybe moved to Declaration
    ErrStmt { message: String },
    /// Represents a comment
    Empty,
    /// Block scopes
    #[display(fmt = "BlockStmt [{:?}]", "_0")]
    Block(Vec<Stmt>),
    /// If statement // todo: can be made better for formatting nested if/else
    #[display(
        fmt = "{{ \n\tIfStmt (Condition : {}) \n\t{} \n\t{}\n\t}}",
        condition,
        r#"{
            use colored::Colorize;
            // let mut padding = String::from("");

            format!("[Then({})]" , then_).bright_green().bold()
        }"#,
        r#"{
            use colored::Colorize;
            if let Some(e) = else_ {
                format!("[Else({})]", *e).bright_red().bold()
            } 
            else { "(No else)".to_string().bright_red() }
        }"#
    )]
    IfStmt {
        condition: Box<Expression>,
        then_: Box<Stmt>,
        else_: Option<Box<Stmt>>,
        // padding: usize,
    },
    #[display(fmt = "VarDecl IDENTIFER : '{}', Expression : {:?}", name, initializer)]
    VarDecl {
        name: String,
        initializer: Option<Box<Expression>>,
    },
}

// Since we are using Ok(ErrStmt) instead of Err(ParserError) at some stages : expression_statement and print_statement
// Having a From<ParserError> for ErrStmt would help
impl From<ParserError> for Stmt {
    fn from(perr: ParserError) -> Self {
        Stmt::ErrStmt {
            message: format!("{perr}"),
        }
    }
}
