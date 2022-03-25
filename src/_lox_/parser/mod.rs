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
//!     L: Expression + Printer + Evaluate,
//!     R: Expression + Printer + Evaluate,
//! {
//!     left: L,
//!     right: R,
//! }
//! ```
//! 
//! Is this any better? Again, sadly no, as trait bounds will need to be modified when you add new kinds of operations
//! As it stands now, Rust allows us to easily expand behaviour through traits, but to extend types, especially types
//! that share the "class", we would have to resort to enums, wrapped enums in the case of extending types, and if we
//! don't want enums, trait objects which would require to be downcasted in order to do anything useful


/// Definition for Expression enum, and types that are Expression
pub mod traits;

/// Expression types
pub mod expressions;

