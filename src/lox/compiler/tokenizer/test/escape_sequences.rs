

use crate::lox::compiler::{Scanner, Tokenizer, Tokenize, TokenKind};

// Escape sequences

#[test]
fn doublequoted_string_tab() {
    let code = "\"\\t\""; // "\t" should parse as a string containing a single tab character (ASCII 0x09)
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\t");
}

#[test]
fn doublequoted_string_newline() {
    let code = "\"\\n\""; // "\n" should parse as a string containing a single newline character (ASCII 0x0A)
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\n");
}

#[test]
fn doublequoted_string_return() {
    let code = "\"\\r\""; // "\r" should parse as a string containing a single return character (ASCII 0x0D)
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\r");
}

#[test]
fn doublequoted_string_doublequote() {
    let code = "\"\\\"\""; // "\"" should parse as a string containing a single doublequote character (ASCII 0x22)
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\"");
}

#[test]
fn doublequoted_string_singlequote() {
    let code = "\"\\\'\""; // "\'" is an error; \' is only valid in single quoted strings
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::Error);
}

#[test]
fn doublequoted_string_backslash() {
    let code = "\"\\\\\""; // "\\" should parse as a string containing a single backslash character (ASCII 0x5C)
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\\");
}

#[test]
fn doublequoted_string_unicode() {
    let code = "\"\\u{0201}\""; // "\u{0201}" should parse as a string containing a single unicode character
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\u{0201}");
}

#[test]
fn doublequoted_string_unicode_with_prefix() {
    let code = "\"foo\\u{0201}\""; // "foo\u{0201}" should parse as a string containing a single unicode character
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "foo\u{0201}");
}

#[test]
fn doublequoted_string_unicode_with_suffix() {
    let code = "\"\\u{0201}bar\""; // "foo\u{0201}" should parse as a string containing a single unicode character
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\u{0201}bar");
}

#[test]
fn doublequoted_string_unicode_with_prefix_and_suffix() {
    let code = "\"foo\\u{0201}bar\""; // "foo\u{0201}" should parse as a string containing a single unicode character
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "foo\u{0201}bar");
}

#[test]
fn doublequoted_string_reject_malformed_unicode_1() {
    let code = "\"\\u{0201\""; // "\u{0201" is an error
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::Error);
}

#[test]
fn doublequoted_string_reject_malformed_unicode_2() {
    let code = "\"\\u0201\""; // "\u0201" is an error
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::Error);
}

#[test]
fn doublequoted_string_reject_malformed_unicode_3() {
    let code = "\"\\u{}\""; // "\u{}" is an error
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::Error);
}

#[test]
fn doublequoted_string_reject_malformed_unicode_4() {
    let code = "\"\\u\""; // "\u{}" is an error
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::Error);
}

#[test]
fn doublequoted_string_reject_invalid_unicode() {
    let code = "\"\\u{110000}\""; // codepoints > 0x10FFFF are invalid
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::Error);
}


// In single quoted strings, escape sequences are included verbatim

#[test]
fn singlequoted_string_tab() {
    let code = "'\\t'"; // '\t' should parse literally
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\\t");
}

#[test]
fn singlequoted_string_newline() {
    let code = "'\\n'"; // '\n' should parse literally
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\\n");
}

#[test]
fn singlequoted_string_return() {
    let code = "'\\r'"; // '\r' should parse literally
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\\r");
}

#[test]
fn singlequoted_string_doublequote() {
    let code = "'\\\"'"; // '\"' should parse literally
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\\\"");
}

#[test]
fn singlequoted_string_singlequote() {
    let code = "'\\''"; // '\'' should parse as a string containing a single singlequote character (ASCII 0x27)
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "'");
}

#[test]
fn singlequoted_string_backslash() {
    let code = "'\\\\'"; // '\\' should parse literally
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\\\\");
}

#[test]
fn singlequoted_string_unicode() {
    let code = "'\\u{0201}'"; // '\u{0201}' should parse literally
    let reader = std::io::Cursor::new(code);    
    let scanner = Scanner::new("test", reader);
    let tokenizer = Tokenizer::new(scanner);
    println!("{}", tokenizer.current());
    assert_eq!(tokenizer.current().kind(), TokenKind::String);
    assert_eq!(tokenizer.current().lexeme(), "\\u{0201}");
}

