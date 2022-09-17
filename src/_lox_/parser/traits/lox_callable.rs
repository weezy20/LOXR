use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::{Environment, Memory};
use crate::parser::value::{LoxFunction, ValueResult};

pub trait LoxCallable {
    type Environment: Memory;
    fn call(&self, env: &Self::Environment) -> ValueResult;
}

impl LoxCallable for LoxFunction {
    type Environment = Rc<RefCell<Environment>>;
    fn call(&self, env: &Self::Environment) -> ValueResult {
        todo!()
    }
}
