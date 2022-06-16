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
    TERNARYC,      // ? Ternary operator condition
    TERNARYE,      // : Ternary operator else

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
    MISSING_OPERAND
}

use crate::_lox_::tokenizer::token_type::TokenType::*;
impl TokenType {
    /// Check if the literal is a primary as in contains a name, number, string, boolean, or nil 
    /// This may be subject to change as the parser progresses or maybe removed entirely
    pub fn is_primary(&self) -> bool {
        match self {
           NIL | FALSE | TRUE | STRING | IDENTIFIER | NUMBER => true,
           _ => false
        }
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        let str = match self {
            LEFT_PAREN => "(",
            RIGHT_PAREN => ")",
            LEFT_BRACE => "{",
            RIGHT_BRACE => "}",
            LEFT_SQUARE => "[",
            RIGHT_SQUARE => "]",
            COMMA => ",",
            DOT => ".",
            MINUS => "-",
            PLUS => "+",
            SEMICOLON => ";",
            SLASH => "/",
            STAR => "*",
            BANG => "!",
            BANG_EQUAL => "!=",
            EQUAL => "=",
            EQUAL_EQUAL => "==",
            GREATER => ">",
            GREATER_EQUAL => ">=",
            LESS => "<",
            LESS_EQUAL => "<=",
            IDENTIFIER => "some identifer",
            STRING => "some string",
            NUMBER => "some number",
            AND => "and",
            CLASS => "class",
            ELSE => "else",
            FALSE => "false",
            FUN => "fun",
            FOR => "for",
            IF => "if",
            NIL => "nil",
            OR => "or",
            PRINT => "print",
            RETURN => "return",
            SUPER => "super",
            THIS => "this",
            TRUE => "true",
            VAR => "var",
            WHILE => "while",
            EOF => "eof",
            MULTI_LINE_COMMENT => "multi-line comment",
            COMMENT => "single-line comment",
            TERNARYC => "?",
            TERNARYE => ":",
            MISSING_OPERAND => "Missing Operand",
        };
        str.to_string()
    }
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::NIL
    }
}

