use crate::interpreter::Memory;
use crate::parser::value::{LoxFunction, ValueResult};

pub trait LoxCallable<E: Memory> {
    fn call(&self, env: &E) -> ValueResult;
}

impl<E: Memory> LoxCallable<E> for LoxFunction {
    fn call(&self, env: &E) -> ValueResult {
        todo!()
    }
}
