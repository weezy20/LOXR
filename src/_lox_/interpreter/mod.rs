use crate::loc;
use crate::parser::error::RuntimeError;
use crate::parser::{
    statement::Stmt,
    traits::evaluate::{Evaluate, ValueResult},
    value::Value,
    Parser,
};
use crate::tokenizer::token::Token;
use colored::Colorize;
use std::cell::RefCell;
use std::rc::Rc;
mod environment;
pub use environment::Environment;

#[derive(Default, Debug)]
pub struct Interpreter {
    stmts: Vec<Stmt>,
    env: Rc<RefCell<Environment>>,
    pub(crate) repl: bool,
    // index for repl mode
    previous: usize,
}

pub trait Memory {
    fn define(&self, name: &str, value: Value);
    fn get(&self, name: &Token) -> Result<Option<Value>, RuntimeError>;
    fn put(&self, name: &str, value: Value) -> Result<(), RuntimeError>;
}

impl Interpreter {
    pub fn new(mut p: Parser) -> Self {
        Self {
            stmts: p.parse(),
            ..Default::default()
        }
    }
    /// Extend with env
    pub fn extend_with_env(&mut self, mut p: Parser, env: Rc<RefCell<Environment>>) {
        self.env = env;
        self.previous = self.stmts.len();
        self.stmts.append(&mut p.parse());
        loc!(format!("Interpreter modified -> {self:?}"));
        self.interpret();
    }
    /// Extend a repl interpreter and interpret the added stmts
    pub fn extend(&mut self, mut p: Parser) {
        assert!(
            self.repl,
            "ICE : Extend can only be called on repl mode, call interpret() instead"
        );
        self.previous = self.stmts.len();
        self.stmts.append(&mut p.parse());
        loc!(format!("Interpreter modified -> {self:?}"));
        self.interpret();
        // if self.is_repl_mode ? then for stmt in self.stmts[self.previous..].iter() { .. }
    }
    /// Execute a block of statements inside environment `sub_env`
    fn execute_block(
        &self,
        statements: &Vec<Stmt>,
        sub_env: Rc<RefCell<Environment>>,
    ) -> ValueResult {
        for stmt in statements.iter() {
            match self.execute(stmt, Rc::clone(&sub_env)) {
                Ok(val) => {
                    if val != Value::Nil {
                        println!(">> {}", val);
                    }
                }
                Err(e) => {
                    loc!();
                    eprintln!("{} {e}", "Interpreter Error:".red());
                }
            };
        }
        Ok(Value::Nil)
    }
    /// Execute a statement inside a new environment `rc_env`
    fn execute(&self, stmt: &Stmt, rc_env: Rc<RefCell<Environment>>) -> ValueResult {
        // Since our Rc is already "owned" by enclosing functions, we cannot safely deref_mut it
        // But in a single threaded context this will be safe
        // let env: &mut Environment = unsafe { Rc::get_mut_unchecked(&mut rc_env) };
        // let env: &mut Environment = &mut rc_env.borrow_mut(); // not needed after impl Memory for Rc<RefCell<Environment>>
        match stmt {
            Stmt::ExprStmt(x) | Stmt::Print(x) => x.eval(&mut Rc::clone(&rc_env)),
            Stmt::ErrStmt { message } => {
                loc!();
                eprintln!(
                    "{}{}{message}",
                    "Interpreter Error: ".red(),
                    "Bad statement ".yellow()
                );
                Ok(Value::Nil)
            }
            Stmt::Empty => Ok(Value::Nil),
            // Create a new environment
            Stmt::Block(stmts) => self.execute_block(
                stmts,
                Rc::new(RefCell::new(Environment::enclosed_by(Rc::clone(&rc_env)))),
            ),
            _ifstmt @ Stmt::IfStmt {
                condition,
                then_,
                else_,
            } => {
                // println!(" Got a {_ifstmt}");
                let condition_value = condition.eval(&mut Rc::clone(&rc_env))?;
                // create a new environment
                let if_else = Rc::new(RefCell::new(Environment::enclosed_by(Rc::clone(&rc_env))));
                let mut val = Value::Nil;
                if condition_value.is_truthy() {
                    val = self.execute(then_.as_ref(), if_else)?;
                }
                else if let Some(else_branch) = else_ {
                    val = self.execute(else_branch, if_else)?;
                }
                Ok(val)
            }
            Stmt::While { condition, body } => {
                let mut val = Value::Nil;
                let loop_env = Rc::new(RefCell::new(Environment::enclosed_by(Rc::clone(&rc_env))));
                while condition.eval(&Rc::clone(&rc_env))?.is_truthy() {
                    val = self.execute(&body.as_ref(), Rc::clone(&loop_env))?;
                }
                Ok(val)
            },
            Stmt::VarDecl { name, initializer } => {
                // let init_err : Option<EvalError> = None;
                let val = if let Some(expr) = initializer {
                    match expr.eval(&mut Rc::clone(&rc_env)) {
                        Ok(v) => v,
                        Err(eval_err) => {
                            loc!();
                            eprintln!("{} {eval_err}", "Interpreter Error:".red());
                            return Err(eval_err);
                        }
                    }
                } else {
                    Value::Nil
                };
                println!("var {name} declared to {}", val);
                rc_env.define(name, val);
                crate::loc!(format!("{:?}", self.env.borrow().values));
                Ok(Value::Nil)
            }
        }
        // Ok(Value::Nil)
    }
    pub fn interpret(&mut self) -> () {
        for stmt in self.stmts[self.previous..].iter() {
            let val: ValueResult = match stmt {
                    Stmt::ExprStmt(e) | Stmt::Print(e) => {
                        // e.eval(Rc::get_mut(&mut self.env).expect("ICE: obtain &mut Env"))
                        e.eval(&mut Rc::clone(&self.env))
                    }
                    Stmt::ErrStmt { message } => {
                        loc!();
                        eprintln!(
                            "{}{}{message}",
                            "Interpreter Error: ".red(),
                            "Bad statement ".yellow()
                        );
                        Ok(Value::Nil)
                    }
                    Stmt::Empty => Ok(Value::Nil),
                    Stmt::Block(scoped_stmts) => self.execute_block(
                        scoped_stmts,
                        Rc::new(RefCell::new(Environment::enclosed_by(Rc::clone(&self.env)))),
                    ),
                    // fancy @ syntax
                    ifstmt @ Stmt::IfStmt {
                        condition: _,
                        then_: _,
                        else_: _,
                    } => {
                        self.execute(&ifstmt, Rc::clone(&self.env))
                    }
                ,
                // Declarations should produce no values
                Stmt::VarDecl { name, initializer } => {
                    // let init_err : Option<EvalError> = None;
                    let val = if let Some(expr) = initializer {
                        match expr.eval(&mut Rc::clone(&self.env)) {
                            Ok(v) => v,
                            Err(eval_err) => {
                                loc!();
                                eprintln!("{} {eval_err}", "Interpreter Error:".red());
                                continue;
                            }
                        }
                    } else {
                        Value::Nil
                    };
                    println!("var {name} declared to {}", val);
                    self.env.define(name, val);
                    crate::loc!(format!("{:?}", self.env.borrow().values));
                    Ok(Value::Nil)
                }
                while_stmt @ Stmt::While { condition: _, body: _ } => {
                    self.execute(&while_stmt, Rc::clone(&self.env))
                },
                
            };
            match val {
                Ok(val) => {
                    if val != Value::Nil {
                        println!(">> {}", val);
                    }
                }
                Err(e) => {
                    loc!();
                    eprintln!("{} {e}", "Interpreter Error:".red());
                }
            };
        }
    }
}
