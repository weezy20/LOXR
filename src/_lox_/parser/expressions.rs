use derive_more::Display;
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
    TernExpr(TernaryExpr),
    BinExpr(BinaryExpr),
    UnExpr(UnaryExpr),
    Lit(Literal),
    Group(Grouping),
    Error(Box<Expression>),
    Assignment(AssignmentExpr),
    // Should not be evaluated by the interpreter, only for parser usage
    Variable(Token),
    LogicOr(OrExpr),
    LogicAnd(AndExpr),
    Call(FnCallExpr),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print display for variants which have the display method otherwise fallback to debug print
        let out = match &self {
            Expression::BinExpr(x) => format!("{x}"),
            Expression::UnExpr(x) => format!("{x}"),
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
            Expression::TernExpr(x) => format!("{x:?}"),
            Expression::Group(x) => format!("{x:?}"),
            Expression::Error(x) => format!("{x:?}"),
            Expression::Assignment(AssignmentExpr { name, right }) => {
                format!("{name} = {right}")
            }
            Expression::Variable(t) => format!("{t}"),
            Expression::LogicOr(l) => format!("{l}"),
            Expression::LogicAnd(l) => format!("{l}"),
            Expression::Call(e) => format!("{e}"),
        };
        write!(f, "{out}")
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[display(fmt = "|Function Call to -> {:?} Args -> ({:?})|", callee, args)]
pub struct FnCallExpr {
    /// Typically one would expect a function call to look like `some_var_name(..args?)`
    /// Where some_var_name is a Variable token. But since 
    /// callee is a Box<Expression> it allows for multiple calls 
    /// to be chained Fn(a,b)(2,3,5) such that the callee to (2,3,5) is 
    /// Fn(a,b) which will be evaluated first before being assigned as a callee
    pub callee: Box<Expression>,
    /// Stores the token ')' to report a runtime err incase of trouble with the function call
    pub paren: Token,
    pub args: Vec<Box<Expression>>,
}
impl FnCallExpr {
    pub fn location(&self) -> String {
        self.paren.location()
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[display(fmt = "LogicalAnd(Left [{}] and Right [{}])", left, right)]
pub struct AndExpr {
    pub left: Box<Expression>,
    pub operator: Token, // type AND
    pub right: Box<Expression>,
}
#[derive(Debug, PartialEq, Clone, Display)]
#[display(fmt = "LogicalOr(Left [{}] or Right [{}])", left, right)]
pub struct OrExpr {
    pub left: Box<Expression>,
    pub operator: Token, // type OR
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpr {
    /// Type IDENTIFIER
    pub name: Token,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TernaryExpr {
    pub condition: Box<Expression>,
    pub if_true: Box<Expression>,
    pub if_false: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone, Display)]
#[display(fmt = "{left} {operator} {right}")]
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

#[derive(Debug, PartialEq, Clone, Display)]
#[display(fmt = "{operator}{operand}")]

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

#[derive(Debug, PartialEq, Clone, Default, Display)]
#[display(fmt = "{inner}")]

pub struct Literal {
    pub inner: Token,
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
            Literal::new(Token::new(TokenType::NUMBER, "1".into(), line_number, col))
                .unwrap(),
        );
        let two = Expression::Lit(
            Literal::new(Token::new(TokenType::NUMBER, "2".into(), line_number, col))
                .unwrap(),
        );
        let four = Expression::Lit(
            Literal::new(Token::new(TokenType::NUMBER, "4".into(), line_number, col))
                .unwrap(),
        );
        let five = Expression::Lit(
            Literal::new(Token::new(TokenType::NUMBER, "5".into(), line_number, col))
                .unwrap(),
        );
        let group45 = Expression::Group(Grouping {
            inner: Box::new(Expression::BinExpr(BinaryExpr {
                left: Box::new(four),
                right: Box::new(five),
                operator: Token::new(TokenType::SLASH, "/".into(), line_number, col),
            })),
        });

        let group245 = Expression::Group(Grouping {
            inner: Box::new(Expression::BinExpr(BinaryExpr {
                left: Box::new(two),
                right: Box::new(group45),
                operator: Token::new(TokenType::MINUS, "-".into(), line_number, col),
            })),
        });

        let r#final = Expression::BinExpr(BinaryExpr {
            left: Box::new(one),
            right: Box::new(group245),
            operator: Token::new(TokenType::PLUS, "+".into(), line_number, col),
        });

        println!("{:?}", r#final.print());
    }
}
