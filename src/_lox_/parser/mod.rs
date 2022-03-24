//! The way we solve the expression problem here, which briefly states, that due to the way languages are designed
//! certain operations are difficult. In the OOP paradigm, a class contains behaviour bundled with it, but let's say
//! we were introducing a new behaviour across all our types, that would imply modifying code from each of our pre-existing
//! types to include that behaviour, uniquely implemented for each class. This goes against two principles that we'd like 
//! to adhere when writing scalable software, the first being the open-close principle which briefly states that scalable
//! software should be open to extension but closed to modification. Another problem with bundling behaviour with the class
//! is violation of concerns. A tree node shouldn't have methods pertaining specifically either to the parser where it is
//! produced or the interpreter where it is consumed. This leads to a violation of separation of concerns where 
//! two domains are stepping on each other's toes by smushing interpreter and parser specific logic in the same location
//! which is the class definition.
//! 
//! In the functional programming paradigm, types are easy to add, they are inert types, with no behaviour defined on them
//! instead, behaviour, known as functions, pattern match on the type that's passed to them and then perform the appropriate 
//! action. This has a downside, because first, if you were to add a new operation, it's adheres to extensibility as you can
//! add code, in a function, and then write it for each pre-existing type. But what happens when you have to add 
//! new type? That would imply going back to the code, which remember, we must treat as untouchable, open only to extension
//! and not modification, and changing each function body to include our new type. It would be nice if we could just add 
//! new functions like OOP paradigm, by bundling the implementation of a each behaviour while defining our type, and 
//! it would also be desirable to include new types like the FP way, inert data structures.
//! 
//! In rust, we make use of the type system, and traits, which roughly act like class interfaces to achieve both goals:
//! First, consider what we are trying to achieve, mainly, adding new types should be straight forward, should not
//! imply having to change pre-existing functional code, and should enable for implementing all desired behaviour
//! by adding code. 
//! 
//! Secondly, we would like to have a way to implement behaviour the OOP way, which was that with each new type, it brings
//! its own implementations for methods that it is supposed to work with, like eval(), print(), etc. which may pertain
//! either to the parser or the interpreter. We maintain separation of concerns this way. We also enable code extension,
//! rather than modification. 
//! 
//! If we have to add a new type, none of the existing code (behaviour) needs to change
//! If we have to add a new operation, none of the existing type's impls need to be modified, instead
//! a new operation is defined in the form of a trait, and we write a file new_operation.rs with the new 
//! operation :
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



/// Definition for Expression enum, and types that are Expression
pub mod traits;

/// Expression types
pub mod expressions;