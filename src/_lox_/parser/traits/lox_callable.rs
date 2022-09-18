use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::Environment;
use crate::parser::value::{Value, ValueResult};

/// Some type that can be called like classes or functions
/// Requires an environment to evaluate expressions
/// ### *Question*: Should callers provide the environment or should callables bring their own execution environment? 
/// Since a function should always execute in the execution context that was passed to it during its creation, it makes sense 
/// for the caller to not worry about it. For example, a function declared inside a scope should have access to the scope, but it shouldn't
/// be the caller's responsibility to explicitly mention this detail on every call
pub trait LoxCallable: std::fmt::Debug  {
    fn call(&self, args: Vec<Value>, env: Rc<RefCell<Environment>>) -> ValueResult;
    fn arity(&self) -> usize;
}
