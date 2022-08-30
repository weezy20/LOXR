# another Lox interpreter 

The main focus of this project is to understand how compilers work on scanning, parsing, and evaluating ASTs. We follow Robert Nystroms book, but we also keep in mind rustic idioms and try leverage Rust's language design to our benefit rather than simply translating the Java implementation to Rust. 

Here's something of interest that I stumbled upon while studying representation of code: 

[My analysis of the expression problem](./Expression_Problem.md)

### Some quotes from the book : 
1. `State and statements go hand in hand. Since statements, by definition, don’t evaluate to a value, they need to do something else to be useful. That something is called a side effect. It could mean producing user-visible output or modifying some state in the interpreter that can be detected later. The latter makes them a great fit for defining variables or other named entities.`

2. `A token represents a unit of code at a specific place in the source text, but when it comes to looking up variables, all identifier tokens with the same name should refer to the same variable (ignoring scope for now). Using the raw string ensures all of those tokens refer to the same map key.`