use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::interpreter::{Environment, Memory};
use crate::parser::error::{EvalError, RuntimeError};
use crate::parser::expressions::*;
use crate::parser::traits::lox_callable::LoxCallable;
use crate::parser::value::ValueResult;
use crate::parser::value::{LoxFunction, Value};
use crate::tokenizer::token_type::TokenType::*;
use crate::{loc, Lox};
pub trait Evaluate {
    type Environment: Memory;
    fn eval(&self, env: &Self::Environment) -> ValueResult;
}

impl Evaluate for Expression {
    type Environment = Rc<RefCell<Environment>>;
    fn eval(&self, env: &Self::Environment) -> ValueResult {
        match self {
            Expression::CommaExpr(expr_list) => {
                // Comma expressions evaluate the list, discarding all results uptil the last one
                expr_list.iter().enumerate().for_each(|(idx, item)| {
                    if idx != expr_list.len() - 1 {
                        // eval and discard
                        match item.eval(env) {
                            Ok(_x) => { /*println!("Evaluating {item:?} got -> {x:?}")*/ }
                            Err(e) => println!("Evaluating {item:?} got error -> {e:?}"),
                        }
                    }
                });
                if let Some(last) = expr_list.last() {
                    last.eval(env)
                } else {
                    Err(EvalError::InvalidExpr(
                        self.clone(),
                        Some(format!("Cannot evaluate comma expression {:?}", expr_list)),
                    ))
                }
            }
            Expression::TernExpr(ternary) => ternary.eval(env),
            Expression::BinExpr(bin_exp) => bin_exp.eval(env),
            Expression::UnExpr(un_exp) => un_exp.eval(env),
            Expression::Lit(literal) => literal.eval(env),
            Expression::Group(group) => group.eval(env),
            // TODO: We need to interpret this separately in the Interpreter as
            // Only the Interpreter has access to Environment, for now we don't add it to Evaluate trait definition
            Expression::Assignment(assignment_expr) => assignment_expr.eval(env),
            // For now let's throw an error on error production evaluations
            Expression::Error(_err) => Err(EvalError::ErrorProduction),
            // We include this because user may hit `a` and expect to see a value just like in python
            Expression::Variable(t) => {
                // We want the syntax tree to reflect that an l-value isn’t evaluated like a normal expression.
                // TODO: What should a variable evaluate to?
                match env.get(t) {
                    Ok(v) => {
                        if let Some(x) = v {
                            Ok(x.to_owned())
                        } else {
                            // Ok(None) means variable was found in storage, but not initialized therefore it's an error
                            // to use it before initialization
                            Err(EvalError::VariableEval(RuntimeError::UndefinedVar(
                                t.lexeme.clone(),
                            )))
                        }
                    }
                    // undefined
                    Err(err) => {
                        loc!(format!("Error on variable.eval() {err}"));
                        Err(EvalError::VariableEval(err))
                    }
                }
            }
            Expression::LogicOr(l) => l.eval(env),
            Expression::LogicAnd(l) => l.eval(env),
            Expression::Call(
                fncallexpr @ FnCallExpr {
                    callee,
                    paren: _, // TODO: use this for error reporting
                    args,
                },
            ) => {
                // We allow for Fn(1)(2)(3).. so the callee for (2) is actually Fn(1) and the callee for (3) is actually Fn(1)(2)

                // TODO : In case of an indentifier or Variable(Token), what modifications
                // should we make to Variable(Token)'s evaluation implementation for this
                // to work correctly?

                // For now, we stay consistent with our overall pattern and "eval" whatever the callee expression is
                let evaluated_callee: Value = callee.eval(env)?;
                let mut args_result: Vec<ValueResult> = vec![];
                for arg in args.iter() {
                    args_result.push(arg.eval(env));
                }
                if args_result.iter().any(|res| res.is_err()) {
                    return Err(EvalError::FunctionArgError);
                }
                let args = args_result
                    .into_iter()
                    .map(|x| x.unwrap())
                    .collect::<Vec<_>>();

                if let Value::Function(lox_fn) = evaluated_callee {
                    <LoxFunction as LoxCallable>::call(&lox_fn, args, env)
                } else {
                    return Err(EvalError::FunctionCallError(fncallexpr.location()));
                }
            }
        }
    }
}
// logical operators short circuit in rust so we can make use of that
// https://stackoverflow.com/questions/53644809/do-logical-operators-short-circuit-in-rust
// https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators
impl Evaluate for AndExpr {
    type Environment = Rc<RefCell<Environment>>;
    fn eval(&self, env: &Self::Environment) -> ValueResult {
        Ok(
            (self.left.eval(env)?.is_truthy() && self.right.eval(env)?.is_truthy())
                .into(),
        )
    }
}
impl Evaluate for OrExpr {
    type Environment = Rc<RefCell<Environment>>;

    fn eval(&self, env: &Self::Environment) -> ValueResult {
        // Ok((self.left.eval(env)?.is_truthy() || panic!("cannot panic this if left true")).into())
        Ok(
            (self.left.eval(env)?.is_truthy() || self.right.eval(env)?.is_truthy())
                .into(),
        )
    }
}

impl Evaluate for AssignmentExpr {
    type Environment = Rc<RefCell<Environment>>;

    fn eval(&self, env: &Self::Environment) -> ValueResult {
        let (name, right) = (&self.name.lexeme, &self.right);
        let rval = right.eval(env)?;
        /*.map_err(|eval_err| {
            // Lox::report_runtime_err(format!("{eval_err}"));
            eval_err // Idempotent mapping lol
        })?;*/
        match env.put(name, rval.clone()) {
            // print a = 2 should print "2"
            Ok(()) => Ok(rval),
            Err(err) => {
                loc!(format!("{err}"));
                Lox::report_runtime_err(format!("{err}"));
                Err(EvalError::InvalidExpr(
                    Expression::Assignment(self.clone()),
                    Some("Cannot assign as variable not declared. Consider declaring with `var` first ".into()),
                ))
            }
        }
    }
}

impl Evaluate for TernaryExpr {
    type Environment = Rc<RefCell<Environment>>;

    fn eval(&self, env: &Self::Environment) -> ValueResult {
        // TernaryExpr { condition : Box<expr> , if_true : Box<expr>, if_false : Box<expr> }
        let condition = self.condition.eval(env)?;
        let condition = condition.is_truthy();
        let result = [&self.if_false, &self.if_true][condition as usize];
        result.eval(env)
    }
}

impl Evaluate for BinaryExpr {
    type Environment = Rc<RefCell<Environment>>;

    fn eval(&self, env: &Self::Environment) -> ValueResult {
        let err_exp = Expression::BinExpr(self.clone());
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;
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
                        let r = rstr.into_owned();
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
                Some(o) => {
                    Ok(Value::from(o == Ordering::Greater || o == Ordering::Equal))
                }
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
    type Environment = Rc<RefCell<Environment>>;

    fn eval(&self, env: &Self::Environment) -> ValueResult {
        let right = self.operand.eval(env)?;
        let result = match self.operator.r#type {
            BANG => Value::Bool(!right.is_truthy()),
            MINUS => match right {
                Value::Double(rval) => Value::Double(-rval),
                _ => {
                    return Err(EvalError::InvalidExpr(
                        Expression::UnExpr(self.clone()),
                        None,
                    ))
                }
            },
            _ => {
                return Err(EvalError::InvalidExpr(
                    Expression::UnExpr(self.clone()),
                    Some("Cannot evaluate as unary expression".to_string()),
                ))
            }
        };
        Ok(result)
    }
}

impl Evaluate for Literal {
    type Environment = Rc<RefCell<Environment>>;

    fn eval(&self, _env: &Self::Environment) -> ValueResult {
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
    type Environment = Rc<RefCell<Environment>>;

    fn eval(&self, env: &Self::Environment) -> ValueResult {
        self.inner.eval(env)
    }
}
