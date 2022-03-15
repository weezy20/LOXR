//! Definitions for Token types
#[allow(non_camel_case_types, unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LEFT_PAREN,    // (
    RIGHT_PAREN,   // )
    LEFT_BRACE,    // {
    RIGHT_BRACE,   // }
    LEFT_SQUARE,   // [
    RIGHT_SQUARE,  // ]
    COMMA,         // ,
    DOT,           // .
    MINUS,         // -
    PLUS,          // +
    SEMICOLON,     // ;
    SLASH,         // /
    STAR,          // *
    BANG,          // !
    BANG_EQUAL,    // !=
    EQUAL,         // =
    EQUAL_EQUAL,   // ==
    GREATER,       // >
    GREATER_EQUAL, // >=
    LESS,          // <
    LESS_EQUAL,    // <=

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF, // EOF

    MULTI_LINE_COMMENT,
    COMMENT,
}

use crate::_lox_::tokenizer::token_type::TokenType::*;
impl TokenType {
    pub fn is_literal(&self) -> bool {
        match self {
           STRING | IDENTIFIER | NUMBER => true,
           _ => false
        }
    }
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::NIL
    }
}

#[cfg(test)]
mod token_type_tests {
    use super::*;
    #[test]
    fn test_is_literal() {
        let s = STRING;
        let id = IDENTIFIER;
        let num = NUMBER;
        assert!(s.is_literal());
        assert!(id.is_literal());
        assert!(num.is_literal());
    }
}
