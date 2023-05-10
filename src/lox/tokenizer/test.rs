

use crate::lox::scanner::Scanner;
use crate::lox::token::TokenKind;
use super::{Tokenizer, Tokenize};


#[test]
fn tokenizer_emptystring() {
    let code = "";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new(reader);
    let tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_symbols() {
    let code = "+-*/";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new(reader);
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.previous().kind(), TokenKind::Plus);
    assert_eq!(tokenizer.current().kind(), TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.previous().kind(), TokenKind::Minus);
    assert_eq!(tokenizer.current().kind(), TokenKind::Star);
    assert_eq!(tokenizer.current().lexeme(), "*");
    tokenizer.advance();
    assert_eq!(tokenizer.previous().kind(), TokenKind::Star);
    assert_eq!(tokenizer.current().kind(), TokenKind::Slash);
    assert_eq!(tokenizer.current().lexeme(), "/");
    tokenizer.advance();
    assert_eq!(tokenizer.previous().kind(), TokenKind::Slash);
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_symbols_and_newlines() {
    let code = "+\n-\n*\n/\n";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new(reader);
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Star);
    assert_eq!(tokenizer.current().lexeme(), "*");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Slash);
    assert_eq!(tokenizer.current().lexeme(), "/");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_line_comments() {
    let code = " + // comment\n- // comment";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new(reader);
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_block_comment() {
    let code = " +/*// comment\n * // comment */-";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new(reader);
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_add() {
    let code = "2+3";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new(reader);
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
    assert_eq!(tokenizer.current().lexeme(), "2");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
    assert_eq!(tokenizer.current().lexeme(), "3");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

