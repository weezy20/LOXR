#![allow(unused, warnings)]
use super::Memory;
use crate::{
    parser::{error::RuntimeError, value::Value},
    tokenizer::token::Token,
};
use std::{collections::HashMap, rc::Rc};

/// Construct the global environment with default
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    /// Enclosing scope, for global scope it's none
    enclosing: Option<Rc<Environment>>,
    is_global: bool,
}
impl Default for Environment {
    fn default() -> Self {
        Self {
            values: Default::default(),
            enclosing: None,
            is_global: true,
        }
    }
}
impl Environment {
    /// Create a new environment with an enclosing environment
    pub fn new(enclosing: Rc<Environment>) -> Self {
        Self {
            // If surrounded by an environment, cannot be global
            is_global: false,
            enclosing: Some(Rc::clone(&enclosing)),
            ..Default::default()
        }
    }
}
// impl Memory for Rc<Environment> {
//     fn define(&mut self, name: &str, value: Value) {
//         (*self).define(name, value)
//     }
//     fn get(&self, name: &Token) -> Result<&Value, RuntimeError> {
//         (*self).get(name)
//     }
//     fn put(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
//         (*self).put(name, value)
//     }
// }
impl Memory for Environment {
    fn define(&mut self, name: &str, value: Value) {
        // If previous was something, the user just used var x = _ syntax to reassign to x instead of
        // x = _ syntax
        let _previous: Option<Value> = self.values.insert(name.to_owned(), value);
    }
    fn get(&self, token: &Token) -> Result<&Value, RuntimeError> {
        // crate::loc!(format!("{:?}", self.values));
        let name = token.lexeme.clone();
        match self.values.get(&name) {
            Some(val) => Ok(val),
            None => {
                let mut current_env = self;
                // We either find a value in enclosing scopes or none
                let mut scoped_val: Option<&Value> = None;
                'check_scopes: loop {
                    if let Some(ref encl_env) = current_env.enclosing {
                        if let Some(val) = encl_env.get(&token).ok() {
                            break scoped_val = Some(val);
                        } else {
                            current_env = encl_env;
                            continue;
                        }
                    }
                    // No enclosing environment, current_env is global env
                    else {
                        assert!(
                            current_env.is_global,
                            "ICE: Current env expected to be global at this point"
                        );
                        current_env.values.get(&name).ok_or_else(|| {
                            RuntimeError::UncaughtReference(
                                token.clone(),
                                format!("variable '{name}' is not defined"),
                            )
                        });
                    }
                } // Loop ends at current_env= global scope
                if let Some(val) = scoped_val {
                    return Ok(val);
                }
                // This code is unreachable but exists to make the compiler happy
                println!("This should never print!");
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
