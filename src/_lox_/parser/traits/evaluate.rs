use std::cmp::Ordering;

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
                            Ok(x) => { /*println!("Evaluating {item:?} got -> {x:?}")*/ },
                            Err(e) => println!("Evaluating {item:?} got error -> {e:?}"),
                        }
                    }
                });
                if let Some(last) = expr_list.last() {
                    last.eval()
                } else {
                    Err(EvalError::InvalidExpr(
                        self.clone(),
                        Some(format!("Cannot evaluate comma expression {:?}", expr_list)),
                    ))
                }
            }
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
            MODULUS => match (left.is_numeric(), right.is_numeric()) {
                (Some(lval), Some(rval)) => Ok(Value::from(lval % rval)),
                _ => Err(EvalError::InvalidExpr(
                    err_exp,
                    Some("Cannot apply modulo to this binexp".to_string()),
                )),
            },
            SLASH => {
                if let Some((lval, rval)) = left.is_numeric().and_then(|lval| {
                    if let Some(rval) = right.is_numeric() {
                        return Some((lval, rval));
                    }
                    None
                }) {
                    if rval == 0.0 {
                        Err(EvalError::DivideByZero(err_exp))
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
                    (Some(lstr), None) => {
                        let mut l = lstr.into_owned();
                        if let Some(n) = right.is_numeric() {
                            l.push_str(&(n.to_string()));
                            return Ok(Value::String(l.to_owned()));
                        } else {
                            return Err(EvalError::InvalidExpr(
                                err_exp,
                                Some("Cannot add this binexp".to_string()),
                            ));
                        }
                    }
                    (None, Some(rstr)) => {
                        let mut r = rstr.into_owned();
                        if let Some(n) = left.is_numeric() {
                            let mut x = n.to_string();
                            x.push_str(&r);
                            return Ok(Value::String(x.to_owned()));
                        } else {
                            return Err(EvalError::InvalidExpr(
                                err_exp,
                                Some("Cannot add this binexp".to_string()),
                            ));
                        }
                    }
                    _ => {
                        return Err(EvalError::InvalidExpr(
                            err_exp,
                            Some("Cannot add this binexp".to_string()),
                        ))
                    }
                }
            }
            GREATER => match left.partial_cmp(&right) {
                Some(o) => Ok(Value::from(o == Ordering::Greater)),
                None => Err(EvalError::InvalidExpr(
                    err_exp,
                    Some(format!("Cannot compare {left:?} with {right:?}")),
                )),
            },
            GREATER_EQUAL => match left.partial_cmp(&right) {
                Some(o) => Ok(Value::from(o == Ordering::Greater || o == Ordering::Equal)),
                None => Err(EvalError::InvalidExpr(
                    err_exp,
                    Some(format!("Cannot compare {left:?} with {right:?}")),
                )),
            },
            LESS => match left.partial_cmp(&right) {
                Some(o) => Ok(Value::from(o == Ordering::Less)),
                None => Err(EvalError::InvalidExpr(
                    err_exp,
                    Some(format!("Cannot compare {left:?} with {right:?}")),
                )),
            },
            LESS_EQUAL => match left.partial_cmp(&right) {
                Some(o) => Ok(Value::from(o == Ordering::Less || o == Ordering::Equal)),
                None => Err(EvalError::InvalidExpr(
                    err_exp,
                    Some(format!("Cannot compare {left:?} with {right:?}")),
                )),
            },
            EQUAL_EQUAL => match left.partial_cmp(&right) {
                Some(o) => Ok(Value::from(o == Ordering::Equal)),
                None => Err(EvalError::InvalidExpr(
                    err_exp,
                    Some(format!("Cannot compare {left:?} with {right:?}")),
                )),
            },
            BANG_EQUAL => match left.partial_cmp(&right) {
                Some(o) => Ok(Value::from(!(o == Ordering::Equal))),
                None => Err(EvalError::InvalidExpr(
                    err_exp,
                    Some(format!("Cannot compare {left:?} with {right:?}")),
                )),
            },
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
