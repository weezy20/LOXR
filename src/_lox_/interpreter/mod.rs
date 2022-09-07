use crate::loc;
use crate::parser::error::{RuntimeError, EvalError};
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
    /// Extend stmts with parser `p` and also set Environment to `env`
    /// Currently used for tests only
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
        inside_loop: bool
    ) -> ValueResult {
        for stmt in statements.iter() {
            match self.execute(stmt, Rc::clone(&sub_env), inside_loop) {
                Ok(val) if matches!(val, Value::Break) => {
                    // Early return
                    return Ok(Value::Break);
                }
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
    fn execute(&self, stmt: &Stmt, rc_env: Rc<RefCell<Environment>>, inside_loop: bool) -> ValueResult {
        // Create a new environment surrounded by rc_env
        let inside_env = RefCell::new(if inside_loop {
            Environment::loop_enclosed_by(Rc::clone(&rc_env))
        } else {
            Environment::enclosed_by(Rc::clone(&rc_env))
        });
        match stmt {
            Stmt::ExprStmt(e) => {
                    match **e {
                        crate::parser::expressions::Expression::Assignment(_)
                        | crate::parser::expressions::Expression::Variable(_) => {
                            // println!("(EXECUTE)FOUND ASSIGNMENT OR VAR");
                            let _a = e.eval(&rc_env);
                            if _a.is_ok() && !self.repl { 
                                Ok(Value::Nil) }
                            else { _a }
                        },
                        _ =>  e.eval(&rc_env)
                    }                                        
            }
            Stmt::Print(x) => x.eval(&Rc::clone(&rc_env)),
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
                Rc::new(inside_env), inside_loop
            ),
            _ifstmt @ Stmt::IfStmt {
                condition,
                then_,
                else_,
            } => {
                // println!(" Got a {_ifstmt}");
                // Exec the condition in current env
                let condition_value = condition.eval(&Rc::clone(&rc_env))?;
                // create a new environment
                let if_else = Rc::new(inside_env);
                let mut val = Value::Nil;
                if condition_value.is_truthy() {
                    val = self.execute(then_.as_ref(), if_else, inside_loop)?;
                }
                else if let Some(else_branch) = else_ {
                    val = self.execute(else_branch, if_else, inside_loop)?;
                }
                Ok(val)
            }
            Stmt::While { condition, body } => {
                let mut val = Value::Nil;
                let loop_env = Rc::new(inside_env);
                assert!(inside_loop);
                assert!(loop_env.borrow().in_loop());
                while condition.eval(&Rc::clone(&rc_env))?.is_truthy() {
                    val = self.execute(&body.as_ref(), Rc::clone(&loop_env), inside_loop)?;
                    if matches!(val, Value::Break) {
                        return Ok(Default::default());
                    }
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
            Stmt::Break => if !inside_loop {
                Err(EvalError::BreakWithout)
            } else {
                Ok(Value::Break)
            },
        }
        // Ok(Value::Nil)
    }
    pub fn interpret(&mut self) -> () {
        for stmt in self.stmts[self.previous..].iter() {
            let val: ValueResult = match stmt {
                // top level expr statements should be executed in global scope
                expr_stmt @ Stmt::ExprStmt(_) => self.execute(expr_stmt, Rc::clone(&self.env), false),
                    Stmt::Print(e) => e.eval(&Rc::clone(&self.env)),
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
                        false
                    ),
                    // fancy @ syntax
                    ifstmt @ Stmt::IfStmt {
                        condition: _,
                        then_: _,
                        else_: _,
                    } => {
                        self.execute(&ifstmt, Rc::clone(&self.env), false)
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
                    self.execute(&while_stmt, Rc::clone(&self.env), true)
                },
                Stmt::Break => {
                    Err(EvalError::BreakWithout)
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
