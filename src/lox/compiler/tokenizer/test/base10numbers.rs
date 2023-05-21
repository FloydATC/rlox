

use crate::lox::compiler::{Scanner, Tokenizer, Tokenize, TokenKind};


#[test]
fn tokenizer_base10number_digits() {
    for digit in '0'..='9' {
        let code = format!("{}", digit);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        assert_eq!(tokenizer.current().lexeme(), format!("{}", digit).as_str());
    }
}

#[test]
fn tokenizer_base10number_whole_numbers() {
    for number in 0..=100 {
        let code = format!("{}", number);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        assert_eq!(tokenizer.current().lexeme(), format!("{}", number).as_str());
    }
}

#[test]
fn tokenizer_base10number_whole_numbers_leading_0() {
    for number in 0..=100 {
        let code = format!("0{}", number);
        let want = format!("0{}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base10number_whole_numbers_leading_00() {
    for number in 0..=100 {
        let code = format!("00{}", number);
        let want = format!("00{}", number);
        println!("number={} code={} want={}", number, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base10number_fractions() {
    for number in 0..=100 {
        let fraction = number as f32/25.0;
        let code = format!("{}", fraction);
        println!("number={} fraction={} code={}", number, fraction, code);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        assert_eq!(tokenizer.current().lexeme(), format!("{}", fraction).as_str());
    }
}

#[test]
fn tokenizer_base10number_fractions_cutoff_trailing_dots() {
    for number in 0..=100 {
        let fraction = number as f32/25.0;
        let code = format!("{}.", fraction);
        let want = format!("{}", fraction);
        println!("number={} fraction={} code={} want={}", number, fraction, code, want);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        assert_eq!(tokenizer.current().lexeme(), want);
    }
}

#[test]
fn tokenizer_base10number_fractions_cutoff_trailing_decimals() {
    for number in 0..=100 {
        let fraction = number as f32/25.0;
        let code = format!("{}.00", fraction);
        println!("number={} fraction={} code={}", number, fraction, code);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        println!("lexeme={}", tokenizer.current().lexeme());
        assert_eq!(tokenizer.current().lexeme().parse::<f32>(), Ok(fraction));
    }
}

#[test]
fn tokenizer_base10number_fractions_cutoff_trailing_characters() {
    for number in 0..=100 {
        let fraction = number as f32/25.0;
        let code = format!("{}.foo", fraction);
        println!("number={} fraction={} code={}", number, fraction, code);
        let reader = std::io::Cursor::new(code);    
        let scanner = Scanner::new(reader);
        let tokenizer = Tokenizer::new(scanner);
        assert_eq!(tokenizer.current().kind(), TokenKind::Base10Number);
        println!("lexeme={}", tokenizer.current().lexeme());
        assert_eq!(tokenizer.current().lexeme().parse::<f32>(), Ok(fraction));
    }
}
