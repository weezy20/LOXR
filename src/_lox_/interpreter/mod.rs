// use crate::parser::expressions::Expression;
// use crate::parser::value::Value;

// pub struct Interpreter {
//     pub expr: Box<Expression>,
//     pub val: Value,
// }

// impl Interpreter {
//     fn interpret(self) -> Value {
//         let expr = *self.expr;
//         match expr {
//             Expression::CommaExpr(_) => todo!(),
//             Expression::TernExp(_) => todo!(),
//             Expression::BinExp(_) => todo!(),
//             Expression::UnExp(_) => todo!(),
//             Expression::Lit(_) => todo!(),
//             Expression::Group(g) => {
//                 let expr = g.inner;
//                 expr.evaluate();
//             },
//             Expression::Error(_) => todo!(),
//         }
//         Value::Nil
//     }
// }
