use crate::_lox_::tokenizer::token::Token;
use crate::_lox_::tokenizer::token_type::TokenType;

/// # The overarching Expression type
///
/// An Expression can be of the following types:
/// 1. Literal
/// 2. Unary Expression with prefixes like ! or -
/// 3. Binary Expression with infix arithmetic operators  (+, -, *, /) or logic operators (==, !=, <, <=, >, >=)
/// 4. Parantheses: An Expression maybe wrapped in a a pair of ( and )

#[derive(PartialEq, Debug)]
pub enum Expression {
    BinExp(BinaryExpr),
    UnExp(UnaryExpr),
    Lit(Literal),
    Group(Grouping),
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpr {
    pub fn new(left: Box<Expression>, operator: Token, right: Box<Expression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub operator: Token,
    pub operand: Box<Expression>,
}
impl UnaryExpr {
    /// Question: What happens if operand : is a UnaryExpr. Nothing special, valid syntax
    pub fn new(operator: Token, operand: Box<Expression>) -> Result<Self, String> {
        match operator.r#type {
            TokenType::MINUS | TokenType::BANG => Ok(Self { operand, operator }),
            u => Err(format!(
                "Cannot construct Unary expression with operator: {u:?}"
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub inner: Token,
}
impl Literal {
    pub fn new(inner: Token) -> Result<Self, String> {
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

#[derive(Debug, PartialEq)]
pub struct Grouping {
    pub inner: Box<Expression>,
}

#[cfg(test)]
mod test {
    use crate::_lox_::parser::traits::ExpressionPrinter;
    use crate::_lox_::tokenizer::token::Token;
    use crate::_lox_::tokenizer::token_type::TokenType;

    use super::*;

    #[test]
    fn pretty_print() {
        let expression = " 1 + (2 - (4 / 5))";
        let (line_number, col) = (1, 1);
        let one = Expression::Lit(
            Literal::new(Token::new(TokenType::NUMBER, "1".into(), line_number, col)).unwrap(),
        );
        let two = Expression::Lit(
            Literal::new(Token::new(TokenType::NUMBER, "2".into(), line_number, col)).unwrap(),
        );
        let four = Expression::Lit(
            Literal::new(Token::new(TokenType::NUMBER, "4".into(), line_number, col)).unwrap(),
        );
        let five = Expression::Lit(
            Literal::new(Token::new(TokenType::NUMBER, "5".into(), line_number, col)).unwrap(),
        );
        let group45 = Expression::Group(Grouping {
            inner: Box::new(Expression::BinExp(BinaryExpr {
                left: Box::new(four),
                right: Box::new(five),
                operator: Token::new(TokenType::SLASH, "/".into(), line_number, col),
            })),
        });

        let group245 = Expression::Group(Grouping {
            inner: Box::new(Expression::BinExp(BinaryExpr {
                left: Box::new(two),
                right: Box::new(group45),
                operator: Token::new(TokenType::MINUS, "-".into(), line_number, col),
            })),
        });

        let r#final = Expression::BinExp(BinaryExpr {
            left: Box::new(one),
            right: Box::new(group245),
            operator:  Token::new(TokenType::PLUS, "+".into(), line_number, col),
        });

        println!("{:?}", r#final.print());
    }
}
