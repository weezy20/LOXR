 # Lox grammer: 
 *program*          → `declaration`* EOF;
 
 *declaration*      → `variableDecl` | statement;
 
 *variableDecl*     → `"var"` **IDENTIFIER** `("=" expression)? ";"` ;
 
 *statement*        → `exprStmt` | `printStmt` | `block` | `ifStmt` ;
 
 *exprStmt*         → `expression` ";" ;
 
 *printStmt*        → print `expression` ";" ;
 
 *block*            → `"{" (declaration)* "}"` ;
 
 *ifStmt*           → `"if" "(" expression ")"  statement ("else" statement)?` ;

 *whileStmt*        → `"while" "(" expression ")"  statement` ;

 
 **A comma expression evaluates to the final expression**
 
 *comma expr*  → `expression , (expression)* | "(" expression ")"`;

 *expression*     → `ternary
                   | literal
                   | unary
                   | binary
                   | grouping ;`


 *expression*  → `ternary`;
 
 *ternary*     → `assignment` | `assignment` ? `assignment` : `assignment`;
 
 *assignment*  → `logic_or` | **IDENTIFIER** "=" `ternary`
 
 *logic_or*    → `logic_and` ( "or" `logic_and`)* ;
 
 *logic_and*   → `equality` ("and" `equality`)* ; 

 *equality*    → `comparsion ("==" | "!=" comparison)*;`

 *comparison*  → `term ("<="|"<"|">"|">=" term)*;`

 *term*        → `factor ("+"|"-" factor)*;`

 *factor*      → `unary (( "%" | "/" | "*" ) unary )*;`

 *unary*       → `("-" | "!") unary | primary;`

 *primary*     → `literal | IDENTIFIER | "(" expression ")"`;
 
 *literal*        → `NUMBER | STRING | "true" | "false" | "nil" ;`

 *grouping*       → `"(" expression ")" ;`

 *unary*          → `( "-" | "!" ) expression ;`

 *binary*         → `expression operator expression ;`

 *operator*       → `"==" | "!=" | "<" | "<=" | ">" | ">="
                  | "+"  | "-"  | "*" | "/" | "%";`
