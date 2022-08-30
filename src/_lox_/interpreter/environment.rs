#![allow(unused, warnings)]
use crate::{
    parser::{error::RuntimeError, value::Value},
    tokenizer::token::Token,
};
use std::collections::HashMap;

struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    fn define(&mut self, name: &str, value: Value) {
        // If previous was something, the user just used var x = _ syntax to reassign to x instead of
        // x = _ syntax
        let _previous: Option<Value> = self.values.insert(name.to_owned(), value);
    }
    /// Getting a None represents that the value was declared but not initialized
    fn get(&self, token: Token) -> Result<Option<&Value>, RuntimeError> {
        let name = token.lexeme.clone();
        match self.values.get(&name) {
            Some(val) => Ok(Some(val)),
            None => {
                // redundant as when inserting values we make sure to insert Value::Nil for var declarations
                // if self.values.contains_key(&name) {
                //     return Ok(None);
                // }
                Err(RuntimeError::UncaughtReference(
                    token,
                    format!("variable {name} is not defined"),
                ))
            }
        }
    }
}
