#![allow(unused)]
use super::expressions::*;
use crate::_lox_::tokenizer::{token::Token, token_type::TokenType};
use std::fmt::Debug;



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