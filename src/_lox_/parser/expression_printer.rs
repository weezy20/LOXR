//! This is an example of adding a new operation, We create a new file for our operation
//! We write the implementations for this operation for each type. If a new type was to be added
//! we just import the trait from this file, and implement it for our new type
#![allow(unused)]
use super::expressions::*;
use crate::_lox_::tokenizer::{token::Token, token_type::TokenType};
use std::fmt::Debug;

macro_rules! start {
    ($id: tt) => {{
        let mut s = format!(" {} ( ", $id);
        s
    }};
}

pub trait ExpressionPrinter: Expression {
    /// String representation of current ExpressionPrinter
    fn print(&self) -> String;
}

impl ExpressionPrinter for Literal {
    fn print(&self) -> String {
        let mut s = start!("Literal");
        s.push_str(&self.inner.lexeme);
        s.push_str(" )");
        s
    }
}
// Note the pattern
impl<E: Expression + ExpressionPrinter> ExpressionPrinter for Grouping<E> {
    fn print(&self) -> String {
        let mut s = start!("Grouping");
        s.push_str(&self.inner.print());
        s.push_str(" ) ");
        s
    }
}

impl<E: Expression + ExpressionPrinter> ExpressionPrinter for UnaryExpr<E> {
    fn print(&self) -> String {
        let mut s = start!("UnaryExp");
        s.push_str(&self.operator.lexeme);
        s.push_str(&self.operand.print());
        s
    }
}

impl<L, R> ExpressionPrinter for BinaryExpr<L, R>
where
    L: Expression + ExpressionPrinter,
    R: Expression + ExpressionPrinter,
{
    fn print(&self) -> String {
        let mut s = start!("BinaryExp");
        s.push_str(&self.operator.lexeme);
        s.push_str(&self.left.print());
        s.push_str(&self.right.print());
        s
    }
}
