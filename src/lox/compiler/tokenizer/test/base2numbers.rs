use crate::lox::compiler::{Scanner, Tokenizer, Tokenize, TokenKind};


#[test]
fn tokenizer_base2number_digits() {
    for digit in '0'..='1' {
        let code = format!("0b{}", digit);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base2Number);
        assert_eq!(tokenizer.current().lexeme(), format!("0b{}", digit).as_str());
    }
}

#[test]
fn tokenizer_base2number_numbers() {
    for number in 0..=100 {
        let code = format!("0b{:08b}", number);
        let want = format!("0b{:08b}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base2Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base2number_numbers_ignore_decimals() {
    for number in 0..=100 {
        let code = format!("0b{:08b}.5", number);
        let want = format!("0b{:08b}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base2Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base2number_numbers_cutoff_nonbinary() {
    for number in 0..=100 {
        let code = format!("0b{:08b}5", number);
        let want = format!("0b{:08b}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base2Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

