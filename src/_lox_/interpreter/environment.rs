use super::Memory;
use crate::{
    parser::{error::RuntimeError, value::Value},
    tokenizer::token::Token,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// An environment for executing [Statements](crate::parser::statement::Declaration)s
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
    pub fn enclosed_by(enclosing: Rc<RefCell<Environment>>) -> Self {
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
    fn define(&self, name: &str, value: Value) {
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
            }
        }
    }
    /// put is not allowed to create new keys or variable definitions, it can only update existing ones
    fn put(&self, name: &str, value: Value) -> Result<(), RuntimeError> {
        println!("PUT called with {name} and {value}");
        if !self.borrow().values.contains_key(name) {
            let current_env: Rc<RefCell<Environment>> = Rc::clone(&self);
            let mut previous: Rc<RefCell<Environment>> = Default::default();
            // if self doesn't contain key, we must, changed from false -> true
            let mut upgrade_scope = 0;
            loop {
                // println!("PUT LOOP, current now {:?}", current_env.borrow().values.keys());
                // This code is skipped on first iter where instead we would like to upgrade_scope
                if upgrade_scope >= 1 {
                    // Only swap if switch is set, this will ensure previous = encl_env rather than the default value
                    // println!(
                    //     "before swap, previous {:?} :  current  {:?}",
                    //     previous.borrow().values.keys(),
                    //     current_env.borrow().values.keys()
                    // );
                    // previous must be initialized with encl_env (enclosing scope of current_env)
                    current_env.swap(&previous);
                    // Current_env now points to what it had as enclosing_env
                    // println!(
                    //     "after swap, previous {:?} :  current  {:?}",
                    //     previous.borrow().values.keys(),
                    //     current_env.borrow().values.keys()
                    // );
                }
                if let Some(ref encl_env) = current_env.borrow().enclosing {
                    previous = Rc::clone(encl_env);
                    if upgrade_scope == 0 {
                        // On first iteration, upgrade_scope == false, and we must check the enclosing scope
                        // for the variable, so it makes sense to run (if upgrade_scope) code block
                        // before we proceed
                        upgrade_scope = 1;
                        continue;
                    }
                    if encl_env.borrow().values.contains_key(name) {
                        encl_env.borrow_mut().values.insert(name.to_owned(), value);
                        return Ok(());
                    }
                    // code block never entered
                    else {
                        upgrade_scope += upgrade_scope; // next loop iteration swap previous with current
                        previous = Rc::clone(&encl_env);
                        // current_env.swap(encl_env); // panics becz encl env is from current_env.borrow()
                        // let x = current_env.borrow_mut(); // also panics, same reasons
                        continue;
                    }
                }
                // Upto this we have not found the var declared
                // else { //removed else as the previous if either returns or restarts the loop
                // the if let statement borrows() and therefore that Ref lives well into the else branch
                // that was causing our borrow_mut() to panic

                // No enclosing environment, current_env is global env
                assert!(
                    current_env.borrow().is_global,
                    "ICE: Current env expected to be global at this point"
                );
                // let contains = current_env.borrow().values.contains_key(name);

                let mut env_borrow = current_env
                    .try_borrow_mut()
                    .expect("ICE: cannot borrow as mut");
                if env_borrow.values.contains_key(name) {
                    env_borrow.values.insert(name.to_owned(), value);
                    return Ok(());
                } else {
                    return Err(RuntimeError::UndefinedVar(name.to_owned()));
                }
                // } //removed else
            }
            // Err(RuntimeError::UndefinedVar(name.to_owned()))
        } else {
            self.borrow_mut().values.insert(name.to_owned(), value);
            Ok(())
        }
    }
}
