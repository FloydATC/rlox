

//#[allow(unused_imports)]
use super::{Scanner, Scan};


#[test]
fn scanner_emptystring() {
    let code = "";
    let reader = std::io::Cursor::new(code);
    let mut scanner = Scanner::new(reader);
    let (fileno, lineno, charno) = scanner.at();
    assert_eq!(fileno, 0);
    assert_eq!(lineno, 1);
    assert_eq!(charno, 1);
    assert_eq!(scanner.eof(), true);
    assert_eq!(scanner.current(), '\0');
    assert_eq!(scanner.peek(), '\0');
    assert_eq!(scanner.peek_next(), '\0');
}


#[test]
fn scanner_no_advance_past_eof() {
    let code = "";
    let reader = std::io::Cursor::new(code);
    let mut scanner = Scanner::new(reader);
    scanner.advance();
    let (fileno, lineno, charno) = scanner.at();
    assert_eq!(fileno, 0);
    assert_eq!(lineno, 1);
    assert_eq!(charno, 1);
    assert_eq!(scanner.eof(), true);
    assert_eq!(scanner.current(), '\0');
    assert_eq!(scanner.peek(), '\0');
    assert_eq!(scanner.peek_next(), '\0');
}


#[test]
fn scanner_count_lines() {
    let code = "\n\n\n";
    let reader = std::io::Cursor::new(code);
    let mut scanner = Scanner::new(reader);
    scanner.advance();
    scanner.advance();
    scanner.advance();
    let (fileno, lineno, charno) = scanner.at();
    assert_eq!(fileno, 0);
    assert_eq!(lineno, 4);
    assert_eq!(charno, 1);
    assert_eq!(scanner.eof(), true);
    assert_eq!(scanner.current(), '\0');
    assert_eq!(scanner.peek(), '\0');
    assert_eq!(scanner.peek_next(), '\0');
}


#[test]
fn scanner_count_chars() {
    let code = "foo";
    let reader = std::io::Cursor::new(code);
    let mut scanner = Scanner::new(reader);
    scanner.advance();
    scanner.advance();
    scanner.advance();
    let (fileno, lineno, charno) = scanner.at();
    assert_eq!(fileno, 0);
    assert_eq!(lineno, 1);
    assert_eq!(charno, 4);
    assert_eq!(scanner.eof(), true);
    assert_eq!(scanner.current(), '\0');
    assert_eq!(scanner.peek(), '\0');
    assert_eq!(scanner.peek_next(), '\0');
}


#[test]
fn scanner_count_lines_and_chars() {
    let code = "foo\nbar";
    let reader = std::io::Cursor::new(code);
    let mut scanner = Scanner::new(reader);
    scanner.advance();
    scanner.advance();
    scanner.advance();
    scanner.advance();
    let (fileno, lineno, charno) = scanner.at();
    assert_eq!(fileno, 0);
    assert_eq!(lineno, 2);
    assert_eq!(charno, 1);
    assert_eq!(scanner.eof(), false);
    assert_eq!(scanner.current(), 'b');
    assert_eq!(scanner.peek(), 'a');
    assert_eq!(scanner.peek_next(), 'r');
}



