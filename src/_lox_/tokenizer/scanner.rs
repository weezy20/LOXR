//! The purpose of this file is to define a scanner that takes a string and tokenizes it

use crate::_lox_::lox::Lox;
use std::iter::Peekable;
use std::str::CharIndices;

use super::{token::Token, token_type::TokenType};
#[derive(Debug)]
pub struct Scanner<'a: 'b, 'b> {
    /// Source string to tokenize
    pub(crate) source: &'a str,
    /// Iterator over source characters
    chars: Peekable<CharIndices<'a>>,
    /// Offset from start of source
    current: usize,
    /// Points to the first character of the current lexeme under consideration
    start: usize,
    /// Line number in source string, starts with 1
    line: usize,
    /// A list of all tokens
    pub(crate) tokens: Vec<Token>,
    /// Pointer to our Lox instance
    pub(crate) lox: &'b mut Lox,
}

impl<'a, 'b> Scanner<'a, 'b> {
    /// Create a scanner that's ready to be used with scan_tokens
    pub fn new(source: &'a str, lox: &'b mut Lox) -> Self {
        let char_indices = source.char_indices().peekable();
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
    /// If line breaks encountered, incremenet line number
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
    /// create a new TokenType with the piece of text from source and push it to tokens list
    fn add_token(&mut self, r#type: TokenType) {
        let lexeme_text = &self.source[self.start..self.current];
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
            ';' => self.add_token(TokenType::SEMICOLON),
            ' ' => {} // Ignore whitespace , for now.
            // Single or Double character lexemes: !, !=, <, <=, >, >=
            '!' => {
                // ! are a part of a lexeme "!=" just like "<=" or ">="
                if self.next_match('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '<' => {
                if self.next_match('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.next_match('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                // Either a comment start or a division operator
                if self.next_match('/') {
                    // We ignore everything till line end or source end whichever comes first
                    let comment_line = self.line;
                    while !self.is_at_end() && comment_line < self.line {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            _ => {
                self.lox.had_error = true; // Notify the lox machine that error has encountered so we can ignore running the file
                                           // however we must continue scanning tokens
                Lox::report_err(self.line, "Unexpected character".into());
            }
        }
        self.start = self.current; // set start to the beginning of next lexeme;
        Default::default()
    }
    /// Check if the very next character is equal to parameter
    fn next_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(&(_, next_ch)) = self.chars.peek() {
            if next_ch == expected {
                // Only advance "current" if the next char is what we expected
                self.current += 1;
                self.chars.next(); // Also advance our iterator to keep up with `current`
                return true;
            } else {
                return false;
            }
        } else {
            false
        }
    }
}
