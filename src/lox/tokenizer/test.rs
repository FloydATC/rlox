

use crate::lox::scanner::Scanner;
use crate::lox::token::TokenKind;
use super::{Tokenizer, Tokenize};


#[test]
fn tokenizer_emptystring() {
    let scanner = Scanner::<std::io::Cursor<&str>>::str("");
    let tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_symbols() {
    let scanner = Scanner::<std::io::Cursor<&str>>::str("+-*/");
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
    let scanner = Scanner::<std::io::Cursor<&str>>::str("+\n-\n*\n/\n");
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
    let scanner = Scanner::<std::io::Cursor<&str>>::str(" + // comment\n- // comment");
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
    let scanner = Scanner::<std::io::Cursor<&str>>::str(" +/*// comment\n * // comment */-");
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
    let scanner = Scanner::<std::io::Cursor<&str>>::str("2+3");
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



