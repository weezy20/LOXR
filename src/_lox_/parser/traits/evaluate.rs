use crate::parser::error::EvalError;
use crate::parser::expressions::*;
use crate::parser::value::Value;
use crate::tokenizer::token_type::TokenType::*;

type ValueResult = Result<Value, EvalError>;

pub trait Evaluate {
    fn eval(&self) -> ValueResult;
}

impl Evaluate for Expression {
    fn eval(&self) -> ValueResult {
        match self {
            Expression::CommaExpr(expr_list) => {
                // Comma expressions evaluate the list, discarding all results uptil the last one
                expr_list.iter().enumerate().for_each(|(idx, item)| {
                    if idx != expr_list.len() - 1 {
                        // eval and discard
                        match item.eval() {
                            Ok(x) => println!("Evaluating {item:?} got -> {x:?}"),
                            Err(e) => println!("Evaluating {item:?} got error -> {e:?}"),
                        }
                    }
                });
                if let Some(last) = expr_list.last() {
                    last.eval()
                } else {
                    Err(EvalError::InvalidExpr(self.clone(), Some("Cannot evaluate comma expression".into())))
                }
            },
            Expression::TernExp(ternary) => ternary.eval(),
            Expression::BinExp(bin_exp) => bin_exp.eval(),
            Expression::UnExp(un_exp) => un_exp.eval(),
            Expression::Lit(literal) => literal.eval(),
            Expression::Group(group) => group.eval(),
            // For now let's throw an error on error production evaluations
            Expression::Error(err) => Err(EvalError::ErrorProduction),
        }
    }
}

impl Evaluate for TernaryExpr {
    fn eval(&self) -> ValueResult {
        // TernaryExpr { condition : Box<expr> , if_true : Box<expr>, if_false : Box<expr> }
        let condition = self.condition.eval()?;
        let condition = condition.is_truthy();
        let result = [&self.if_false, &self.if_true][condition as usize];
        result.eval()
    }
}

impl Evaluate for BinaryExpr {
    fn eval(&self) -> ValueResult {
        let err_exp = Expression::BinExp(self.clone());
        let left = self.left.eval()?;
        let right = self.right.eval()?;
        match self.operator.r#type {
            MINUS => {
                if let Some((lval, rval)) = left.is_numeric().and_then(|lval| {
                    if let Some(rval) = right.is_numeric() {
                        return Some((lval, rval));
                    }
                    None
                }) {
                    Ok(Value::Double(lval - rval))
                } else {
                    Err(EvalError::InvalidExpr(
                        err_exp,
                        Some("Cannot subtract this binexp".to_string()),
                    ))
                }
            }
            SLASH => {
                if let Some((lval, rval)) = left.is_numeric().and_then(|lval| {
                    if let Some(rval) = right.is_numeric() {
                        return Some((lval, rval));
                    }
                    None
                }) {
                    if rval == 0.0 {
                        Err(EvalError::InvalidExpr(
                            err_exp,
                            Some("Cannot divide by zero".to_string()),
                        ))
                    } else {
                        Ok(Value::Double(lval / rval))
                    }
                } else {
                    Err(EvalError::InvalidExpr(
                        err_exp,
                        Some("Cannot divide this binexp".to_string()),
                    ))
                }
            }
            STAR => {
                if let Some((lval, rval)) = left.is_numeric().and_then(|lval| {
                    if let Some(rval) = right.is_numeric() {
                        return Some((lval, rval));
                    }
                    None
                }) {
                    Ok(Value::Double(lval * rval))
                } else {
                    Err(EvalError::InvalidExpr(
                        err_exp,
                        Some("Cannot multiply this binexp".to_string()),
                    ))
                }
            }
            PLUS => {
                if let Some((lval, rval)) = left.is_numeric().and_then(|lval| {
                    if let Some(rval) = right.is_numeric() {
                        return Some((lval, rval));
                    }
                    None
                }) {
                    return Ok(Value::Double(lval + rval));
                }
                // Another approach for mutliple Options
                match (left.is_string(), right.is_string()) {
                    (Some(lstr), Some(rstr)) => {
                        // into_owned moves data out of the Cow
                        // This should be fine as once we eval a binexp, we won't need the value
                        let mut l = lstr.into_owned();
                        l.push_str(&rstr);
                        return Ok(Value::String(l.to_owned()));
                    }
                    _ => {
                        return Err(EvalError::InvalidExpr(
                            err_exp,
                            Some("Cannot add this binexp".to_string()),
                        ))
                    }
                }
            }
            _ => Err(EvalError::InvalidExpr(err_exp, None)),
        }
    }
}

impl Evaluate for UnaryExpr {
    fn eval(&self) -> ValueResult {
        let right = self.operand.eval()?;
        let mut result = match self.operator.r#type {
            BANG => Value::Bool(!right.is_truthy()),
            MINUS => match right {
                Value::Double(rval) => Value::Double(-rval),
                _ => {
                    return Err(EvalError::InvalidExpr(
                        Expression::UnExp(self.clone()),
                        None,
                    ))
                }
            },
            _ => {
                return Err(EvalError::InvalidExpr(
                    Expression::UnExp(self.clone()),
                    Some("Cannot evaluate as unary expression".to_string()),
                ))
            }
        };
        Ok(result)
    }
}

impl Evaluate for Literal {
    fn eval(&self) -> ValueResult {
        match self.inner.r#type {
            STRING => Ok(self.inner.lexeme.clone().into()),
            NUMBER => {
                let n = (&self.inner.lexeme).parse::<f64>().expect(
                    "Internal compiler error: Parsing a Number token as Number is infallible",
                );
                Ok(n.into())
            }
            TRUE => Ok(Value::Bool(true)),
            FALSE => Ok(Value::Bool(false)),
            NIL => Ok(Value::Nil),
            _ => Err(EvalError::InvalidExpr(Expression::Lit(self.clone()), None)),
        }
    }
}

impl Evaluate for Grouping {
    fn eval(&self) -> ValueResult {
        self.inner.eval()
    }
}
