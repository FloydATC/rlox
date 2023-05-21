

use crate::lox::compiler::{Scanner, Scanners, Scan};


#[test]
fn new() {
    let code = "";
    let reader = std::io::Cursor::new(code);
    let scanner = Scanner::new(reader);
    let _scanners = Scanners::new(scanner);
}

#[test]
fn simple_eof_true() {
    let code = "";
    let reader = std::io::Cursor::new(code);
    let scanner = Scanner::new(reader);
    let mut scanners = Scanners::new(scanner);
    assert_eq!(scanners.eof(), true);
}

#[test]
fn simple_eof_false() {
    let code = "a";
    let reader = std::io::Cursor::new(code);
    let scanner = Scanner::new(reader);
    let mut scanners = Scanners::new(scanner);
    assert_eq!(scanners.eof(), false);
}

#[test]
fn simple_peek_into_void() {
    let code = "a";
    let reader = std::io::Cursor::new(code);
    let scanner = Scanner::new(reader);
    let mut scanners = Scanners::new(scanner);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.peek(), '\0');
}

#[test]
fn simple_peek_into_nonvoid() {
    let code = "ab";
    let reader = std::io::Cursor::new(code);
    let scanner = Scanner::new(reader);
    let mut scanners = Scanners::new(scanner);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.peek(), 'b');
}

#[test]
fn include() {
    let code1 = "b";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "a";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.include(scanner2);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'a');
    assert_eq!(scanners.peek(), 'b');
}

#[test]
fn include_then_advance() {
    let code1 = "b";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "a";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.include(scanner2);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'a');
    assert_eq!(scanners.peek(), 'b');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'b');
    assert_eq!(scanners.peek(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), true);
}

#[test]
fn include_before_peek_next() {
    let code1 = "cd";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "ab";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.include(scanner2);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'a');
    assert_eq!(scanners.peek(), 'b');
    assert_eq!(scanners.peek_next(), 'c');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'b');
    assert_eq!(scanners.peek(), 'c');
    assert_eq!(scanners.peek_next(), 'd');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'c');
    assert_eq!(scanners.peek(), 'd');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'd');
    assert_eq!(scanners.peek(), '\0');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), true);
}


#[test]
fn include_middle_peek_next() {
    let code1 = "ad";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "bc";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.advance();
    scanners.include(scanner2);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'b');
    assert_eq!(scanners.peek(), 'c');
    assert_eq!(scanners.peek_next(), 'd');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'c');
    assert_eq!(scanners.peek(), 'd');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'd');
    assert_eq!(scanners.peek(), '\0');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), true);
}

#[test]
fn include_after_peek_next() {
    let code1 = "ab";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "cd";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.advance();
    scanners.advance();
    scanners.include(scanner2);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'c');
    assert_eq!(scanners.peek(), 'd');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'd');
    assert_eq!(scanners.peek(), '\0');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), true);
}


#[test]
fn include_one_and_one_peek_next() {
    let code1 = "c";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "b";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.include(scanner2);
    let code3 = "a";
    let reader3 = std::io::Cursor::new(code3);
    let scanner3 = Scanner::new(reader3);
    scanners.include(scanner3);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'a');
    assert_eq!(scanners.peek(), 'b');
    assert_eq!(scanners.peek_next(), 'c');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'b');
    assert_eq!(scanners.peek(), 'c');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'c');
    assert_eq!(scanners.peek(), '\0');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), true);
}

#[test]
fn include_one_then_two_peek_next() {
    let code1 = "";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "c";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.include(scanner2);
    let code3 = "ab";
    let reader3 = std::io::Cursor::new(code3);
    let scanner3 = Scanner::new(reader3);
    scanners.include(scanner3);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'a');
    assert_eq!(scanners.peek(), 'b');
    assert_eq!(scanners.peek_next(), 'c');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'b');
    assert_eq!(scanners.peek(), 'c');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'c');
    assert_eq!(scanners.peek(), '\0');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), true);
}

#[test]
fn include_two_then_one_peek_next() {
    let code1 = "";
    let reader1 = std::io::Cursor::new(code1);
    let scanner1 = Scanner::new(reader1);
    let mut scanners = Scanners::new(scanner1);
    let code2 = "bc";
    let reader2 = std::io::Cursor::new(code2);
    let scanner2 = Scanner::new(reader2);
    scanners.include(scanner2);
    let code3 = "a";
    let reader3 = std::io::Cursor::new(code3);
    let scanner3 = Scanner::new(reader3);
    scanners.include(scanner3);
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'a');
    assert_eq!(scanners.peek(), 'b');
    assert_eq!(scanners.peek_next(), 'c');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'b');
    assert_eq!(scanners.peek(), 'c');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), false);
    assert_eq!(scanners.current(), 'c');
    assert_eq!(scanners.peek(), '\0');
    assert_eq!(scanners.peek_next(), '\0');
    scanners.advance();
    assert_eq!(scanners.eof(), true);
}

