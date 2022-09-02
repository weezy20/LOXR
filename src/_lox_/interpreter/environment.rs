#![allow(unused, warnings)]
use super::Memory;
use crate::{
    parser::{error::RuntimeError, value::Value},
    tokenizer::token::Token,
};
use std::{collections::HashMap, rc::Rc};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    /// Enclosing scope, for global scope it's none
    enclosing: Option<Rc<Environment>>,
}
impl Environment {
    /// Create a new sub-environment of `enclosing`
    pub fn new(enclosing: &Rc<Environment>) -> Self {
        Self {
            enclosing: Some(Rc::clone(&enclosing)),
            ..Default::default()
        }
    }
}

impl Memory for Environment {
    fn define(&mut self, name: &str, value: Value) {
        // If previous was something, the user just used var x = _ syntax to reassign to x instead of
        // x = _ syntax
        let _previous: Option<Value> = self.values.insert(name.to_owned(), value);
    }
    /// Getting a None represents that the value was declared but not initialized
    fn get(&self, token: &Token) -> Result<Option<&Value>, RuntimeError> {
        // crate::loc!(format!("{:?}", self.values));
        let name = token.lexeme.clone();
        match self.values.get(&name) {
            Some(val) => Ok(Some(val)),
            None => {
                // redundant as when inserting values we make sure to insert Value::Nil for var declarations
                // if self.values.contains_key(&name) {
                //     return Ok(None);
                // }
                let mut super_scope = self.enclosing;
                while let Some(encl) = super_scope {
                    if let Ok(Some(val)) = encl.get(&token) {
                        return Ok(Some(val))
                    }
                    super_scope = encl.enclosing;
                }
                Err(RuntimeError::UncaughtReference(
                    token.clone(),
                    format!("variable '{name}' is not defined"),
                ))
            }
        }
    }
    fn put(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        // put is not allowed to create new keys or variable definitions, it can only update existing ones
        if !self.values.contains_key(name) {
            Err(RuntimeError::UndefinedVar(name.to_owned()))
        } else {
            self.values.insert(name.to_owned(), value);
            Ok(())
        }
    }
}
