use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    r#type: TokenType,
    /// A Lexeme is a part of valid Lox grammer. Some lexemes can be single char long
    /// whilst others maybe two or more characters
    lexeme: String,
    // literal: Literal,
    /// We include line number to track syntax error
    line_number: usize,
}

impl Token {
    /// Create a new token with type info, value, and line number
    pub fn new(
        r#type: TokenType,
        lexeme: String,
        // literal: Literal,
        line_number: usize,
    ) -> Self {
        Self {
            r#type,
            lexeme,
            // literal,
            line_number,
        }
    }
    /// Returns a string representation of the current Token
    pub fn to_string(&self) -> String {
        format!("{:?} {} {}", self.r#type, self.lexeme, self.line_number)
    }
}

// #[derive(Debug, Default, Clone, Copy)]
// pub struct Literal;
