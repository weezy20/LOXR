use super::{token::Token, token_type::TokenType};
#[derive(Debug)]
pub struct Scanner {
    /// Source string to tokenize
    source: String,
    /// Current offset for the lexeme, 0 by default
    current: usize,
    /// Points to the first character of the lexeme, starts with 0
    start: usize,
    /// Line number in source string, starts with 1
    line: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Scanner {
    /// Create a scanner that's ready to be used with scan_tokens
    pub fn new(source: String) -> Self {
        Self::default()
    }
    /// Calling this method updates the `source` field of scanner with the new String that was passed in
    /// Each call also updates the `line_number` for current scanner
    pub fn scan_tokens(&mut self, src: String) -> Vec<Token> {
        // Update state
        self.source = src;
        self.line += 1;
        
        // Start tokenizer
        let mut tokens: Vec<Token> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
        }
        tokens.push(Token::from(TokenType::AND));
        tokens
    }
    /// Are we at the end of current line?
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_single_token() {
        todo!()
    }
}
