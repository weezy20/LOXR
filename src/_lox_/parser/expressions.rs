use crate::_lox_::tokenizer::token::Token;
use crate::_lox_::tokenizer::token_type::TokenType;

/// # The overarching Expression type
///
/// An Expression can be of the following types:
/// 1. Literal
/// 2. Unary Expression with prefixes like ! or -
/// 3. Binary Expression with infix arithmetic operators  (+, -, *, /) or logic operators (==, !=, <, <=, >, >=)
/// 4. Parantheses: An Expression maybe wrapped in a a pair of ( and )
///
/// Types that require nesting of Expression, use generics with trait bounds as a means to convey
/// "type information"

pub trait Expression {}
// // Thanks to Yandros
// macro_rules! impl_expr {(
//     [$($generics:tt)*] => $E:ty $(where $($wc:tt)*)?
// ) => (
//     impl<$($generics)*> Expression for $E
//     where
//         /* your own bounds here, with a trailing `,` */
//         $($(wc)*)?
//     {

//     }
// )}

// impl_expr! {
//     for[T : Expression] => BinaryExpr<L,R>
// }
// // or
// impl_expr! {
//     for[T] Foo<T> where T : Expression
// }
// // as well as:
// impl_expr! {
//     for[] Bar where T : Expression
// }
macro_rules! impl_expr {
    // ($e:ident$(<$($g:ident),+>)?) => {
    //     impl<$($g)+>
    // };
    ($e:tt<$L:ident,$R:ident>) => {
        impl<$L, $R> Expression for $e<$L, $R>
        where
            $L: Expression,
            $R: Expression,
        {
        }
    };
    ($e:tt<$E:ident>) => {
        impl<$E> Expression for $e<$E> where $E: Expression {}
    };
    ($e:ident) => {
        impl Expression for $e {}
    };
}

impl_expr!(BinaryExpr<L,R>);
impl_expr!(Grouping<E>);
impl_expr!(UnaryExpr<E>);
impl_expr!(Literal);

#[derive(Debug)]
pub struct BinaryExpr<L, R>
where
    L: Expression,
    R: Expression,
{
    pub left: L,
    pub operator: Token,
    pub right: R,
}

impl<L, R> BinaryExpr<L, R>
where
    L: Expression,
    R: Expression,
{
    pub fn new(left: L, operator: Token, right: R) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr<E: Expression> {
    pub operator: Token,
    pub operand: E,
}
impl<E: Expression> UnaryExpr<E> {
    /// Question: What happens if operand : is a UnaryExpr. Nothing special, valid syntax
    pub fn new(operator: Token, operand: E) -> Result<Self, String> {
        match operator.r#type {
            TokenType::MINUS | TokenType::BANG => Ok(Self { operand, operator }),
            u => Err(format!(
                "Cannot construct Unary expression with operator: {u:?}"
            )),
        }
    }
}

#[derive(Debug)]
pub struct Literal {
    pub inner: Token,
}
impl Literal {
    pub fn new(inner: Token) -> Result<Self, String> {
        let token_type = inner.r#type;
        if token_type.is_literal() {
            Ok(Self { inner })
        } else {
            Err(format!(
                "Cannot build a literal of token type {token_type:?}"
            ))
        }
    }
}

#[derive(Debug)]
pub struct Grouping<E: Expression> {
    pub inner: E,
}

#[cfg(test)]
mod test {
    use crate::_lox_::parser::expression_printer::ExpressionPrinter;
    use crate::_lox_::tokenizer::token::Token;
    use crate::_lox_::tokenizer::token_type::TokenType;

    use super::*;

    #[test]
    fn pretty_print() {
        let expression = " 1 + (2 - (4 / 5))";
        let (line_number, col) = (1, 1);
        let one =
            Literal::new(Token::new(TokenType::NUMBER, "1".into(), line_number, col)).unwrap();

        let two =
            Literal::new(Token::new(TokenType::NUMBER, "2".into(), line_number, col)).unwrap();

        let four =
            Literal::new(Token::new(TokenType::NUMBER, "4".into(), line_number, col)).unwrap();

        let five =
            Literal::new(Token::new(TokenType::NUMBER, "5".into(), line_number, col)).unwrap();

        let group45 = Grouping {
            inner: BinaryExpr {
                left: four,
                right: five,
                operator: Token::new(TokenType::SLASH, "/".into(), line_number, col),
            },
        };

        let group245 = Grouping {
            inner: BinaryExpr {
                left: two,
                right: group45,
                operator: Token::new(TokenType::MINUS, "-".into(), line_number, col),
            },
        };

        let r#final = BinaryExpr {
            left: one,
            right: group245,
            operator: Token::new(TokenType::PLUS, "+".into(), line_number, col),
        };

        println!(
            "original expression: {expression:?}\nexpression_printer output:\n{:?}",
            r#final.print()
        );
    }
}
