use crate::parser::{error::EvalError, traits::lox_callable::LoxCallable, value::Value};
use derive_more::Display;
use std::{cell::RefCell, rc::Rc, time::{SystemTime, UNIX_EPOCH}};

use super::Interpreter;
#[derive(Debug, Display)]
#[display(fmt = "<native fn: clock>")]
pub struct Clock;

impl LoxCallable for Clock {
    fn call(
        &self,
        args: Vec<crate::parser::value::Value>,
        _interpreter: &mut Interpreter
    ) -> crate::parser::value::ValueResult {
        if args.len() != 0 {
            crate::Lox::report_runtime_err(format!(
                "Expected {} but got {} arguments",
                self.arity(),
                args.len()
            ));
            return Err(EvalError::FunctionArgError);
        } else {
            Ok(Value::Double(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("system time before epoch")
                    .as_secs_f64(),
            ))
        }
    }
    fn arity(&self) -> usize {
        0
    }
}
