use thiserror::Error;

use crate::_lox_::tokenizer::token::Token;
#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Parenthesis mismatch")]
    UnbalancedParen,
    #[error("Invalid token found")]
    InvalidToken(Option<Token>),
    #[error("Expected operand ")]
    MissingOperand,
    #[error("Expected Expression")]
    UnexpectedExpression
}
