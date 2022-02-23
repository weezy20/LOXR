use crate::tokenizer::token_type::TokenType;

#[derive(Debug, Default, Clone, Copy)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line_number: usize,
}

impl Token {
    /// Create a new token with type info, value, and line number
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Literal,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }
    /// Returns a string representation of the current Token
    pub fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.line_number)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Literal;
