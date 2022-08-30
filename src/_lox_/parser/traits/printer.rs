use super::super::expressions::*;

macro_rules! start {
    ($id: tt) => {{
        let s = format!(" {} ( ", $id);
        s
    }};
}

pub trait ExpressionPrinter {
    /// String representation of current ExpressionPrinter
    fn print(&self) -> String;
}

impl ExpressionPrinter for Expression {
    fn print(&self) -> String {
        match self {
            Expression::BinExp(e) => e.print(),
            Expression::UnExp(e) => e.print(),
            Expression::Lit(e) => e.print(),
            Expression::Group(e) => e.print(),
            Expression::CommaExpr(e) => e
                .iter()
                .map(|expr| expr.print())
                .collect::<Vec<String>>()
                .join(" --COMMA EXPR-- "),
            Expression::TernExp(e) => {
                let mut result = format!("Ternary Expression\n");
                result.push_str(&format!("Condition: {}", &e.condition.print()));
                result.push_str(&format!("If Condtion true eval: {}", &e.if_true.print()));
                result.push_str(&format!("If Condtion false eval: {}", &e.if_false.print()));
                result
            }
            Expression::Error(e) => {
                format!("Printing Erroneous Expression: {}", e.print())
            }
        }
    }
}

impl ExpressionPrinter for Literal {
    fn print(&self) -> String {
        let mut s = start!("Literal");
        s.push_str(&self.inner.lexeme);
        s.push_str(" )");
        s
    }
}

impl ExpressionPrinter for Grouping {
    fn print(&self) -> String {
        let mut s = start!("Grouping");
        s.push_str(&self.inner.print());
        s.push_str(" ) ");
        s
    }
}

impl ExpressionPrinter for UnaryExpr {
    fn print(&self) -> String {
        let mut s = start!("UnaryExp");
        s.push_str(&self.operator.lexeme);
        s.push_str(&self.operand.print());
        s
    }
}

impl ExpressionPrinter for BinaryExpr {
    fn print(&self) -> String {
        let mut s = start!("BinaryExp");
        s.push_str(&self.operator.lexeme);
        s.push_str(&self.left.print());
        s.push_str(&self.right.print());
        s
    }
}
