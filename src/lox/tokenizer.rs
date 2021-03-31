
mod test;


use super::token::{Token, TokenKind};
use super::scanner::Scanner;


// ======== Layout ========
#[allow(dead_code)]
pub struct Tokenizer {
    scanner: Option<Scanner>,
    current: Option<Token>,
    previous: Option<Token>,
}


// ======== Public interface ========
#[allow(dead_code)]
impl Tokenizer {

    // Construct a Tokenizer that uses Scanner for input
    pub fn new(scanner: Scanner) -> Tokenizer {
        let mut tokenizer = Tokenizer {
            scanner: 	Some(scanner),
            current: 	None,
            previous:	None,
        };
        tokenizer.advance();
        return tokenizer;
    }
    
    // Return a reference to current token
    pub fn current(&self) -> &Token {
        match &self.current {
            Some(token) => &token,
            None => panic!("No current Token")
        }
    }
    
    // Return a reference to previous token
    pub fn previous(&self) -> &Token {
        match &self.previous {
            Some(token) => &token,
            None => panic!("No current Token")
        }
    }
    
    // Return true if current token is EOF
    // Otherwise return false
    pub fn eof(&self) -> bool {
        return self.matches(TokenKind::EOF);
    }

    // Advance to next token
    pub fn advance(&mut self) {
        self.previous = self.current.take();

        self.skip_whitespace();

        let token;
        if self.scanner().eof() {

            // EOF will require some special handling when we get to 
            // #include directives. For now, simply produce an EOF token.
            token = Token::new_at(TokenKind::EOF, "\0", self.scanner().at());

        } else {
            token = self.scan_next_token();
        }
        
        self.current = Some(token);
        
        //println!("Tokenizer.advance() previous={:?}, current={:?}", self.previous, self.current);
    }

    // Return true if current tokenkind matches
    pub fn matches(&self, kind: TokenKind) -> bool {
        return self.current().matches(kind);
    }
    
    // Advance and return true if current tokenkind matches
    // Otherwise return false
    pub fn advance_on(&mut self, kind: TokenKind) -> bool {
        if !self.matches(kind) { return false; }
        self.advance();
        return true;
    }
}


// ======== Private methods ========
impl Tokenizer {
    fn scanner(&mut self) -> &mut Scanner {
        return self.scanner.as_mut().unwrap();
    }

    // Use scanner to produce next Token
    fn scan_next_token(&mut self) -> Token {
        let c = self.scanner().current();
        if is_alpha(c) { return self.identifier_token(); }
        if is_base10digit(c) { return self.number_token(); }
        // Not an identifier or a number so it must be a symbol
        return self.symbol_token();        
    }
    
    // First character is a-z or A-Z
    fn identifier_token(&mut self) -> Token {
        let at = self.scanner().at();
        let mut lexeme = String::new();
        while is_alphanum(self.scanner().current()) || self.scanner().matches('_') {
            lexeme.push(self.scanner().current());
            self.scanner().advance();    
        }
        match lexeme.as_str() {
            "return" => return Token::new_at(TokenKind::Return, &lexeme, at),
            _ => return Token::new_at(TokenKind::Identifier, &lexeme, at),
        }
    }
    
    // First character is 0-9
    fn number_token(&mut self) -> Token {
        let at = self.scanner().at();
        let mut lexeme = String::new();
        while is_base10digit(self.scanner().current()) {
            lexeme.push(self.scanner().current());
            self.scanner().advance();    
        }
        match lexeme.as_str() {
            _ => return Token::new_at(TokenKind::Base10Number, &lexeme, at),
        }
    }

    // First character is not alphanumerical so it must be a symbol
    fn symbol_token(&mut self) -> Token {
        let at = self.scanner().at();
        match self.scanner().current() {
            '+' => return self.make_token_at("+", TokenKind::Plus, at),
            '-' => return self.make_token_at("-", TokenKind::Minus, at),
            '*' => return self.make_token_at("*", TokenKind::Star, at),
            '/' => return self.make_token_at("/", TokenKind::Slash, at),
            ';' => return self.make_token_at(";", TokenKind::Semicolon, at),
            '!' => {
                match self.scanner().peek() {
                    '=' => return self.make_token_at("!=", TokenKind::BangEqual, at),
                    _ => return self.make_token_at("!", TokenKind::Bang, at),
                }
            }
            '=' => {
                match self.scanner().peek() {
                    '=' => return self.make_token_at("==", TokenKind::EqualEqual, at),
                    _ => return self.make_token_at("=", TokenKind::Equal, at),
                }
            }
            _ => {
                // Bad/unknown symbol encountered, return an Error token
                let lexeme = self.scanner().current().to_string();
                self.make_token_at(&lexeme, TokenKind::Error, at)
            }
        }
    }
    
    // Make a token and scan past the lexeme
    fn make_token_at(&mut self, lexeme: &str, kind: TokenKind, at: (usize, usize, usize)) -> Token {
        for _c in lexeme.chars() { self.scanner().advance(); }
        return Token::new_at(kind, lexeme, at);
    }
    
    // Scan forward until we find something that isn't whitespace    
    fn skip_whitespace(&mut self) {
        loop {
            // Treat comments like whitespace
            if self.scanner().matches('/') {
                if self.scanner().peek() == '/' { self.skip_line_comment(); }
                if self.scanner().peek() == '*' { self.skip_block_comment(); }
            }
            // Stop if we found a non-whitespace character (incl. EOF)
            if !is_whitespace(self.scanner().current()) { break; }
            // No? Keep scanning
            self.scanner().advance();
        }
    }
    
    // A line comment (//) goes until the end of the line    
    fn skip_line_comment(&mut self) {
        println!("Tokenizer.skip_line_comment()");
        self.scanner().skip('/');
        self.scanner().skip('/');
        loop {
            if self.scanner().eof() { break; }
            if self.scanner().matches('\n') { break; }
            self.scanner().advance();
        }
    }

    // A block comment (/*) goes until (*/)
    fn skip_block_comment(&mut self) {
        self.scanner().skip('/');
        self.scanner().skip('*');
        loop {
            if self.scanner().eof() { break; }
            if self.scanner().matches('*') {
                if self.scanner().peek() == '/' {
                    self.scanner().skip('*');
                    self.scanner().skip('/');
                    break;
                }
            }
            self.scanner().advance();
        }
    }
}


// ======== Internal functions ========

// Return true if c is whitespace
pub fn is_whitespace(c: char) -> bool {
    match c {
        ' '	=> return true,
        '\t'    => return true,
        '\r'    => return true,
        '\n'    => return true,
        _       => return false,
    }
}

// Return true if c is a..z or A..Z
pub fn is_alpha(c: char) -> bool {
    return c >= 'a' && c <= 'z'
        || c >= 'A' && c <= 'Z';
}

// Return true if c is 0..1
#[allow(dead_code)]
pub fn is_base2digit(c: char) -> bool {
    return c >= '0' && c <= '1';
}

// Return true if c is 0..7
#[allow(dead_code)]
pub fn is_base8digit(c: char) -> bool {
    return c >= '0' && c <= '7';
}

// Return true if c is 0..9
pub fn is_base10digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

// Return true if c is 0..9 or a..f or A..F
#[allow(dead_code)]
pub fn is_base16digit(c: char) -> bool {
    return c >= '0' && c <= '9'
        || c >= 'a' && c <= 'f'
        || c >= 'A' && c <= 'F';
}

// Return true if c is alphanumerical
pub fn is_alphanum(c: char) -> bool {
    return is_alpha(c) || is_base10digit(c);
}

