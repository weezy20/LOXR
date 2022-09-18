use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::Environment;
use crate::parser::value::{Value, ValueResult};

/// Some type that can be called like classes or functions
/// Requires an environment to evaluate expressions
pub trait LoxCallable: std::fmt::Debug {
    fn call(&self, args: Vec<Value>, env: Rc<RefCell<Environment>>) -> ValueResult;
    fn arity(&self) -> usize;
}
