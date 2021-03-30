

#[cfg(test)]
use crate::lox::scanner::Scanner;

#[cfg(test)]
use crate::lox::token::TokenKind;

#[cfg(test)]
use super::Tokenizer;


#[test]
fn tokenizer_emptystring() {
    let scanner = Scanner::str("");
    let tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), &TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_symbols() {
    let scanner = Scanner::str("+-*/");
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), &TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Star);
    assert_eq!(tokenizer.current().lexeme(), "*");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Slash);
    assert_eq!(tokenizer.current().lexeme(), "/");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_symbols_and_newlines() {
    let scanner = Scanner::str("+\n-\n*\n/\n");
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), &TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Star);
    assert_eq!(tokenizer.current().lexeme(), "*");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Slash);
    assert_eq!(tokenizer.current().lexeme(), "/");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_line_comments() {
    let scanner = Scanner::str(" + // comment\n- // comment");
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), &TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}

#[test]
fn tokenizer_block_comment() {
    let scanner = Scanner::str(" +/*// comment\n * // comment */-");
    let mut tokenizer = Tokenizer::new(scanner);
    assert_eq!(tokenizer.current().kind(), &TokenKind::Plus);
    assert_eq!(tokenizer.current().lexeme(), "+");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::Minus);
    assert_eq!(tokenizer.current().lexeme(), "-");
    tokenizer.advance();
    assert_eq!(tokenizer.current().kind(), &TokenKind::EOF);
    assert_eq!(tokenizer.current().lexeme(), "\0");
}



