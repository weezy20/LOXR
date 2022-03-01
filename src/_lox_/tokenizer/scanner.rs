//! The purpose of this file is to define a scanner that takes a string and tokenizes it

use std::str::CharIndices;

use crate::_lox_::lox::Lox;

use super::{token::Token, token_type::TokenType};
#[derive(Debug)]
pub struct Scanner<'a: 'b, 'b> {
    /// Source string to tokenize
    source: &'a str,
    /// Iterator over source characters
    chars: CharIndices<'a>,
    /// Offset from start of source
    current: usize,
    /// Points to the first character of the current lexeme under consideration
    start: usize,
    /// Line number in source string, starts with 1
    line: usize,
    /// A list of all tokens
    tokens: Vec<Token>,
    /// Pointer to our Lox instance
    lox: &'b mut Lox,
}

impl<'a, 'b> Scanner<'a, 'b> {
    /// Create a scanner that's ready to be used with scan_tokens
    pub fn new(source: &'a str, lox: &'b mut Lox) -> Self {
        let char_indices = source.char_indices();
        Self {
            source,
            lox,
            current: 0,
            start: 0,
            line: 0,
            tokens: vec![],
            chars: char_indices,
        }
    }
    /// The raison d'etere for this file, note the trailing 's', different from scan_token()
    pub fn scan_tokens(&mut self) {
        // Each turn of this loop should consume as many characters as it wants
        // to produce a single Token
        while !self.is_at_end() {
            // initialize start to the beginning of next lexeme
            self.start = self.current;
            let _next = self.scan_single_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".into(), self.line));
    }
    /// Are we at the end of source code?
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    /// Consume the iterator, increment `current` offset and return the next char, returns "" if nothing left
    fn advance(&mut self) -> Option<char> {
        if let Some((_pos, next_char)) = self.chars.next() {
            self.current += 1;
            if next_char == '\n' {
                self.line += 1;
            }
            Some(next_char)
        } else {
            None
        }
    }
    fn add_token(&mut self, r#type: TokenType) {
        let lexeme_text = &self.source[self.start..self.current + 1];
        self.tokens
            .push(Token::new(r#type, lexeme_text.into(), self.line))
    }
    fn scan_single_token(&mut self) -> Option<Token> {
        let c = self.advance()?;
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            '[' => self.add_token(TokenType::LEFT_SQUARE),
            ']' => self.add_token(TokenType::RIGHT_SQUARE),
            ',' => self.add_token(TokenType::RIGHT_SQUARE),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            '*' => self.add_token(TokenType::STAR),
            '/' => self.add_token(TokenType::SLASH),
            ';' => self.add_token(TokenType::SEMICOLON),
            _ => self.lox.had_error = true, // Notify the lox machine that error has encountered so we can ignore running the file
                                            // however we must continue scanning tokens
        }
        Default::default()
    }
}
