 # Lox grammar: 
 *program*          → `declaration`* EOF;
 
 *declaration*      → `variableDecl` | `statement`;
 
 *variableDecl*     → `"var" IDENTIFIER ("=" expression)? ";"` ;
 
 *statement*        → `exprStmt` | `printStmt` | `block` | `ifStmt` ;
 
 *exprStmt*         → `expression` ";" ;
 
 *printStmt*        → print `expression` ";" ;
 
 *block*            → `"{" (declaration)* "}"` ;
 
 *ifStmt*           → `"if" "(" expression ")"  statement ("else" statement)?` ;
 
 A comma expression evaluates to the final expression
 
 *comma expr*     → `expression , (expression)* | "(" expression ")"`;

 *ternary*        → `expression` ? `expression` : `expression`;

 *expression*     → `literal
                  | unary
                  | binary
                  | grouping ;`

 *literal*        → `NUMBER | STRING | "true" | "false" | "nil" ;`

 *grouping*       → `"(" expression ")" ;`

 *unary*          → `( "-" | "!" ) expression ;`

 *binary*         → `expression operator expression ;`

 *operator*       → `"==" | "!=" | "<" | "<=" | ">" | ">="
                  | "+"  | "-"  | "*" | "/" | "%";`
