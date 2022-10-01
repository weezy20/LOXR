use crate::loc;
use crate::parser::error::{RuntimeError, EvalError};
use crate::parser::value::LoxFunction;
use crate::parser::{
    statement::Stmt,
    traits::evaluate::Evaluate,
    value::{Value, ValueResult},
    Parser,
};
use crate::tokenizer::token::Token;
use colored::Colorize;
use std::cell::RefCell;
use std::rc::Rc;
mod environment;
mod native_fn;
use native_fn::*;
pub use environment::Environment;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Interpreter {
    stmts: Vec<Stmt>,
    /// Fixed on the global execution context
    globals : Rc<RefCell<Environment>>,
    /// Tracks the current execution context
    env: Rc<RefCell<Environment>>,
    pub(crate) repl: bool,
    // index for repl mode
    previous: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        let global_env = Rc::new(RefCell::new(Environment::default()));
        global_env.define("clock", Value::Function(Rc::new(Clock)));
        Self { stmts: vec![], globals:Rc::clone(&global_env), env : global_env, repl: false, previous: 0 }
    }
}
pub trait Memory {
    fn define(&self, name: &str, value: Value);
    fn get(&self, name: &Token) -> Result<Option<Value>, RuntimeError>;
    fn put(&self, name: &str, value: Value) -> Result<(), RuntimeError>;
}

impl Interpreter {
    pub fn new(mut p: Parser) -> Self {
        let global_env = Rc::new(RefCell::new(Environment::default()));
        global_env.define("clock", Value::Function(Rc::new(Clock)));
        Self {
            stmts: p.parse(),
            globals : Rc::clone(&global_env),
            env : global_env,
            ..Default::default()
        }
    }
    /// Extend stmts with statements and also set Environment to `env`
    /// Currently used for tests only
    pub fn extend_with_env(&mut self, mut stmts: Vec<Stmt>, env: Rc<RefCell<Environment>>) {
        self.env = env;
        self.previous = self.stmts.len();
        self.stmts.append(&mut stmts);
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
    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        sub_env: Rc<RefCell<Environment>>,
        inside_loop: bool
    ) -> ValueResult {
        for stmt in statements.iter() {
            // check if a statement is a loop, if yes, set `inside_loop`
            let loop_stmt = if matches!(stmt, Stmt::While { .. }) {
                true
            } else { false };
            match self.execute(&stmt, Rc::clone(&sub_env), loop_stmt || inside_loop) {
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
    pub fn execute(&mut self, stmt: &Stmt, rc_env: Rc<RefCell<Environment>>, inside_loop: bool) -> ValueResult {
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
                            let _a = e.eval(&rc_env, self);
                            if _a.is_ok() && !self.repl { 
                                Ok(Value::Nil) }
                            else { _a }
                        },
                        _ =>  e.eval(&rc_env, self)
                    }                                        
            }
            Stmt::Print(x) => x.eval(&Rc::clone(&rc_env), self),
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
                let condition_value = condition.eval(&Rc::clone(&rc_env),self)?;
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
                // BUG : ASsertions fail when while is inside a scope
                assert!(inside_loop);
                assert!(loop_env.borrow().in_loop());
                while condition.eval(&Rc::clone(&rc_env),self)?.is_truthy() {
                    val = self.execute(&body.as_ref(), Rc::clone(&loop_env), true)?;
                    if matches!(val, Value::Break) {
                        return Ok(Default::default());
                    }
                }
                Ok(val)
            },
            Stmt::VarDecl { name, initializer } => {
                // let init_err : Option<EvalError> = None;
                let val = if let Some(expr) = initializer {
                    match expr.eval(&mut Rc::clone(&rc_env),self) {
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
            Stmt::FunDecl { ident, params, body } => {
                let stack_env = Rc::new(inside_env);
                let mut fn_params = vec![];
                for param in params {
                    if let Some(ident) = param.to_ident() {
                        stack_env.define(ident, Value::Nil);
                        fn_params.push(ident.to_owned());
                    }
                }
                let lox_fn = LoxFunction { stack_env , ident: ident.to_owned(), arity: params.len(), body : body.clone(), params : fn_params};
                rc_env.define(&ident.lexeme, Value::Function(Rc::new(lox_fn)));
                println!("fn declared <{}>", ident.lexeme);
                Ok(Value::Nil)
            },
        }
    }
    pub fn interpret(&mut self) -> () {
        let mut stmts = self.stmts.clone();
        for stmt in stmts.iter_mut() {
            let val: ValueResult = match stmt {
                // top level expr statements should be executed in global scope
                expr_stmt @ Stmt::ExprStmt(_) => self.execute(expr_stmt, Rc::clone(&self.env), false),
                    Stmt::Print(e) => e.eval(&Rc::clone(&self.env),self),
                    Stmt::ErrStmt { message } => {
                        loc!("Err stmt was printed");
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
                        match expr.eval(&Rc::clone(&self.env),self) {
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
                fn_decl @ Stmt::FunDecl { .. } => self.execute(fn_decl, Rc::clone(&self.env), false),
                
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
