//! This module contains all definitions for the Lox compiler and Lox interpreter

/// Machinery for running a Lox file or Lox interpreter
pub mod lox;

/// A module for token definitions, and a lox lexer and scanner
pub mod tokenizer;

/// Parser module that defines Lox syntactical grammar and constructs ASTs
pub mod parser;
