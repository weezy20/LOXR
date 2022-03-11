#![allow(unused)]
use super::tokenizer::{token::Token, token_type::TokenType};

macro_rules! impl_expr {
    ($e: tt) => {
        impl Expression for $e {}
    };
}

/// An expression can be of the following types:
/// 1. Literal
/// 2. Unary expressions with prefixes like ! or -
/// 3. Binary expressions with infix arithmetic operators  (+, -, *, /) or logic operators (==, !=, <, <=, >, >=)
/// 4. Parantheses: An expression maybe wrapped in a a pair of ( and )
/// This trait represents all four of them with a associated `type Kind` which can be filled
/// We intentionally only allow Expression types to be data structures with minimal or no methods as they
/// are intended to me communicators b/w our parser and interpreter
pub(crate) trait Expression {}

impl_expr!(BinaryExpr);
impl_expr!(UnaryExpr);
impl_expr!(Literal);
impl_expr!(Grouping);

/// A boxed expression. Boxy read as Box-E where E -> Expression. Get it? haha
pub(crate) type Boxy = Box<dyn Expression>;

/// I guess having a BinaryExp with left Boxy<U> <operator> right Boxy<K> wouldn't make sense unless
/// it's a loosely typed system like Javascript where you can add a number to a string but that's for later
/// We are intentionally refraining from any generic mumbo jumbo just to make life easier.
pub(crate) struct BinaryExpr {
    left: Boxy,
    operator: Token,
    right: Boxy,
}

impl BinaryExpr {
    pub(crate) fn new(left: Boxy, operator: Token, right: Boxy) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

pub(crate) struct UnaryExpr {
    operator: Token,
    operand: Boxy,
}
impl UnaryExpr {
    pub(crate) fn new(operator: Token, operand: Boxy) -> Result<Self, String> {
        match operator.r#type {
            TokenType::MINUS | TokenType::BANG => Ok(Self { operand, operator }),
            u => Err(format!(
                "Cannot construct Unary expression with operator: {u:?}"
            )),
        }
    }
}

pub(crate) struct Literal {
    inner: Token,
}
impl Literal {
    pub(crate) fn new(inner: Token) -> Result<Self, String> {
        let token_type = inner.r#type;
        if token_type.is_literal() {
            Ok(Self { inner })
        } else {
            Err(format!(
                "Cannot build a literal of token type {token_type:?}"
            ))
        }
    }
}

pub(crate) struct Grouping {
    inner: Boxy,
}
