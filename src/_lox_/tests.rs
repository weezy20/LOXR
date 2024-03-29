#![allow(unused, warnings)]
#![cfg(test)]
use crate::interpreter::Environment;
use crate::parser::value::Value;
use crate::parser::Parser;
use crate::tokenizer::scanner::*;
use crate::Lox;
use std::cell::RefCell;
use std::rc::Rc;

mod tokenizer_tests {
    use super::*;
    #[test]
    fn test_tokenizer() {
        let source = String::from(
            r#"
    !*+-/= = = +=<> <
// This is a comment
hello = 4
- + --  
"hi this is a string" -

 "hi this 
    is a multiline
       string "

123.64 "hey jude"

45

// keyword keyword ident
and or not_a_keyword
    "#,
        );
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        dbg!(tokens);
    }

    #[test]
    fn bad_number1() {
        let source = String::from("..123");
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        dbg!(tokens);
    }

    #[test]
    fn bad_number2() {
        // Number at EOF
        let source = String::from("hello = 10.123");
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        dbg!(tokens);
        assert_eq!(scanner.current, source.len());
    }

    #[test]
    fn bad_number3() {
        // alphabet at number end
        let source = String::from("hello = 10.123a ");
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        dbg!(tokens);
        assert_eq!(scanner.current, source.len());
    }

    #[test]
    fn multi_line_comment() {
        let source = String::from(
            r#"
    /* This is a multi line comment
yababababdbbdbabdbabdba
adsadasdasdasd */

// This is a single line comment"#,
        );
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        dbg!(tokens);
        assert_eq!(scanner.current, source.len());
    }

    #[test]
    fn unclosed_comment() {
        let source = String::from(
            r#"
    /* This is a multi line comment
yababababdbbdbabdbabdba
adsadasdasdasd 

// This is a single line comment"#,
        );
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        dbg!(tokens);
        assert_eq!(scanner.current, source.len());
    }
}

mod parser_tests {
    use super::*;
    use crate::interpreter::{self, Interpreter};
    use crate::parser::error::ParserError;
    use crate::parser::traits::evaluate::Evaluate;
    use crate::parser::traits::printer::ExpressionPrinter;
    use crate::setup_lox;
    use crate::tokenizer::token::Token;
    #[test]
    fn term_expression() {
        let source = String::from("4 +10.123");
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        // dbg!(tokens);
        assert_eq!(scanner.current, source.len());
        let mut parser = Parser::new(tokens);
        let parser_result = parser.run();
        println!("Parser Result : {parser_result:?}");
        assert!(parser_result.is_ok());
    }
    #[test]
    fn factor_expression() {
        let source = String::from("4 +10.123/1.2");
        let mut lox = Lox::new(source.clone());
        let mut scanner = Scanner::new(&source, &mut lox);
        scanner.scan_tokens();
        let tokens = scanner.tokens;
        // dbg!(tokens);
        assert_eq!(scanner.current, source.len());
        let mut parser = Parser::new(tokens);
        let parser_result = parser.run();
        println!("Source : {source}\nParser Result : {parser_result:?}");
        assert!(parser_result.is_ok());

        // BinExp[1 + [(2.3+3.4)  * 20] ]
        let tokens = setup_lox!("1+(2.3+3.4)*(4*5)");
        let parser_result = Parser::new(tokens).run();
        assert!(parser_result.is_ok());
        println!("Source : \"1+(2.3+3.4)*(4*5)\"\nParser Result : {parser_result:?}")
    }
    #[test]
    fn illegal_termination() {
        let tokens = setup_lox!("1+3+4/");
        let res = Parser::new(tokens).run();
        assert_eq!(res, Err(ParserError::ExpectedExpression));
    }

    #[test]
    fn unclosed_paren_at_end() {
        use crate::tokenizer::{token::Token, token_type::TokenType::*};
        let tokens = setup_lox!("1+3+4-(3+4");
        let res = Parser::new(tokens).run();
        // assert_eq!(res, Err(ParserError::UnbalancedParen));
        assert_eq!(
            res, // UnexpectedExpression
            Err(ParserError::UnexpectedEOF)
        );
    }
    // #[ignore = "Lox cannot handle beyond simple arithmetic expressions at this point"]
    #[test]
    fn illegal_expressions() {
        // The first two are legal but unimplemented
        // let tokens = setup_lox!("*1+3+4-(3+4)");
        // let tokens = setup_lox!("/1+3+4-(3+4)");
        // let tokens = setup_lox!("/1+3+4-(3+4)");
        // TODO
        // Note these are entirely different expressions yet the assertion passes if you run this
        let tokens1 = setup_lox!("1+3+4(3+4)"); // illegal
        let res1 = Parser::new(tokens1).run();
        let tokens2 = setup_lox!("1+3+4(3+4)"); // illegal
        let res2 = Parser::new(tokens2).run();
        // println!("res1: {res1:#?}");
        // println!("res2: {res2:#?}");
        assert_eq!(res1, res2);
    }
    #[test]
    fn check_ternary_expression() {
        let tokens = setup_lox!("4 == 5? 1 : 0");
        let res = Parser::new(tokens).run();
        println!("{:?}", res);
        assert!(res.is_ok());
    }
    #[test]
    fn check_nested_ternary_expression() {
        let tokens = setup_lox!("4 == 5? 1 < 2 ? 44 < 55 ? 1 : 0 : -1 : -2");
        let res = Parser::new(tokens).run();
        println!("4 == 5? 1 < 2 ? 44 < 55 ? 1 : 0 : -1 : -2 -> \n{:?}", res);
        assert!(res.is_ok());
    }
    #[test]
    fn check_nested_ternary_expression1() {
        let tokens = setup_lox!("4 == 5? 1 < 2 ? 1 : 2 : 3;");
        let mut env = Rc::new(RefCell::new(Environment::default()));
        let res = Parser::new(tokens).run();
        assert!(res.is_ok());
        let res = res.unwrap().eval(&mut env).unwrap();
        assert_eq!(Value::Double(3.0), res);
        println!("{:#?}", res);
    }

    #[test]
    fn check_nested_ternary_expression2() {
        let tokens = setup_lox!("4 == 5? 1 < 2 ? 1 : 2 : 3");
        let mut env = Rc::new(RefCell::new(Environment::default()));
        let res = Parser::new(tokens).run().unwrap().eval(&mut env).unwrap();
        println!("4 == 5? 1 < 2 ? 1 : 2 : 3 -> \n{:?}", res);
        assert_eq!(res, Value::Double(3.0));
    }
    #[test]
    fn check_nested_ternary_expression3() {
        // let tokens = setup_lox!("var a; var b; var c; var d; var e; a = !(b = 2) ? c = 2 : d = !(e = 3) ? 100 : 1000;");
        let mut env = Rc::new(RefCell::new(Environment::default()));
        {
            let mut e = env.borrow_mut();
            e.values.insert("a".to_string(), Value::Nil);
            e.values.insert("b".to_string(), Value::Nil);
            e.values.insert("c".to_string(), Value::Nil);
            e.values.insert("d".to_string(), Value::Nil);
            e.values.insert("e".to_string(), Value::Nil);
        } // RefMut dropped here
        let tokens = setup_lox!("a = !(b = 2) ? c = 2 : d = !(e = 3) ? 100 : 1000;");
        let p = Parser::new(tokens);
        let mut int = Interpreter::default();
        &mut int.extend_with_env(p, env);
        // figure out a way to test from stdout
        println!("var a = !(b = 2) ? c = 2 : d = !(e = 3) ? 100 : 1000; -> \n");
        int.interpret();
        // assert_eq!(res, Value::Double(1000.0));
    }
    #[test]
    /// Missing left operand. This should trigger a synchronization and pick up parsing from 10+11==12
    fn incomplete_expressions() {
        // let tokens = setup_lox!("1+");
        // let tokens = setup_lox!("-+*4/62;10+11==12"); // works
        // let tokens = setup_lox!("+*4/62;10+11==12"); // works
        // let tokens = setup_lox!("++*4/62;10+11==12"); // works
        // let tokens = setup_lox!("/+*4/62;10+11==12"); // works
        // let tokens = setup_lox!("/*+4/62;10+11==12"); // Unclosed Comment /*
        // let res = Parser::new(tokens).run();
        // println!("INCOMPLETE_EXPRESSIONS RESULT : {res:#?}");
        let test_cases: Vec<Vec<Token>> = vec![
            // setup_lox!("1+"),
            setup_lox!("-+*4/62;10+11==12"),
            setup_lox!("+*4/62;10+11==12"),
            setup_lox!("++*4/62;10+11==12"),
            setup_lox!("/+*4/62;10+11==12"),
            // setup_lox!("/*+4/62;10+11==12"),
        ];
        for case in test_cases {
            let res = Parser::new(case.clone()).run();
            // println!("Input : {case:?} ");
            println!("Result : {res:#?}");
            assert!(res.is_ok());
        }
    }
    #[test]
    /// Missing left operand. This should trigger a synchronization and pick up parsing from 10+11==12
    fn incomplete_expressions_special1() {
        let tokens = setup_lox!("+-+-+-+-+-+*-/1");
        // let tokens = setup_lox!("/*+4/62;10+11==12"); // Not working Err(UnexpectedExpression)
        let res = Parser::new(tokens).run();
        println!("INCOMPLETE_EXPRESSIONS RESULT : {res:#?}");
    }

    #[test]
    /// Missing left operand. This should trigger a synchronization and pick up parsing from 10+11==12
    fn incomplete_expressions_special2() {
        // let tokens = setup_lox!("//5");  // A double slash is a start of a comment
        let tokens = setup_lox!("/*+4/62;10+11==12"); // Not working Err(UnexpectedExpression)
        let res = Parser::new(tokens).run();
        println!("INCOMPLETE_EXPRESSIONS RESULT : {res:#?}");
    }
    #[test]
    fn legal_expressions() {
        // The first two are legal but unimplemented
        // let tokens = setup_lox!("*1+3+4-(3+4)");
        // let tokens = setup_lox!("/1+3+4-(3+4)");
        // let tokens = setup_lox!("/1+3+4-(3+4)");
        // TODO :
        let tokens2 = setup_lox!("1+3+4*((3+4))"); // legal
        let res2 = Parser::new(tokens2).run();
        println!("res2: {res2:#?}");
        assert!(res2.is_ok());
    }
    // #[ignore = "FIX ME: Write a better test"]¡
    #[test]
    fn assignment() {
        let mut env = Rc::new(RefCell::new(crate::interpreter::Environment::default()));
        let tokens = setup_lox!("a=1+3+4(3+4)");
        let tokens = setup_lox!("a=-1+3+4/(3+4);");
        let res = Parser::new(tokens).run();
        assert!(res.is_ok());
        let tokens = setup_lox!("var a=-1+3+4/(3+4);");
        let res = Parser::new(tokens).parse();
        println!("assingment res {}", res[0]);
    }
    #[test]
    fn comma_expression_print() {
        let tokens = setup_lox!("1+2, 3-23, 4/5");
        let res = Parser::new(tokens).run().unwrap();
        println!("{}", res.print());
    }
    #[test]
    fn function_expression() {
        // let tokens = setup_lox!("first()(data))");
        let tokens = setup_lox!("first()");
        let res = Parser::new(tokens).run().unwrap();
        println!("{}", res.print());
    }
}

mod parser_evaluator {

    use super::*;
    use crate::{parser::traits::evaluate::Evaluate, setup_lox};
    #[test]
    fn simple_eval() {
        let mut env = Rc::new(RefCell::new(crate::interpreter::Environment::default())); // Arithmetic
        let tokens = setup_lox!("1+3+4*((3+4))");
        let res = Parser::new(tokens).run().unwrap().eval(&mut env);
        assert!(res.is_ok());
    }
}

// mod statements {
//     use super::*;
//     #[test]
//     fn statement() {
//         todo!()
//     }
// }

#[macro_export]
macro_rules! setup_lox {
    ($e:literal) => {{
        let src = String::from($e);
        let mut lox = Lox::new(src.clone());
        let mut scanner = Scanner::new(&src, &mut lox);
        scanner.scan_tokens();
        scanner.tokens
    }};
}
