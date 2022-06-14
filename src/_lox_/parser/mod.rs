//! The way we address the expression problem here, which briefly states, that due to the way languages are designed
//! certain operations are difficult without modifying pre-existing code. In the OOP paradigm, a class contains behaviour bundled with it, but let's say
//! we were introducing a new behaviour across all our types, that would imply modifying code from each of our pre-existing
//! types to include that behaviour, uniquely implemented for each class. This goes against two principles that we'd like
//! to uphold when writing scalable software, the first being the open-close principle which briefly states that scalable
//! software should be open to extension but closed to modification. Another problem with bundling behaviour with the class
//! is violation of concerns. A tree node shouldn't have methods pertaining specifically either to the parser where it is
//! produced or the interpreter where it is consumed. This leads to a violation of separation of concerns where
//! two domains are stepping on each other's toes by smushing interpreter and parser specific logic in the same location
//! which is the class definition.
//!
//! In the functional programming paradigm, new operations are easy to add, and types are mostly inert, with no behaviour defined on them. Instead, behaviour, known as functions, pattern match on the type that's passed to them and then perform the appropriate action
//! This has a downside, because first, if you were to add a new operation, it's adheres to extensibility as you can
//! add code, in a function, and then write it for each pre-existing type. But what happens when you have to add
//! new type? That would imply going back to each function's code, which remember, we must treat as untouchable, and modify
//! it violating our contract with the open-close principle.
//!  It would be nice if we could just add new types like OOP paradigm, by bundling the implementation of a each behaviour while defining our type, and
//! it would also be desirable to include new operations like the FP way but without having to modify any pre-existing behaviour code
//!
//! In Rust, which borrows some ideas from FP, behavriour is decoupled from structs and enums, and we are free
//! to choose traits that we'd want on our types
//!
//! ```rust
//! trait NewOp {
//!     fn new_op();
//! }
//!
//! // Then implement this trait for each of the pre-existing types
//! impl NewOp for Expr { .. }
//! impl NewOp for Stmt { .. }
//! ```
//!
//! If we have to add a new type, adding structs is trivial, but what if we want something like an
//! enum to pattern match on them? This would be going against the OC principle, as we would be required to
//! *modify* pre-existing enum code, but there's a workaround for it:
//! ```rust
//! pub enum ExtendExpression {
//!     NewExpr,
//!     OldExpr(Expression)
//! }
//!
//! impl ExtendExpression {
//!     fn dispatch(&self) {
//!       match self {
//!             OldExpr(E) => todo!(),
//!             NewExpr => todo!(),
//!         }  
//!     }
//! }
//! ```
//!
//! A third way would be to have no enums at all, just structs for each type of expression or statements,
//! like so:
//! ```rust
//! struct BinaryExpr;
//! struct Literal;
//! struct Grouping;
//! ```
//! The problem however is types like `BinaryExpr` require references to other kinds of `Expression`.
//!
//! If we were writing an enum Expression, it would be trivial to write
//! ```rust
//! struct BinaryExpr {
//!     left : Box<Expression>
//!     right : Box<Expression>
//! }
//! ```
//! But as we discussed, this brings us back to having a closed type, `Expression` which cannot be extended without
//! a wrapper Enum. The solution could be trait objects. We could instead have a `trait Expression`
//! ```rust
//! pub trait Expression {
//!     fn eval();
//!     fn pretty_print();
//! }
//!
//! struct BinaryExpr {
//!     left : Box<dyn Expression>
//!     right : Box<dyn Expression>
//! }
//! ```
//! But this comes with it's own problems regarding modification. Consider now adding a new operation. We would be required
//! to go back to this trait definition, add the new operation, then modifying the impl block for each type that implements
//! Expression trait, and modify them in turn. A nightmare for open-close principle.
//!
//! We could however, keep `Expression` empty, only using it as a placeholder in places like `left` and `right` fields
//! of struct `BinaryExpr`, and then, when adding new operations, write a trait as an operation, that is a sub-trait of Expression
//!
//! ```rust
//! /// A trait that prints, used by parsers for debug printing
//! pub trait Printer : Expression {
//!     fn print();
//! }
//!
//! /// A trait that evaluates an expression to a literal value
//! pub trait Evaluate : Expression {
//!     fn eval();
//! }
//! ```
//!
//! And then implement these traits on our structs. This will probably take us to generic land as we would have to
//! make use of trait bounds in order to make sure that our structs can actually `eval()` or `print()` if they nest
//! other expression types.
//!
//! Consider this
//!
//! ```rust
//! pub struct BinaryExpr<L, R>
//! where
//!     L: Expression,
//!     R: Expression,
//! {
//!     left: L,
//!     right: R,
//! }
//! ```
//!
//! Is this any better? Maybe. Our initial problem was that we needed two types of abstraction,
//! one for the types, which would allow nesting types within types like `BinaryExpr` and `Grouping`
//! and the other for behaviour decoupled from each other;
//! In a way that would allow us to extend behaviour without worrying about modifying existing type or method definitions
//! and also to allow adding types without requiring to modify existing code. Rust by default with traits, addresses the first problem.
//!
//! The second problem was that if we used an enum to define an overarching type for expression, adding types
//! would require modifying this enum. This is where generics come in, providing us the much needed abstraction, without
//! having to modify existing code. Consider a trait `Expression`
//! As it stands now, Rust allows us to easily expand behaviour through traits, but to extend types, especially types
//! that share the "class", we would have to resort to enums, wrapped enums in the case of extending types, and if we
//! don't want enums, trait objects is another option, which gets unwieldy due to downcasting required to do anything
//! useful, defeating the purpose of having an abstraction in the first place.
//!
//! However, generics, in combination with traits, provide, in my experience, a very solid abstraction that can abstract
//! over types like expressions, and make it easy to solve the expression problem. How this works out in practice, we
//! will have to find out as we implement the parser.
//!
//! ```rust
//! // Consider adding a new type
//! pub struct NewExprType {}
//! // Now let's see if this design upholds OC principle:
//! // Adding this type to the class of Expression just invovles implementing `trait Expression`
//! impl Expression for NewExprType {}
//! // Now consider the definition of `BinaryExpr`
//! pub struct BinaryExpr<L, R>
//! where
//!     L: Expression,
//!     R: Expression,
//! {
//!     left: L,
//!     right: R,
//! }
//!
//! // Will this struct construct with our NewExprType? Of course.
//! let b : BinaryExpr<NewExprType, NewExprType> =
//!     BinaryExpr { left : NewExprType {}, right: NewExprType {} };
//! // Now suppose a trait `Eval` exists on `BinaryExpr`
//! pub trait Eval : Expression {
//!     fn eval(&self) -> f32;
//! }
//! // note the trait bound of `Expression`, every `Eval` must be an `Expression`
//! // But not all `Expression` is `Eval`
//! // Let's implement `Eval` for our new type
//! impl Eval for NewExprType {
//!     fn eval(&self) -> f32 { 42.0 }
//! }
//!
//! // Let's see the implementation for BinaryExpr
//! // All that's required for this eval, is the inner generic types are also `Eval`
//! impl<L,R> Eval for BinaryExpr<L,R>
//! where
//!     L: Eval
//!     R: Eval
//! {
//!     fn eval(&self) -> f32 {
//!        // let's just add for now
//!        self.left.eval() + self.right.eval()
//!     }
//! }
//! ```
//! So adding new types was EASY using generics and traits for type abstraction
//! and adheres to OC principle. The only problem that I can see with this approach is when implementing a new operation
//! say trait `OP` requires the inner generics to be bounded by something other than `OP + Expression`, in that case
//! the generics trait bound won't be satisfied and it wouldn't be possible to implement the operation. But at this point
//! It's good enough to warrant it's own development branch
//!

use crate::_lox_::parser::expressions::*;
use crate::_lox_::tokenizer::token::Token;
use crate::_lox_::tokenizer::token_type::TokenType::{self, *};
use better_peekable::{BPeekable, BetterPeekable};
use expressions::Expression;
use std::vec::IntoIter;

use self::error::ParserError;

use super::lox::Lox;
/// ParserError
pub mod error;

/// Definition for Expression enum, and types that are Expression
pub mod traits;

/// Expression types
pub mod expressions;

/// Parser grammar:
///
/// *expression*     → `literal
///                  | unary
///                  | binary
///                  | grouping ;`
///
/// *literal*        → `NUMBER | STRING | "true" | "false" | "nil" ;`
///
/// *grouping*       → `"(" expression ")" ;`
///
/// *unary*          → `( "-" | "!" ) expression ;`
///
/// *binary*         → `expression operator expression ;`
///
/// *operator*       → `"==" | "!=" | "<" | "<=" | ">" | ">="
///                  | "+"  | "-"  | "*" | "/" ;`
///
/// Furthermore if we bake in the precedence rules it looks like this,
/// where top to bottom indicates the level of precedence of a given rule, top being matched the least
/// and bottom being matched the first:
///
/// *expression*  → `equality`
///
/// *equality*    → `comparsion ("==" | "!=" comparison)*;`
///
/// *comparison*  → `term ("<="|"<"|">"|">=" term)*;`
///
/// *term*        → `factor ("+"|"-" factor)*;`
///
/// *factor*      → `unary (( "/" | "*" ) unary )*;`
///
/// *unary*       → `("-" | "!") unary | primary;`
///
/// *primary*     → `literal | "(" expression ")";`

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: BPeekable<IntoIter<Token>>,
    current: usize,
    previous: Option<Token>,
}
/// In a recursive descent parser, the least priority rule is matched first
/// as we descend down into nested grammer rules
impl Parser {
    /// *expression*  → `equality`
    pub fn expression(&mut self) -> Result<Box<Expression>, ParserError> {
        self.equality()
    }
    /// *equality*    → `comparsion ("==" | "!=" comparison)*;`
    pub fn equality(&mut self) -> Result<Box<Expression>, ParserError> {
        // This creates a left associative nested tree of binary operator nodes
        // The previous `expr` becomes the new `left` of an equality expression if matches returns true
        let mut expr = self.comparison()?;
        while self.matches(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let operator: Token = self
                .previous
                .take()
                .expect("matches will ensure this field to be something");
            let right = self.comparison()?;
            expr = Box::new(Expression::BinExp(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }
    /// *comparison*  → `term ("<="|"<"|">"|">=" term)*;`
    pub fn comparison(&mut self) -> Result<Box<Expression>, ParserError> {
        let mut expr = self.term()?;
        while self.matches(vec![LESS, LESS_EQUAL, GREATER, GREATER_EQUAL]) {
            let operator: Token = self
                .previous
                .take()
                // .clone()
                .expect("matches will ensure this field to be something");
            let right = self.term()?;
            expr = Box::new(Expression::BinExp(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }
    /// *term*        → `factor ("+"|"-" factor)*;`
    pub fn term(&mut self) -> Result<Box<Expression>, ParserError> {
        let mut expr = self.factor()?;
        while self.matches(vec![MINUS, PLUS]) {
            let operator: Token = self
            .previous
            .take()
            .expect("matches will ensure this field to be something");
            let right = self.factor()?;
            expr = Box::new(Expression::BinExp(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }
    /// *factor*      → `unary (( "/" | "*" ) unary )*;`
    pub fn factor(&mut self) -> Result<Box<Expression>, ParserError> {
        let mut expr = self.unary()?;
        while self.matches(vec![STAR, SLASH]) {
            let operator: Token = self
            .previous
            .take()
            .expect("matches will ensure this field to be something");
            let right = self.unary()?;
            expr = Box::new(Expression::BinExp(BinaryExpr::new(expr, operator, right)));
        }
        Ok(expr)
    }
    /// *unary*       → `("-" | "!") unary | primary;`
    pub fn unary(&mut self) -> Result<Box<Expression>, ParserError> {
        if self.matches(vec![MINUS, BANG]) {
            let operator: Token = self
            .previous
            .take()
            .expect("matches will ensure this field to be something");
            let right_expr = self.unary()?;
            return Ok(Box::new(Expression::UnExp(
                UnaryExpr::new(operator, right_expr)
                .expect("Scanner should catch malformed unary expressions"),
            )));
        }
        self.primary()
    }
    /// *primary*     → `literal | "(" expression ")";`
    /// *literal*     → Number | String | "true" | "false" | "nil" ;
    pub fn primary(&mut self) -> Result<Box<Expression>, ParserError> {
        // "1+3+4(3+4)"
        if self.matches(vec![FALSE, TRUE, NIL, NUMBER, STRING]) {
            // Previous is sure to exist if this branch is entered
            // Also constructing a literal is infallible at this stage
            let p = self.previous.clone().expect("Previous should have something here");
            if let Some(peeked_token) = self.peek() {
                match peeked_token.r#type {
                    // LEFT_PAREN | LEFT_BRACE | LEFT_SQUARE | RIGHT_BRACE | RIGHT_PAREN | RIGHT_SQUARE => {
                    //     Lox::report_err(
                    //         peeked_token.line_number, 
                    //         peeked_token.col, 
                    //         format!("Unexpected token {peeked_token:#?} after {p:#?}")
                    //     );
                    //     return Err(ParserError::InvalidToken(Some(peeked_token).cloned()));
                    // }
                    _ => {}
                }
            }
            Ok(Box::new(Expression::Lit(
                Literal::new(self.previous.take().unwrap()).unwrap(),
            )))
        } else if self.matches(vec![LEFT_PAREN]) {
            let expr = self.expression()?;
            let _expect_right_paren = self.consume(RIGHT_PAREN)?;
            // This assertion should never fail
            assert!(_expect_right_paren.is_some());
            // .expect("Expect ')' after expression");
            Ok(Box::new(Expression::Group(Grouping::new(expr))))
        } else {
            // "Each token must be matched by now, if not, the parser may have not understand where the Token
            // fits into the grammar production after falling from expression upto token, in which case we have to write code
            // to handle that, or the Token is simply in the wrong place and a parser error should be reported "
            // panic!("Cannot parse as primary expression");
            if !self.is_at_end() {
                Err(ParserError::InvalidToken(self.tokens.peek().cloned()))
            }
            // The next token is EOF and therefore we've run out of tokens to parse
            else {
                // self.is_at_end == true and a primary expression is being searched for, but since is_at_end == true,
                // the next token is EOF, and therefore the expression is ill-formed
                Err(ParserError::UnexpectedExpression)
            }
        }
    }
}
impl Parser {
    /// Peeks the current token iterator for a match in the list of searchable token types passed to it.
    /// For instance in the comparison rule, we may want to check a multitude of tokentypes('<','<=',...) for a comparision,
    /// so we can pass all comparison operators in the searchable list and if we get a yes back from this function,
    /// it means that we must call the comparision rule again, otherwise we are done with comparison expressions and must
    /// "descend" down the grammar rule list to *term* and so on
    fn matches(&mut self, searchable_list: Vec<TokenType>) -> bool {
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
        else if let Some(peeked_token) = self.tokens.peek() { 
            Lox::report_err(peeked_token.line_number, peeked_token.col, format!("Invalid Token {peeked_token:#?} encountered\nExpected {expected_token:#?}") );
            Err(ParserError::InvalidToken(self.tokens.peek().cloned()))
        } 
        // None is peeked that means we are at EOF
        else {
            // self.previous is guaranteed to exist at this point because we haven't formed an expression yet
            // and we are only peeking ahead to check if the right token follows. If this contract is violated it's a bug
            // and should be reported as a interpreter/compiler internal error
            assert!(self.previous.is_some(), "Internal Lox Error, expected parser.previous to be Some(_) found None");
            let peeked_token = self.previous.clone().unwrap();
            // We should enter this condition
            if self.is_at_end() {
                // This should report EOF in the error msg
                Lox::report_err(peeked_token.line_number, peeked_token.col, format!("Unexpected end of file, found {:#?}", peeked_token.r#type));
            }
            Err(ParserError::UnexpectedExpression)
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

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().better_peekable(),
            current: 0_usize,
            previous: None,
        }
    }
    /// Starts the parser
    pub fn run(&mut self) -> Result<Box<Expression>, ParserError> {
        self.expression()
    }
}
