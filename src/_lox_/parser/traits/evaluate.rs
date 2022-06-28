use crate::parser::expressions::*;
pub enum Value {
    Double(f64),
    Bool(bool),
    String(String),
    Nil,
}
impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}
impl From<String> for Value {
    fn from(x: String) -> Self {
        Self::String(x)
    }
}
impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Self::Double(f)
    }
}
pub trait Evaluate {
    fn eval(&self) -> Option<Value>;
}

impl Evaluate for Expression {
    fn eval(&self) -> Option<Value> {
        match self {
            Expression::CommaExpr(expr_list) => todo!(),
            Expression::TernExp(ternary) => todo!(),
            Expression::BinExp(bin_exp) => todo!(),
            Expression::UnExp(un_exp) => todo!(),
            Expression::Lit(literal) => literal.eval(),
            Expression::Group(group) => group.eval(),
            Expression::Error(err) => todo!(),
        }
    }
}

impl Evaluate for UnaryExpr {
    fn eval(&self) -> Option<Value> {
        todo!()
    }
}

impl Evaluate for Literal {
    fn eval(&self) -> Option<Value> {
        match self.inner.r#type {
            crate::tokenizer::token_type::TokenType::STRING => {
                Some(self.inner.lexeme.clone().into())
            }
            crate::tokenizer::token_type::TokenType::NUMBER => {
                let n = (&self.inner.lexeme).parse::<f64>().expect("Internal compiler error: Parsing a Number token as Number is infallible");
                Some(n.into())
            }
            crate::tokenizer::token_type::TokenType::TRUE => Some(Value::Bool(true)),
            crate::tokenizer::token_type::TokenType::FALSE => Some(Value::Bool(false)),
            crate::tokenizer::token_type::TokenType::NIL => Some(Value::Nil),
            _ => None,
        }
    }
}

impl Evaluate for Grouping {
    fn eval(&self) -> Option<Value> {
        // self.inner.eval()
        None
    }
}
