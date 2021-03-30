
mod test;


use super::token::{Token, TokenKind};
use super::scanner::Scanner;


#[allow(dead_code)]
pub struct Tokenizer {
    scanner: Option<Scanner>,
    current: Option<Token>,
    previous: Option<Token>,
}


#[allow(dead_code)]
impl Tokenizer {
    pub fn new(scanner: Scanner) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            scanner: 	Some(scanner),
            current: 	None,
            previous:	None,
        };
        tokenizer.advance();
        return tokenizer;
    }
    
    
    pub fn current(&self) -> &Token {
        match &self.current {
            Some(token) => &token,
            None => panic!("No current Token")
        }
    }
    
    
    pub fn previous(&self) -> &Token {
        match &self.current {
            Some(token) => &token,
            None => panic!("No current Token")
        }
    }
    

    pub fn eof(&self) -> bool {
        return self.matches(TokenKind::EOF);
    }

    
    pub fn advance(&mut self) {
        self.previous = self.current.take();
        
        let token = Token::new(TokenKind::RETURN, "return");
        self.current = Some(token);
    }
    
    
    pub fn matches(&self, kind: TokenKind) -> bool {
        return self.current().matches(kind);
    }
    
    
    pub fn advance_on(&mut self, kind: TokenKind) -> bool {
        if !self.matches(kind) { return false; }
        self.advance();
        return true;
    }
}


#[allow(dead_code)]
pub fn is_whitespace(c: char) -> bool {
    match c {
        ' '         => return true,
        '\t'        => return true,
        '\r'        => return true,
        '\n'        => return true,
        _           => return false,
    }
}


#[allow(dead_code)]
pub fn is_alpha(c: char) -> bool {
    return c >= 'a' && c <= 'z'
        || c >= 'A' && c <= 'Z';
}


#[allow(dead_code)]
pub fn is_b2digit(c: char) -> bool {
    return c >= '0' && c <= '1';
}


#[allow(dead_code)]
pub fn is_b8digit(c: char) -> bool {
    return c >= '0' && c <= '7';
}


pub fn is_b10digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}


#[allow(dead_code)]
pub fn is_b16digit(c: char) -> bool {
    return c >= '0' && c <= '9'
        || c >= 'a' && c <= 'f'
        || c >= 'A' && c <= 'F';
}


#[allow(dead_code)]
pub fn is_alphanum(c: char) -> bool {
    return is_alpha(c) || is_b10digit(c);
}
