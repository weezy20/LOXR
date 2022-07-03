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

use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

/// # The overarching Expression type
///
/// An Expression can be of the following types:
/// 1. Literal
/// 2. Unary Expression with prefixes like ! or -
/// 3. Binary Expression with infix arithmetic operators  (+, -, *, /) or logic operators (==, !=, <, <=, >, >=)
/// 4. Grouping: An Expression maybe wrapped in a a pair of ( and )

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    CommaExpr(Vec<Box<Expression>>),
    TernExp(TernaryExpr),
    BinExp(BinaryExpr),
    UnExp(UnaryExpr),
    Lit(Literal),
    Group(Grouping),
    Error(Box<Expression>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct TernaryExpr {
    pub condition: Box<Expression>,
    pub if_true : Box<Expression>,
    pub if_false : Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpr {
    pub fn new(
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub operand: Box<Expression>,
}
impl UnaryExpr {
    /// Question: What happens if operand : is a UnaryExpr. Nothing special, valid syntax
    pub fn new(operator: Token, operand: Box<Expression>) -> Result<Self, String> {
        match operator.r#type {
            TokenType::MINUS | TokenType::BANG => Ok(Self { operand, operator }),
            u => Err(format!(
                "Cannot construct Unary expression with operator: {u:?}"
            )),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub inner: Token,
}
impl Literal {
    pub fn new(inner: Token) -> Result<Self, String> {
        let token_type = inner.r#type;
        if token_type.is_primary() {
            Ok(Self { inner })
        } else {
            Err(format!(
                "Cannot build a literal of token type {token_type:?}"
            ))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grouping {
    pub inner: Box<Expression>,
}

impl Grouping {
    pub fn new(inner: Box<Expression>) -> Self {
        Self { inner }
    }
}

#[cfg(test)]
mod test {
    use crate::_lox_::parser::traits::ExpressionPrinter;
    use crate::_lox_::tokenizer::token::Token;
    use crate::_lox_::tokenizer::token_type::TokenType;

    use super::*;

    #[test]
    fn pretty_print() {
        let _expression = " 1 + (2 - (4 / 5))";
        let (line_number, col) = (1, 1);
        let one = Expression::Lit(
            Literal::new(Token::new(
                TokenType::NUMBER,
                "1".into(),
                line_number,
                col,
            ))
            .unwrap(),
        );
        let two = Expression::Lit(
            Literal::new(Token::new(
                TokenType::NUMBER,
                "2".into(),
                line_number,
                col,
            ))
            .unwrap(),
        );
        let four = Expression::Lit(
            Literal::new(Token::new(
                TokenType::NUMBER,
                "4".into(),
                line_number,
                col,
            ))
            .unwrap(),
        );
        let five = Expression::Lit(
            Literal::new(Token::new(
                TokenType::NUMBER,
                "5".into(),
                line_number,
                col,
            ))
            .unwrap(),
        );
        let group45 = Expression::Group(Grouping {
            inner: Box::new(Expression::BinExp(BinaryExpr {
                left: Box::new(four),
                right: Box::new(five),
                operator: Token::new(
                    TokenType::SLASH,
                    "/".into(),
                    line_number,
                    col,
                ),
            })),
        });

        let group245 = Expression::Group(Grouping {
            inner: Box::new(Expression::BinExp(BinaryExpr {
                left: Box::new(two),
                right: Box::new(group45),
                operator: Token::new(
                    TokenType::MINUS,
                    "-".into(),
                    line_number,
                    col,
                ),
            })),
        });

        let r#final = Expression::BinExp(BinaryExpr {
            left: Box::new(one),
            right: Box::new(group245),
            operator: Token::new(TokenType::PLUS, "+".into(), line_number, col),
        });

        println!("{:?}", r#final.print());
    }
}
