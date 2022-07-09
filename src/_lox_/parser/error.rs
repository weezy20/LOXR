use crate::parser::expressions::Expression;
use crate::tokenizer::token::Token;
use thiserror::Error;

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
    ErrorProduction(Box<Expression>),
}

#[derive(Error, Debug, PartialEq)]
pub enum EvalError {
    #[error("Expression Evaluation error: {}", match self {
        EvalError::InvalidExpr(exp, custom_msg) if custom_msg.is_some() => { 
            let msg = custom_msg.as_ref().unwrap();
            format!("{}\nInvalid Expression: {exp:?}", msg)
        },
        EvalError::InvalidExpr(exp, _) => { format!("Cannot evaluate: {:?}", exp) }
        _ => { "ICE : Uncaught exception".to_string() }
    }) ]
    InvalidExpr(Expression, Option<String>),
    #[error("Cannot evaluate Error production")]
    ErrorProduction
}
