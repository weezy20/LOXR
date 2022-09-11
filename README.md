# Another Lox interpreter 

[Lox grammar primer (incomplete)](./Lox_Grammar.md)

The main focus of this project is to understand how compilers work on scanning, parsing, and evaluating ASTs. We follow Robert Nystrom's book, but we also keep in mind rustic idioms and try leverage Rust's language design to our benefit rather than simply translating the Java implementation to Rust. 

**Additional features:** 
1. Ternary operations and comma expressions work. Check the [programs](./programs/) & [comma in while condition](./programs/while.lox) for examples
2. Break statements work in while loops and nested scopes. Check examples [nested_break](./programs/nested_break.lox) & [break](./programs/break.lox)

#### Work in Prorgress (many things don't work as of yet): 
This is mainly a learning exercise but that doesn't mean it can't aspire for best code practices. Rust's error messages are something I **love** and I've tried to replicate that here. You are encouraged to clone, `cargo run`, and try to break it. I've tried my best to handle parsing and evaluation errors and be consistent with the error messages and formatting but all is not perfect. If you find a situation that causes a `panic`, `ICE`, a bad error message or if something doesn't work as expected, please open an issue. You can also try running with `cargo run --features debug` to see additional interpreter debug messages that I've sprinkled across the codebase for my debugging convenience. 

Here's somethings of interest that I stumbled upon while studying representation of code: 

[Learn parser combinators in Rust](https://bodil.lol/parser-combinators/)

[My analysis of the expression problem](./Expression_Problem.md)

### Some quotes I liked from the book : 
1. `State and statements go hand in hand. Since statements, by definition, don’t evaluate to a value, they need to do something else to be useful. That something is called a side effect. It could mean producing user-visible output or modifying some state in the interpreter that can be detected later. The latter makes them a great fit for defining variables or other named entities.`

2. `A token represents a unit of code at a specific place in the source text, but when it comes to looking up variables, all identifier tokens with the same name should refer to the same variable (ignoring scope for now). Using the raw string ensures all of those tokens refer to the same map key.`

3. `Mutating a variable is a side effect and, as the name suggests, some language folks think side effects are dirty or inelegant. Code should be pure math that produces values—crystalline, unchanging ones—like an act of divine creation. Not some grubby automaton that beats blobs of data into shape, one imperative grunt at a time.`
