use super::Memory;
use crate::{
    parser::{error::RuntimeError, value::Value},
    tokenizer::token::Token,
};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

/// Construct the global environment with default
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub values: HashMap<String, Value>,
    /// Enclosing scope, for global scope it's none
    /// The parent environment may be shared by multiple scopes and require interior mutablity for ops
    /// therefore it makes sense to have a RefCell which allows us to obtain a mutable ref to inner Environment
    /// We know this will be safe as the program is single threaded and an "enclosing" environment will never
    /// be simultaneously mutated
    enclosing: Option<Rc<RefCell<Environment>>>,
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
    pub fn new(enclosing: Rc<RefCell<Environment>>) -> Self {
        let enclosing = Some(Rc::clone(&enclosing));
        Self {
            // If surrounded by an environment, cannot be global
            is_global: false,
            enclosing,
            ..Default::default()
        }
    }
}
impl Memory for Rc<RefCell<Environment>> {
    fn define(&mut self, name: &str, value: Value) {
        // If previous was something, the user just used var x = _ syntax to reassign to x instead of
        // x = _ syntax
        let _previous: Option<Value> = self.borrow_mut().values.insert(name.to_owned(), value);
    }
    fn get(&self, token: &Token) -> Result<Option<Value>, RuntimeError> {
        // crate::loc!(format!("{:?}", self.values));
        let name = token.lexeme.clone();
        match self.borrow().values.get(&name) {
            Some(val) if *val == Value::Nil => Ok(None),
            Some(val) => Ok(Some(val.to_owned())),
            None => {
                let current_env: Rc<RefCell<Environment>> = Rc::clone(&self);
                // We either find a value in enclosing scopes or none
                // no clue why this is caught as unused assignment
                // It was an unused assignment becz we never read the RHS ( = None )
                let scoped_val: Option<Value>;
                '_check_scopes: loop {
                    if let Some(ref encl_env) = current_env.borrow().enclosing {
                        if let Ok(Some(val)) = encl_env.get(&token) {
                            break scoped_val = Some(val);
                        } else if let Ok(None) = encl_env.get(&token) {
                            // Variable declared but has Nil initializer
                            break scoped_val = None;
                        } else {
                            current_env.swap(encl_env);
                            continue;
                        }
                    }
                    // No enclosing environment, current_env is global env
                    // Upto this we have not found the var declared
                    else {
                        assert!(
                            current_env.borrow().is_global,
                            "ICE: Current env expected to be global at this point"
                        );
                        let encl_borrow = current_env.borrow();
                        match encl_borrow.values.get(&name) {
                            Some(val) if *val == Value::Nil => return Ok(None),
                            Some(val) => return Ok(Some(val.to_owned())),
                            None => {
                                return Err(RuntimeError::UncaughtReference(
                                    token.clone(),
                                    format!("variable '{name}' is not defined"),
                                ))
                            }
                        }
                    }
                } // Loop ends at current_env = global scope
                return Ok(scoped_val);
                // This code is unreachable but exists to make the compiler happy
                // eprintln!("This should never print!");
                // Err(RuntimeError::UncaughtReference(
                //     token.clone(),
                //     format!("variable '{name}' is not defined"),
                // ))
            }
        }
    }
    // Todo : Update PUT to reflect enclosing scopes
    fn put(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        // put is not allowed to create new keys or variable definitions, it can only update existing ones
        if !self.borrow().values.contains_key(name) {
            Err(RuntimeError::UndefinedVar(name.to_owned()))
        } else {
            self.borrow_mut().values.insert(name.to_owned(), value);
            Ok(())
        }
    }
}
