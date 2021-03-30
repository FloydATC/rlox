

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


