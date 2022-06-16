use thiserror::Error;
use crate::_lox_::tokenizer::token::Token;

use super::expressions::Expression;
#[allow(unused)]
#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Parenthesis mismatch")]
    UnbalancedParen,
    #[error("Invalid token found: {:#?}", match self {
        ParserError::InvalidToken(Some(t)) => format!("{t}", t=t.lexeme),
        ParserError::InvalidToken(None) => format!("Unknown Token"),
        _ => "This should never print lmao?".into()
    })]
    InvalidToken(Option<Token>),
    #[error("Expected operand ")]
    // Most of the times InvalidToken can be more powerful than this error variant
    MissingOperand,
    #[error("Expected Expression")]
    UnexpectedExpression,
    #[error("Error production")]
    ErrorProduction(Box<Expression>)
}
