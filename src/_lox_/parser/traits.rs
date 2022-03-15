#![allow(unused)]
use super::expressions::*;
use crate::_lox_::tokenizer::{token::Token, token_type::TokenType};
use std::fmt::Debug;

/// # The overarching Expression type
///
/// An Expression can be of the following types:
/// 1. Literal
/// 2. Unary Expression with prefixes like ! or -
/// 3. Binary Expression with infix arithmetic operators  (+, -, *, /) or logic operators (==, !=, <, <=, >, >=)
/// 4. Parantheses: An Expression maybe wrapped in a a pair of ( and )
///
///
/// This trait represents all four of them with a associated `type Kind` which can be filled
/// We intentionally only allow Expression types to be data structures with minimal or no methods as they
/// are intended to me communicators b/w our parser and interpreter
/// To add a new Expression type, all one needs to do is implement the behavior traits defined in this module and define the type
/// in expressions.rs file
/// Which will allow us to preserve the open(extension)-closed(modification) principle:
/// we only introduce new code, and don't have to go back and modify existing code.
/// This entire feat is achieved using rust enums and traits.
///
/// My understanding of Lox's visitor pattern is that it wants to accomodate the functional style of adding operations in one place
/// while refraining from modifying existing class code for types that already exist. In other words, we want to eliminate modifying
/// existing code, whenever we add a new type or behaviour. Let's say we have traits for various kinds of operations, then adding a new
/// operation would entail implementing that trait for each type, just as we would in a FP paradigm, but we wouldn't have to rewrite any
/// existing code.
#[derive(PartialEq, Debug)]
pub enum Expression {
    BinExp(BinaryExpr),
    UnExp(UnaryExpr),
    Lit(Literal),
    Group(Grouping),
}

/// Helper struct to store info for Expressions expansion
#[derive(Default)]
pub struct Metadata {
    /// Optional list of boxed Expressions
    expressions: Option<Vec<Box<Expression>>>,
}

macro_rules! start {
    ($id: tt) => {{
        let mut s = format!(" {} ( ", $id);
        s
    }};
}

pub trait ExpressionPrinter {
    /// String representation of current ExpressionPrinter
    fn print(&self) -> String;
}

impl ExpressionPrinter for Expression {
    fn print(&self) -> String {
        match self {
            Expression::BinExp(e) => e.print(),
            Expression::UnExp(e) => e.print(),
            Expression::Lit(e) => e.print(),
            Expression::Group(e) => e.print(),
        }
    }
}

impl ExpressionPrinter for Literal {
    fn print(&self) -> String {
        let mut s = start!("Literal");
        s.push_str(&self.inner.lexeme);
        s.push_str(" )");
        s
    }
}

impl ExpressionPrinter for Grouping {
    fn print(&self) -> String {
        let mut s = start!("Grouping");
        s.push_str(&self.inner.print());
        s.push_str(" ) ");
        s
    }
}

impl ExpressionPrinter for UnaryExpr {
    fn print(&self) -> String {
        let mut s = start!("UnaryExp");
        s.push_str(&self.operator.lexeme);
        s.push_str(&self.operand.print());
        s
    }
}

impl ExpressionPrinter for BinaryExpr {
    fn print(&self) -> String {
        let mut s = start!("BinaryExp");
        s.push_str(&self.operator.lexeme);
        s.push_str(&self.left.print());
        s.push_str(&self.right.print());
        s
    }
}
