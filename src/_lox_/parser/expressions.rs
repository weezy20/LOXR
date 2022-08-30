use std::fmt::Display;

use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

/// # The overarching Expression type
///
/// An Expression can be of the following types:
/// 1. Literal
/// 2. Unary Expression with prefixes like ! or -
/// 3. Binary Expression with infix arithmetic operators  (+, -, *, /) or logic operators (==, !=, <, <=, >, >=)
/// 4. Grouping: An Expression maybe wrapped in a a pair of ( and )

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    CommaExpr(Vec<Box<Expression>>),
    TernExp(TernaryExpr),
    BinExp(BinaryExpr),
    UnExp(UnaryExpr),
    Lit(Literal),
    Group(Grouping),
    Error(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print display for variants which have the display method otherwise fallback to debug print
        let out = match &self {
            Expression::BinExp(x) => format!("{x}"),
            Expression::UnExp(x) => format!("{x}"),
            Expression::Lit(x) => format!("{x}"),
            Expression::CommaExpr(x) => {
                let mut res = String::new();
                res.push_str("[\n");
                for item in x {
                    res.push_str(&format!("\t{item},\n"));
                }
                let mut res = res.trim_end_matches("\n").to_owned();
                res.push_str("\n]");
                res
            }
            Expression::TernExp(x) => format!("{x:?}"),
            Expression::Group(x) => format!("{x:?}"),
            Expression::Error(x) => format!("{x:?}"),
        };
        write!(f, "{out}")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TernaryExpr {
    pub condition: Box<Expression>,
    pub if_true: Box<Expression>,
    pub if_false: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
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
impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = match &*self.left {
            Expression::CommaExpr(_) => "CommaExpr".into(),
            Expression::TernExp(_) => "TernaryExpr".into(),
            Expression::BinExp(_) => "BinExpr".into(),
            Expression::UnExp(unary_expr) => format!("{unary_expr}"),
            Expression::Lit(lit) => lit.inner.lexeme.clone(),
            Expression::Group(_) => "Grouping".into(),
            Expression::Error(_) => "ErrorProduction".into(),
        };
        let right = match &*self.right {
            Expression::CommaExpr(_) => "CommaExpr".into(),
            Expression::TernExp(_) => "TernaryExpr".into(),
            Expression::BinExp(_) => "BinExpr".into(),
            Expression::UnExp(unary_expr) => format!("{unary_expr}"),
            Expression::Lit(lit) => format!("{lit}"),
            Expression::Group(_) => "Grouping".into(),
            Expression::Error(_) => "ErrorProduction".into(),
        };
        write!(f, "{left} {op} {right}", op = self.operator.lexeme)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub operand: Box<Expression>,
}
impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operand = match &*self.operand {
            Expression::CommaExpr(_)
            | Expression::TernExp(_)
            | Expression::Group(_)
            | Expression::Error(_)
            | Expression::UnExp(_)
            | Expression::BinExp(_) => "Expr".into(),
            Expression::Lit(lit) => format!("{lit}"),
        };
        write!(f, "{}{operand}", &self.operator.lexeme)
    }
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Literal {
    pub inner: Token,
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.inner.lexeme)
    }
}
impl Literal {
    pub fn new(inner: Token) -> Result<Self, String> {
        let token_type = inner.r#type;
        if token_type.is_primary() {
            Ok(Self { inner })
        } else {
            Err(format!(
                "Cannot build a literal of token type {token_type:?}"
            ))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub inner: Box<Expression>,
}

impl Grouping {
    pub fn new(inner: Box<Expression>) -> Self {
        Self { inner }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::traits::printer::ExpressionPrinter;
    use crate::tokenizer::token::Token;
    use crate::tokenizer::token_type::TokenType;

    use super::*;

    #[test]
    fn pretty_print() {
        let _expression = " 1 + (2 - (4 / 5))";
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
            operator: Token::new(TokenType::PLUS, "+".into(), line_number, col),
        });

        println!("{:?}", r#final.print());
    }
}
