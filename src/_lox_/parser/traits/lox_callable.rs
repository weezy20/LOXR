use crate::interpreter::Memory;
use crate::parser::value::{ValueResult, Value};

/// Some type that can be called like classes or functions
/// Requires an environment to evaluate expressions
pub trait LoxCallable {
    type Environment: Memory;
    fn call(&self, args : Vec<Value>, env: &Self::Environment) -> ValueResult;
}
