use crate::lox::compiler::{Scanner, Tokenizer, Tokenize, TokenKind};


#[test]
fn tokenizer_base8number_digits() {
    for digit in '0'..='7' {
        let code = format!("0o{}", digit);
        let want = format!("0o{}", digit);
        println!("digit={} code={} want={}", digit, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new("test", reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base8Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base8number_numbers() {
    for number in 0..=100 {
        let code = format!("0o{:04o}", number);
        let want = format!("0o{:04o}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new("test", reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base8Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base2number_numbers_ignore_decimals() {
    for number in 0..=100 {
        let code = format!("0o{:04o}.5", number);
        let want = format!("0o{:04o}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new("test", reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base8Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base2number_numbers_cutoff_nonoctal() {
    for number in 0..=100 {
        let code = format!("0o{:04o}8", number);
        let want = format!("0o{:04o}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new("test", reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base8Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

