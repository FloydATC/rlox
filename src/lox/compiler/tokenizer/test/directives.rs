

use crate::lox::compiler::{Scanner, Tokenizer, Tokenize, TokenKind};


const TESTLIB: &str = "src/lox/compiler/tokenizer/test/testlib/";


#[test]
fn tokenizer_include_invalid_file() {
    let code = "#include<not_found>";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let mut tokenizer = Tokenizer::new_with_library(scanner, TESTLIB);
    assert_eq!(tokenizer.current().kind(), TokenKind::Error);
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_include_test1_then_eof() {
    let code = "#include<test1>";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let mut tokenizer = Tokenizer::new_with_library(scanner, TESTLIB);
    assert_eq!(tokenizer.current().kind(), TokenKind::Identifier);
    assert_eq!(tokenizer.current().lexeme(), "test");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_include_test2_then_eof() {
    let code = "#include<test2>";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let mut tokenizer = Tokenizer::new_with_library(scanner, TESTLIB);
    assert_eq!(tokenizer.current().kind(), TokenKind::Identifier);
    assert_eq!(tokenizer.current().lexeme(), "test");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_include_twice_then_eof() {
    let code = "#include<test1>#include<test1>";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let mut tokenizer = Tokenizer::new_with_library(scanner, TESTLIB);
    assert_eq!(tokenizer.current().kind(), TokenKind::Identifier);
    assert_eq!(tokenizer.current().lexeme(), "test");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Identifier);
    assert_eq!(tokenizer.current().lexeme(), "test");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}


#[test]
fn tokenizer_include_once_test1_then_eof() {
    let code = "#include_once<test1>";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let mut tokenizer = Tokenizer::new_with_library(scanner, TESTLIB);
    assert_eq!(tokenizer.current().kind(), TokenKind::Identifier);
    assert_eq!(tokenizer.current().lexeme(), "test");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_include_once_twice_then_eof() {
    let code = "#include_once<test1>\n#include_once<test1>";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let mut tokenizer = Tokenizer::new_with_library(scanner, TESTLIB);
    assert_eq!(tokenizer.current().kind(), TokenKind::Identifier);
    assert_eq!(tokenizer.current().lexeme(), "test");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

