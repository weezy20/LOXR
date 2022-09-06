//! Parser grammar:
//! program          → `statements`* EOF;
//! 
//! We may declare a variable or declare and assign the result of some expression to it
//! variableDecl     → "var" IDENTIFIER ("=" expression)? ";" ;
//! 
//! statement        → `variableDecl`| `exprStmt` | printStmt | block | ifstmt ;
//! exprStmt         → `expression` ";" ;
//! printStmt        → print `expression` ";" ;
//! block            → "{" declaration* "}" ;
//! ifStmt           → "if" "(" expression ")"  statement ("else" statement)? ;
//! 
//! A comma expression evaluates to the final expression
//! *comma expr*  → `expression , (expression)* | "(" expression ")"`;
//!
//! *expression*  → `ternary`;
//! 
//! *ternary*     → `assignment` | `assignment` ? `assignment` : `assignment`;
//! 
//! *assignment*  → `logic_or` | IDENTIFIER "=" `ternary`
//! 
//! *logic_or*    → `logic_and` ( "or" `logic_and`)* ;
//! 
//! *logic_and*   → `equality` ("and" `equality`)* ; 
//!
//! *equality*    → `comparsion ("==" | "!=" comparison)*;`
//!
//! *comparison*  → `term ("<="|"<"|">"|">=" term)*;`
//!
//! *term*        → `factor ("+"|"-" factor)*;`
//!
//! *factor*      → `unary (( "%" | "/" | "*" ) unary )*;`
//!
//! *unary*       → `("-" | "!") unary | primary;`
//!
//! *primary*     → `literal | identifier | "(" expression ")";`
//!
//! *literal*        → `NUMBER | STRING | "true" | "false" | "nil" ;`
//!
//! *grouping*       → `"(" expression ")" ;`
//!
//! *unary*          → `( "-" | "!" ) expression ;`
//!
//! *binary*         → `expression operator expression ;`
//!
//! *operator*       → `"==" | "!=" | "<" | "<=" | ">" | ">="
//!                  | "+"  | "-"  | "*" | "/" | "%";`
//!
//! Furthermore if we bake in the precedence rules it looks like this,
//! where top to bottom indicates the level of precedence of a given rule, top being matched the least
//! and bottom being matched the first:
//! 
//! Note on assignments, we would like to assign the result of a ternary op to a var
//! such as 
//! a = 1 < 2 ? 3 : 4; // a = 3 (Note the absence of keyword `var`, it's because this is an Assignment Expression)
//! 
//!


#[allow(unused_imports)]
use colored::Colorize;
use crate::parser::expressions::*;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType::{self, *};
use crate::loc;
use better_peekable::{BPeekable, BetterPeekable};
use expressions::Expression;
use std::vec::IntoIter;
use self::error::ParserError;
use self::statement::Stmt;

use crate::Lox;
/// ParserError
pub mod error;

/// Definition for Expression enum, and types that are Expression
pub mod traits;
/// Definition for a Lox value
pub mod value;
/// Expression types
pub mod expressions;
/// Statements
pub mod statement;


#[derive(Debug, Clone)]
// TODO : Add a (line, col) for syntax error reporting
pub struct Parser {
    tokens: BPeekable<IntoIter<Token>>,
    current: usize,
    previous: Option<Token>,
    error_production : Vec<Token>,
    parser_corrupt: bool,
}
/// In a recursive descent parser, the least priority rule is matched first
/// as we descend down into nested grammer rules
// Expression
impl Parser {
    pub fn parse_expression(&mut self) -> Result<Box<Expression>, ParserError> {
        self.comma_expression()
    }
     /// *comma expr* → `expression , (expression)* | "(" expression ")"`;
     pub fn comma_expression(&mut self) -> Result<Box<Expression>, ParserError> {
        let expr = self.expression()?;
        let mut expr_list: Vec<Box<Expression>> = vec![expr];
        while self.matches(&[COMMA] ) {
            let next = self.expression()?;
            expr_list.push(next);
        }
        if expr_list.len() > 1 {
            Ok(Box::new(Expression::CommaExpr(expr_list)))
        } else {
            Ok(expr_list.pop().unwrap())
        }
    }
    /// *expression*  → `ternary`
    pub fn expression(&mut self) -> Result<Box<Expression>, ParserError> {
        self.ternary()
    }
    /// *ternary* → `assignment` | `assignment` ? `assignment` : `assignment`;
    /// In C, the ternary conditional operator has higher precedence than assignment operators.
    pub fn ternary(&mut self) -> Result<Box<Expression>, ParserError> {
        let conditional_expr = self.assignment()?;
        // loc!(format!("ternary here with condition/left -> {conditional_expr}"));
        if self.matches(&[TERNARYC]) {
            let left_expr = self.expression()?;
            // loc!(format!("ternary here with left -> {left_expr}"));
            if self.matches(&[TERNARYE]) {
                let right_expr = self.expression()?;
                // loc!(format!("ternary here with right -> {right_expr}"));
                let t = Expression::TernExpr(TernaryExpr {
                    condition: conditional_expr,
                    if_true: left_expr,
                    if_false: right_expr,
                });
                // loc!(format!("Ternary formed -> {t}"));
                return Ok(Box::new(t));
            } // match TERNARYE
            return Err(ParserError::ExpectedExpression);
        } // match TERNARYC
        Ok(conditional_expr)
    }
    /// *assignment*  → `logic_or` | IDENTIFIER "=" ternary
    pub fn assignment(&mut self) -> Result<Box<Expression>, ParserError> {
        // `a = "value";` This is a deviation from the standard way of parsing exprs until now
        // where we would parse everything as an rval expression; we would match on the operator 
        // and finally parse the remaining as part of one single expression. Here, `a` is not an expression per se
        // rather, it's a reference to a symbol that may or may not exist when this line is being parsed
        // resulting in a RuntimeError/Parser error if the latter is the case
        //
        // Consider makeList().head.next = node;
        // Where assignment characteristic token `=` occurs after parsing multiple tokens like (), . , multiple idents etc.
        // therefore our strategy is to parse as an expression, until we get to a `=` symbol after which we start parsing the 
        // right as an rval and try an assignment operation. We use the lval as a storage location, if not, it's a parserError
        let expression : Box<Expression> = self.or()?;
        if self.matches(&[EQUAL]) {
            // Since this is entered on variable assignment renaming helps 
            // Since we have both if/else returns, we don't worry about moving into lval
            let lval = expression;
            let equal: Token = self
                .previous
                .take()
                .expect("matches will ensure this field to be something");
            let rval: Box<Expression> = self.expression()?; // allows for b = a = 2 which means a -> 2 and b -> 2
            // ensure lval is a Expression::Variable(_) and not something else : 
            if let Expression::Variable(ref t) = *lval {
                return Ok (
                    box Expression::Assignment(AssignmentExpr {
                        name: t.clone(),
                        right: rval, 
                    })
                )
            } else {
                Lox::report_syntax_err(equal.ln, equal.col, format!("{}", ParserError::InvalidAssignmentTarget));
                return Err(ParserError::InvalidAssignmentTarget);
            }
        }
        Ok(expression)
    }
    /// *logic_or*    → `logic_and` ( "or" `logic_and`)* ;
    pub fn or(&mut self) -> Result<Box<Expression>, ParserError> {
        let mut expr = self.and()?;
        while self.matches(&[OR]) {
            let operator = self.previous.take().expect("infallible");
            let right = self.and()?;
            expr = box Expression::LogicOr(OrExpr { left: expr, operator, right });
        }
        Ok(expr)
    }
    /// *logic_and*   → `equality` ("and" `equality`)* ; 
    pub fn and(&mut self) -> Result<Box<Expression>, ParserError> {
        let mut expr = self.equality()?;
        while self.matches(&[AND]) {
            let operator = self.previous.take().expect("infallible");
            let right = self.equality()?;
            expr = box Expression::LogicAnd(AndExpr { left: expr, operator, right });
        }
        Ok(expr)
    }
    /// *equality*    → `comparsion ("==" | "!=" comparison)*;`
    pub fn equality(&mut self) -> Result<Box<Expression>, ParserError> {
        // This creates a left associative nested tree of binary operator nodes
        // The previous `expr` becomes the new `left` of an equality expression if matches returns true
        
        let mut expr: Box<Expression> = match self.comparison() {
            Ok(expr) => expr,
            Err(_e) if self.error_production.len() > 0 => {
                let mut _had_error = false;
                 {
                    loc!();
                    eprintln!("Error productions in Parser cache : {:#?}", self.error_production);
                    _had_error = true;
                    // println!("Discarding Malformed expression:\n{expr:?}");
                    // let _ = Expression::Error(expr); // 
                    self.synchronize();
                    // Time to clear error cache
                    self.error_production.clear();
                    return self.comma_expression();
                }
            },
            Err(e) => return Err(e)
        }; 
        while self.matches(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator: Token = self
                .previous
                .take()
                .expect("matches will ensure this field to be something");
            let right = self.comparison()?;
            expr = Box::new(Expression::BinExpr(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }
    /// *comparison*  → `term ("<="|"<"|">"|">=" term)*;`
    pub fn comparison(&mut self) -> Result<Box<Expression>, ParserError> {
        let mut expr = self.term()?;
        while self.matches(&[LESS, LESS_EQUAL, GREATER, GREATER_EQUAL]) {
            let operator: Token = self
                .previous
                .take()
                // .clone()
                .expect("matches will ensure this field to be something");
            let right = self.term()?;
            expr = Box::new(Expression::BinExpr(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }
    /// *term*        → `factor ("+"|"-" factor)*;`
    pub fn term(&mut self) -> Result<Box<Expression>, ParserError> {
        let mut expr = self.factor()?;
        while self.matches(&[MINUS, PLUS]) {
            let operator: Token = self
            .previous
            .take()
            .expect("matches will ensure this field to be something");
            let right = self.factor()?;
            expr = Box::new(Expression::BinExpr(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }
    /// *factor*      → `unary (( "/" | "*" ) unary )*;`
    pub fn factor(&mut self) -> Result<Box<Expression>, ParserError> {
        // let mut expr = self.unary()?;
        // -- Adding an Error production for binary ops (missing left operand) -- 
        // We choose this location bcz this is the first location where a simple (i.e. non-nested) BinaryExpr may be produced
        // 1. An error production works like this: it fills in the gap caused by a missing left operand
        // 2. Then it proceeds with the parsing until an expression is complete
        // 3. Then it reports error, prints and discards this malformed expression, reports an error
        // This is done at top level binary expression production since we want to still parse the 
        // entire Binary Expression without the left operand, in our case `equality`
        // 4. Synchronizes the parser to next boundary and resume parsing as normal w/o entering panic mode
        let mut had_binary_expr_err = false;
        // #[allow(unused_assignments)]
        // let mut illegal_factor_token : Token = Token::default();
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(ParserError::InvalidToken(i)) => {
                let (mut counter, threshold) = (1, 10);
                had_binary_expr_err = true;
                // TODO: This code results in assymetric error reporting
                // for example `var x = 10-*;` produces a different error message than `var x = 10*-`
                report_token_error(&i);
                loop {
                    let maybe_valid = self.primary();
                    if let Err(ParserError::InvalidToken(ref i2)) = maybe_valid  
                    {
                        report_token_error(i2)
                    }
                    if maybe_valid.is_ok() { break maybe_valid?; }
                    counter += 1;    
                    if counter == threshold {return maybe_valid;}
                }
            },
            Err(e) => return Err(e),
        };
        while self.matches(&[STAR, SLASH, MODULUS]) {
            let operator: Token = self
            .previous
            .take()
            .expect("matches will ensure this field to be something");
            let right = self.unary()?;
            expr = Box::new(Expression::BinExpr(BinaryExpr::new(expr, operator, right)));
        }
        if had_binary_expr_err {
            println!("Recovering..............");
            // return Err(ParserError::ErrorProduction(expr));
        }
        Ok(expr)
    }
    /// *unary*       → `("-" | "!") unary | primary;`
    pub fn unary(&mut self) -> Result<Box<Expression>, ParserError> {
        if self.matches(&[MINUS, BANG]) {
            let operator: Token = self
            .previous
            .take()
            .expect("matches will ensure this field to be something");
            let right_expr = self.unary()?;
            return Ok(Box::new(Expression::UnExpr(
                UnaryExpr::new(operator, right_expr)
                .expect("Scanner should catch malformed unary expressions"),
            )));
        }
        self.primary()
    }
    /// *primary*     → `literal | "(" expression ")";`
    /// *literal*     → Number | String | "true" | "false" | "nil" ;
    pub fn primary(&mut self) -> Result<Box<Expression>, ParserError> {
        if self.matches(&[IDENTIFIER])
        {
            return Ok(box Expression::Variable(self.previous.take().expect("infallible")));
        }
        // "1+3+4(3+4)"
        if self.matches(&[FALSE, TRUE, NIL, NUMBER, STRING]) {
            // Previous is sure to exist if this branch is entered
            // Also constructing a literal is infallible at this stage
            let _p = self.previous.clone().expect("Previous should have something here");
            let x = self.peek().cloned();
            if let Some(peeked_token) = x {
                match peeked_token.r#type {
                    LEFT_PAREN | LEFT_BRACE | LEFT_SQUARE => {
                        Lox::report_syntax_err(
                            peeked_token.ln, 
                            peeked_token.col, 
                            format!("Unexpected token {peeked_token} after {_p}")
                        );
                        self.parser_corrupt = true;
                        self.error_production.push(self.previous.clone().expect("Matches will always be something"));
                        // return Err(ParserError::InvalidToken(Some(peeked_token)));
                    }
                    _ => {}
                }
            }
            Ok(Box::new(Expression::Lit(
                Literal::new(self.previous.take().unwrap()).unwrap(),
            )))
        } else if self.matches(&[LEFT_PAREN]) {
            let expr = self.expression()?;
            let _expect_right_paren = self.consume(RIGHT_PAREN)?;
            // This assertion should never fail
            assert!(_expect_right_paren.is_some());
            // .expect("Expect ')' after expression");
            Ok(Box::new(Expression::Group(Grouping::new(expr))))
        } else {
            // If there's going to be an illegal parse, it's going to be here
            self.parser_corrupt = true;
            // "Each token must be matched by now, if not, the parser may have not understand where the Token
            // fits into the grammar production after falling from expression upto token, in which case we have to write code
            // to handle that, or the Token is simply in the wrong place and a parser error should be reported "
            // panic!("Cannot parse as primary expression");
            if !self.is_at_end() && self.matches(&[PLUS, MINUS, SLASH, STAR, EQUAL_EQUAL, BANG_EQUAL, EQUAL, LESS, GREATER, LESS_EQUAL, GREATER_EQUAL]){
                // Capture multiple invalid tokens or operators appearing at start of expression
                self.error_production.push(self.previous.clone().expect("Matches will always be something"));
                // Don't worry, this error is caught in binary expression parser and it will recognize the error production
                // This err won't be propagated upto the top expression parser logic
                Err(ParserError::InvalidToken(self.previous.clone()))
            }
            // The next token is EOF and therefore we've run out of tokens to parse
            else {
                // self.is_at_end == true and a primary expression is being searched for, but since is_at_end == true,
                // the next token is EOF, and therefore the expression is ill-formed
                Err(ParserError::ExpectedExpression)
            }
        }
    }
}

fn report_token_error(i: &Option<Token>) {
    if let Some(invalid_token) = i {
        let message = format!("Invalid token: '{}' ,found at what appears to be the boundary of a Binary Expression", invalid_token.lexeme);
        Lox::report_syntax_err(invalid_token.ln, invalid_token.col, message);
    }
}
// Private helpers
impl Parser {
    /// Peeks the current token iterator for a match in the list of searchable token types passed to it.
    /// Advances the underlying iterator only on a match, i.e. increments the `current` field and consumes 
    /// the peeked token
    /// For instance in the comparison rule, we may want to check a multitude of tokentypes('<','<=',...) for a comparision,
    /// so we can pass all comparison operators in the searchable list and if we get a yes back from this function,
    /// it means that we must call the comparision rule again, otherwise we are done with comparison expressions and must
    /// "descend" down the grammar rule list to *term* and so on
    fn matches(&mut self, searchable_list: &[TokenType]) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(peeked_token) = self.tokens.peek() && searchable_list.contains(&peeked_token.r#type) {
            let _ = self.advance();
            return true;
        }
        false
    }
    /// Increment the `current` index and consume a token from the Parser's `tokens` list
    /// returning the token that was just consumed OR, in the case that we have reached EOF or
    /// an abrupt end of tokens in our `tokens` list, we just send the previous cached token
    /// More likely than not, this would be a None variant as we our expression parsing rules now
    /// `take()` instead of `clone()`. This does not matter as we are using this function internally.
    fn advance(&mut self) -> Option<Token> {
        if let Some(_) = self.tokens.peek() && !self.is_at_end() {
            self.current += 1;
            self.previous = self.tokens.next();
        }
        self.previous.clone()
    }
    fn is_at_end(&mut self) -> bool {
        if let Some(peeked_token) = self.tokens.peek() && peeked_token.r#type == EOF { return true;}
        false
    }
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    /// Consume the token if & only if it matches the `expected_token` and return it, otherwise report an error,
    /// and return a `ParserError`. 
    fn consume(
        &mut self,
        expected_token: TokenType,
    ) -> Result<Option<Token>, ParserError> {

        if let Some(peeked_token) = self.tokens.peek() && expected_token == peeked_token.r#type {
            return Ok(self.advance());
        }
        else if let Some(peeked_token) = self.tokens.peek() && peeked_token.r#type != EOF { 
            Lox::report_syntax_err(peeked_token.ln, peeked_token.col, format!("Invalid Token: {peeked_token} encountered\nExpected {expected_token:#?}") );
            loc!();
            Err(ParserError::InvalidToken(self.tokens.peek().cloned()))
        } 
        // None is peeked that means we are at EOF
        else {
            // self.previous is guaranteed to exist at this point because we haven't formed an expression yet
            // and we are only peeking ahead to check if the right token follows. If this contract is violated it's a bug
            // and should be reported as a interpreter/compiler internal error
            // assert!(self.previous.is_some(), "Internal Lox Error, expected parser.previous to be Some(_) found None");
            // self.previous may or may not exist as we have started replacing `clone` calls with `take` calls in various rules
            // Which means we cannot rely on the following code for peeked_token anymore
            // let peeked_token = self.previous.clone().unwrap();
            // We should enter this condition
            if let Some(peeked_token) = self.tokens.peek() && peeked_token.r#type == EOF {
                // This should report EOF in the error msg
                loc!();
                Lox::report_syntax_err(peeked_token.ln, peeked_token.col, format!("Unexpected end of file, found {:#?}, expected `{expected_token:?}`", peeked_token.r#type));
                return Err(ParserError::UnexpectedEOF);
            }
            loc!();
            Err(ParserError::ExpectedExpression)
        }
    }
    /// This function is called in the event of a `ParserError`. Handlers of `ParserError` can call this function
    /// to discard the current erroneous Token stream until a synchronization boundary is met. In our case we are using
    /// a `Statement` or Semicolon as a synchronization boundary because it's easy to spot.
    /// Most statements start with `for`, `if`, `return`, `var` etc so we can use this info to mark a synchronization boundary.
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            // After a semicolon, a Statement ends
            if let Some(previous_token) = &self.previous && previous_token.r#type == SEMICOLON {
                return;
            }
            if let Some(token) = self.peek() {
                match token.r#type {
                    // Keywords that mark the beginning of a new Statement
                   CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => 
                   {
                    return;
                   }
                   _ => {}
                }
            }
            self.advance();
        }
    }
}
// Statement parsing
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().better_peekable(),
            current: 0_usize,
            previous: None,
            error_production: vec![],
            parser_corrupt: false,
        }
    }
    /// Parse as an expression
    pub fn run(&mut self) -> Result<Box<Expression>, ParserError> {
        self.parse_expression()
    }
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while !self.is_at_end() {
            stmts.push(self.collect());
            // BUG_FIXED: If var ? or an ErrDecl is returned, this loop never ends
            // BUG_FIXED: Doesn't synchronize on multiline comments
            // BUG_FIXED : Infinte loop on char
            loc!(format!("{} statements : {:?}", stmts.len() , stmts));
        }
        stmts
    }
    // TODO: Transform all statement methods to return a Result
    /// Parse as a variable declaration or else a statment
    fn collect(&mut self) -> Stmt {
        // When panic, call self.synchronize()
        // Declarations can be either a VarDecl or a normal Statement, 
        // we decide that here: 
        if self.matches(&[VAR]) {
            match self.var_declaration() {
                Ok(d) => d,
                Err(err) => { 
                    loc!(format!("Declaration parsing error : {}{}","Parser Error ".bright_cyan(), err));
                    let d = err.into(); // to leverage type inference for the following macro
                    loc!(d);
                    d // due to this rust can infer the type and use it in the above macro
                },
            }
        } else {
            self.statement().into()
        }
    }
    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        if self.matches(&[IDENTIFIER])  {
            let name_token = self.previous.take().expect("matches is infallible");
            let name = name_token.lexeme;
            // Variable decl and init
            if self.matches(&[EQUAL]) {
                let initializer = self.expression()?;
                self.consume(SEMICOLON)?;
                let _equal = self.previous.take().expect("Safe to unwrap here");                
                Ok(Stmt::VarDecl{ name, initializer: Some(initializer) })
            } 
            // Variable declaration without initialization
            else {
                self.consume(SEMICOLON)?;
                Ok(Stmt::VarDecl{ name, initializer: None })
            }
        }   
        else {
           self.synchronize();
           Err(ParserError::IllegalStmt(Some("Missing variable identifer".into())))
        }
    }
    /// Parse as a statement, converting ParserErrors into ErrStmt enclosing the ParserError
    fn statement(&mut self) -> Stmt {
        if self.matches(&[COMMENT, MULTI_LINE_COMMENT]) {
            return Stmt::Empty;
        }
        let stmt = if self.matches(&[PRINT]) {
            self.print_statement()
        } else if self.matches(&[LEFT_BRACE]) {
            self.block_statement()
        }
        else if self.matches(&[IF]){
            self.if_statement()
        }
        else if self.matches(&[WHILE]) {
            self.while_statement()
        }
        else {
            self.expression_statement()
        };
        match stmt {
            Ok(s) => s,
            Err(err) => {
                self.synchronize();
                err.into()
            },
        }
    }
    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(LEFT_PAREN)?;
        let condition = self.expression()?;
        loc!(format!("if condition -> {}", &condition));
        self.consume(RIGHT_PAREN)?;
        let body = box self.collect();
        Ok(Stmt::While { condition, body })
    }
    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(LEFT_PAREN)?;
        let condition = self.expression()?;
        loc!(format!("if condition -> {}", &condition));
        self.consume(RIGHT_PAREN)?;
        // let then = self.collect();
        let then_ = box self.collect();
        loc!(format!("then branch -> {}", *then_));
        let mut else_ = None;
        // This `else` is bound to the nearest if statement
        if self.matches(&[ELSE]) {
            else_ = Some(box self.collect());
            loc!(format!("else branch -> {}", else_.as_ref().unwrap()));
        }
        Ok(Stmt::IfStmt { condition, then_, else_ })

    }
    // We are not making use of Err(ParserError) yet, and just return Ok(ErrStmt) instead
    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let val = self.parse_expression()?;
        // println!("print statement - > {}", val);
        self.consume(SEMICOLON)?;
        Ok(Stmt::Print(val))
    }
    // We are not making use of Err(ParserError) yet, and just return Ok(ErrStmt) instead
    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {     
        let val = self.parse_expression()?;
        // TODO: Errors on EOF not preceded by semicolon, should we error?
        self.consume(SEMICOLON)?;
        Ok(Stmt::ExprStmt(val))
    }
    fn block_statement(&mut self) -> Result<Stmt, ParserError> {     
        Ok(Stmt::Block(self.block()?))
    }
    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut block_stmts: Vec<Stmt> = vec![];
        while let Some(x) = self.peek() && x.r#type != RIGHT_BRACE && !self.is_at_end() {
            block_stmts.push(self.collect());
        } 
        self.consume(RIGHT_BRACE)?;
        loc!("Block parsed successfully");
        Ok(block_stmts)
    }
}