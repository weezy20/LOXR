#![cfg(test)]
use crate::_lox_::tokenizer::scanner::*;

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
    let mut lox = crate::_lox_::lox::Lox::new(source.clone());
    let mut scanner = Scanner::new(&source, &mut lox);
    scanner.scan_tokens();
    let tokens = scanner.tokens;
    dbg!(tokens);
}

#[test]
fn bad_number1() {
    let source = String::from("..123");
    let mut lox = crate::_lox_::lox::Lox::new(source.clone());
    let mut scanner = Scanner::new(&source, &mut lox);
    scanner.scan_tokens();
    let tokens = scanner.tokens;
    dbg!(tokens);
}

#[test]
fn bad_number2() {
    // Number at EOF
    let source = String::from("hello = 10.123");
    let mut lox = crate::_lox_::lox::Lox::new(source.clone());
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
    let mut lox = crate::_lox_::lox::Lox::new(source.clone());
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
    let mut lox = crate::_lox_::lox::Lox::new(source.clone());
    let mut scanner = Scanner::new(&source, &mut lox);
    scanner.scan_tokens();
    let tokens = scanner.tokens;
    dbg!(tokens);
    assert_eq!(scanner.current, source.len());
}
