#![allow(unused)]
use std::io::Write;

use super::token_type::TokenType;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    /// A Lexeme is a part of valid Lox grammer. Some lexemes can be single char long
    /// whilst others maybe two or more characters
    pub lexeme: String,
    /// We include line number to track syntax error
    pub line_number: usize,
    /// Column where token starts
    pub col: usize,
}

impl Token {
    /// Create a new token with type info, value, and line number
    pub fn new(
        r#type: TokenType,
        lexeme: String,
        line_number: usize,
        col: usize,
    ) -> Self {
        Self {
            r#type,
            lexeme,
            line_number,
            col,
        }
    }
    /// Returns a string representation of the current Token
    pub fn to_string(&self) -> String {
        let mut q = '"';
        let mut line_beginning = self.line_number;
        if self.r#type == TokenType::STRING {
            q = '"'; // Note that we already trim out the quotes from source string during scan_string
                     // Offset by new lines if a multi string is present
            line_beginning = self.line_number - self.lexeme.matches('\n').count();
        }
        if self.r#type == TokenType::EOF {
            return format!(
                "{:?} at ({}, {})",
                self.r#type, line_beginning, self.col
            );
        }
        format!(
            "{:?} {q}{}{q} at ({}, {})",
            self.r#type, self.lexeme, line_beginning, self.col
        )
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<TokenType> for Token {
    /// Create a token for test purposes
    fn from(ty: TokenType) -> Self {
        Token {
            r#type: ty,
            lexeme: ty.to_string(),
            ..Default::default()
        }
    }
}
