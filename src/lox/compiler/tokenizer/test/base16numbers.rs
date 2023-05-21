use crate::lox::compiler::{Scanner, Tokenizer, Tokenize, TokenKind};


#[test]
fn tokenizer_base16number_lowercase_digits() {
    for digit in ('0'..='9').chain('a'..='f') {
        let code = format!("0x{}", digit);
        let want = format!("0x{}", digit);
        println!("digit={} code={} want={}", digit, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base16Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base16number_uppercase_digits() {
    for digit in ('0'..='9').chain('A'..='F') {
        let code = format!("0x{}", digit);
        let want = format!("0x{}", digit);
        println!("digit={} code={} want={}", digit, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base16Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base16number_numbers() {
    for number in 0..=100 {
        let code = format!("0x{:02x}", number);
        let want = format!("0x{:02x}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base16Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base16number_numbers_ignore_decimals() {
    for number in 0..=100 {
        let code = format!("0x{:02x}.5", number);
        let want = format!("0x{:02x}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base16Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base16number_numbers_cutoff_nonhex() {
    for number in 0..=100 {
        let code = format!("0x{:02x}g", number);
        let want = format!("0x{:02x}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base16Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

