

use scanner::Scanner;


use crate::lox::compiler::{Tokenizer, Tokenize, TokenKind};


#[test]
fn tokenizer_comment_only() {
    let code = "// comment";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_comment_then_identifier() {
    let code = "// comment\ntest";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), TokenKind::Identifier);
    assert_eq!(tokenizer.current().lexeme(), "test");
}

#[test]
fn tokenizer_line_comments() {
    let code = " + // comment\n- // comment";
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
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
    let scanner = Scanner::new("test", reader);
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

