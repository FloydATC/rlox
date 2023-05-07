

//#[allow(unused_imports)]
use super::{Scanner, Scan};


#[test]
fn scanner_emptystring() {
    let mut scanner = Scanner::<std::io::Cursor<&str>>::str("");
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
    let mut scanner = Scanner::<std::io::Cursor<&str>>::str("");
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
    let mut scanner = Scanner::<std::io::Cursor<&str>>::str("\n\n\n");
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
    let mut scanner = Scanner::<std::io::Cursor<&str>>::str("foo");
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
    let mut scanner = Scanner::<std::io::Cursor<&str>>::str("foo\nbar");
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



