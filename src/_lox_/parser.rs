use std::fmt::Binary;

use super::tokenizer::{token::Token, token_type::TokenType};
pub(crate) trait Expression {}

type BoxExpr = Box<dyn Expression>;
pub(crate) struct BinaryExp {
    left: BoxExpr,
    operator: Token,
    right: BoxExpr,
}

impl Expression for BinaryExp {}

impl BinaryExp {
    fn new(left: BoxExpr, operator: Token, right: BoxExpr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

pub(crate) struct UnaryExp {
    left: Token,
    operand: BoxExpr,
}
impl Expression for UnaryExp {}
