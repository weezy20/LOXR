use super::{token::Token, token_type::TokenType};
pub struct Scanner {
    source: String,
    /// Current offset for the lexeme 
    current: usize,
    /// Points to the first character of the lexeme
    start: usize,
    /// Line number in source string
    line: usize,
}

impl Scanner {
    /// Create a scanner that's ready to be used with scan_tokens
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            start: 0,
            line: 1,
        }
    }
    /// A Lexeme is a part of valid Lox grammer. Some lexemes can be single char long
    /// whilst others maybe two or more characters
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
        }
        tokens.push(TokenType::EOF);
        tokens
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_single_token() {
        todo!()
    }
}
