
#[cfg(test)]
mod test;


use super::{Token, TokenKind};
use super::{Scan, Scanner, Scanners};
use crate::lox::common::keyword::*;


#[derive(PartialEq)]
enum IncludeTimes {
    Once,
    Any,
}


pub trait Tokenize {
    fn current(&self) -> &Token;
    fn previous(&self) -> &Token;
    fn eof(&self) -> bool;
    fn advance(&mut self);
    fn matches(&self, kind: TokenKind) -> bool;
    fn advance_on(&mut self, kind: TokenKind) -> bool;
}


// ======== Layout ========
#[allow(dead_code)]
pub struct Tokenizer<'a> {
    scanners: Scanners<'a>,
    current: Option<Token>,
    previous: Option<Token>,
    library: String,
    included: Vec<String>,
}


// ======== Public interface ========
#[allow(dead_code)]
impl<'a> Tokenizer<'a> {

    pub fn new(scanner: impl Scan + 'a) -> Tokenizer<'a> {
        return Tokenizer::new_with_library(scanner, "lib/");
    }


    // Construct a Tokenizer that uses Scanner for input
    pub fn new_with_library(scanner: impl Scan + 'a, library: &str) -> Tokenizer<'a> {
        let mut tokenizer = Tokenizer {
            scanners: 	Scanners::new(scanner),
            current: 	None,
            previous:	None,
            library: String::from(library),
            included: vec![],
        };
        tokenizer.advance();
        return tokenizer;
    }

}


impl<'a> Tokenize for Tokenizer<'a> {

    // Return a reference to current token
    fn current(&self) -> &Token {
        match &self.current {
            Some(token) => &token,
            None => panic!("No current Token")
        }
    }
    
    // Return a reference to previous token
    fn previous(&self) -> &Token {
        match &self.previous {
            Some(token) => &token,
            None => panic!("No current Token")
        }
    }
    
    // Return true if current token is EOF
    // Otherwise return false
    fn eof(&self) -> bool {
        return self.matches(TokenKind::EOF);
    }

    // Advance to next token
    fn advance(&mut self) {
        self.previous = self.current.take();

        self.skip_whitespace();

        // Process directives, if any
        while self.scanner().current() == '#' { 
            if let Err(msg) = self.directive() {
                self.current = Some(Token::new_at(TokenKind::Error, msg.as_str(), self.scanner().at()));
                return;
            }
            self.skip_whitespace();
        }

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
    fn matches(&self, kind: TokenKind) -> bool {
        return self.current().matches(kind);
    }
    
    // Advance and return true if current tokenkind matches
    // Otherwise return false
    fn advance_on(&mut self, kind: TokenKind) -> bool {
        if !self.matches(kind) { return false; }
        self.advance();
        return true;
    }

}


// ======== Private methods ========
impl<'a> Tokenizer<'a> {
    fn scanner(&mut self) -> &mut dyn Scan {
        return &mut self.scanners;
    }

    // Use scanner to produce next Token
    fn scan_next_token(&mut self) -> Token {
        let c = self.scanner().current();
        if is_alpha(c) { return self.identifier_token(); }
        if is_base10digit(c) { return self.number_token(); }
        // Not an identifier or a number so it must be a symbol
        return self.symbol_token();        
    }

    fn directive(&mut self) -> Result<(), String> {
        self.scanner().advance(); // Consume '#'
        let at = self.scanner().at();
        let mut directive = String::new();
        while is_alphanum(self.scanner().current()) || self.scanner().matches('_') {
            directive.push(self.scanner().current());
            self.scanner().advance();    
        }
        match directive.as_str() {
            "include" => self.include_directive(IncludeTimes::Any, at),
            "include_once" => self.include_directive(IncludeTimes::Once, at),
            _ => Err(format!("Bad directive '{}' at {:?}", directive, at)),
        }
    }

    fn include_directive(&mut self, times: IncludeTimes, at: (usize, usize, usize)) -> Result<(), String> {
        match self.scanner().current() {
            '<' => self.include_library_file(times, at),
            '"' => self.include_user_file(times, at),
            _ => Err(format!("Expected '<' or '\"' after #include directive at {:?}", at)),
        }
    }

    fn parse_filename(&mut self, until: char) -> String {
        self.scanner().advance(); // Consume leading terminator
        let mut filename = String::new();
        while self.scanner().current() != until && !self.scanner().eof() {
            filename.push(self.scanner().current());
            self.scanner().advance();
        }
        if !self.scanner().eof() { self.scanner().advance(); } // Consume trailing terminator
        return filename;
    }

    fn include_library_file(&mut self, times: IncludeTimes, at: (usize, usize, usize)) -> Result<(), String> {
        let filename = self.parse_filename('>');
        let path = format!("{}{}", self.library, filename);
        self.include_file("library", path, times, at)
    }

    fn include_user_file(&mut self, times: IncludeTimes, at: (usize, usize, usize)) -> Result<(), String> {
        let path = self.parse_filename('"');
        self.include_file("user", path, times, at)
    }

    fn include_file(&mut self, libtype: &str, path: String, times: IncludeTimes, at: (usize, usize, usize)) -> Result<(), String> {
        if self.included.contains(&path) && times == IncludeTimes::Once { 
            // File already included once
            return Ok(()); 
        }
        match std::fs::File::open(&path) {
            Err(io_error) => {
                Err(format!("Error including {} file {} at {:?}: {}", libtype, path, at, io_error))
            },
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let scanner = Scanner::new(reader);
                self.scanners.include(scanner);
                self.included.push(path);
                Ok(())
            }
        }
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
            KEYWORD_BREAK 	=> return Token::new_at(TokenKind::Break, &lexeme, at),
            KEYWORD_CLASS 	=> return Token::new_at(TokenKind::Class, &lexeme, at),
            KEYWORD_CONTINUE 	=> return Token::new_at(TokenKind::Continue, &lexeme, at),
            KEYWORD_DEBUG 	=> return Token::new_at(TokenKind::Debug, &lexeme, at),
            KEYWORD_ELSE 	=> return Token::new_at(TokenKind::Else, &lexeme, at),
            KEYWORD_EXIT 	=> return Token::new_at(TokenKind::Exit, &lexeme, at),
            KEYWORD_FOR 	=> return Token::new_at(TokenKind::For, &lexeme, at),
            KEYWORD_FUN 	=> return Token::new_at(TokenKind::Fun, &lexeme, at),
            KEYWORD_FALSE 	=> return Token::new_at(TokenKind::False, &lexeme, at),
            KEYWORD_IF 	=> return Token::new_at(TokenKind::If, &lexeme, at),
            KEYWORD_IN 	=> return Token::new_at(TokenKind::In, &lexeme, at),
            KEYWORD_INF 	=> return Token::new_at(TokenKind::Inf, &lexeme, at),
            KEYWORD_IS 	=> return Token::new_at(TokenKind::Is, &lexeme, at),
            KEYWORD_NAN 	=> return Token::new_at(TokenKind::Nan, &lexeme, at),
            KEYWORD_NOT 	=> return Token::new_at(TokenKind::Not, &lexeme, at),
            KEYWORD_NULL 	=> return Token::new_at(TokenKind::Null, &lexeme, at),
            KEYWORD_OF 	=> return Token::new_at(TokenKind::Of, &lexeme, at),
            KEYWORD_PRINT 	=> return Token::new_at(TokenKind::Print, &lexeme, at),
            KEYWORD_RETURN 	=> return Token::new_at(TokenKind::Return, &lexeme, at),
            KEYWORD_SUPER 	=> return Token::new_at(TokenKind::Super, &lexeme, at),
            KEYWORD_THIS 	=> return Token::new_at(TokenKind::This, &lexeme, at),
            KEYWORD_TRUE 	=> return Token::new_at(TokenKind::True, &lexeme, at),
            KEYWORD_VAR 	=> return Token::new_at(TokenKind::Var,	&lexeme, at),
            KEYWORD_WHILE 	=> return Token::new_at(TokenKind::While, &lexeme, at),
            _ => return Token::new_at(TokenKind::Identifier, &lexeme, at),
        }
    }
    

    // First character is 0-9
    fn number_token(&mut self) -> Token {
        let current_is_zero = self.scanner().current() == '0';
        match (current_is_zero, self.scanner().peek()) {
            (true, 'b') => self.number_base2(),
            (true, 'o') => self.number_base8(),
            (true, 'x') => self.number_base16(),
            _           => self.number_base10(),
        }
    }

    fn number_prefix(&mut self, want: &str) -> String {
        let mut lexeme = String::new();
        for _ in want.chars() {
            lexeme.push(self.scanner().current());
            self.scanner().advance();    
        }
        debug_assert_eq!(lexeme.as_str(), want);
        return lexeme;
    }

    fn number_base2(&mut self) -> Token {
        let at = self.scanner().at();
        let mut lexeme = self.number_prefix("0b");
        while is_base2digit(self.scanner().current()) {
            lexeme.push(self.scanner().current());
            self.scanner().advance();    
        }
        return Token::new_at(TokenKind::Base2Number, &lexeme, at);
    }


    fn number_base8(&mut self) -> Token {
        let at = self.scanner().at();
        let mut lexeme = self.number_prefix("0o");
        while is_base8digit(self.scanner().current()) {
            lexeme.push(self.scanner().current());
            self.scanner().advance();    
        }
        return Token::new_at(TokenKind::Base8Number, &lexeme, at);
    }


    fn number_base10(&mut self) -> Token {
        let at = self.scanner().at();
        let mut lexeme = String::new();
        let mut dots = 0;
        // Allow decimal point if immediately followed by another digit...
        while is_base10digit(self.scanner().current()) || (is_dot(self.scanner().current()) && is_base10digit(self.scanner().peek())) {
            if is_dot(self.scanner().current()) {
                dots = dots + 1;
                if dots > 1 { break; } // ...but only one
            }
            lexeme.push(self.scanner().current());
            self.scanner().advance();    
        }
        return Token::new_at(TokenKind::Base10Number, &lexeme, at);
    }


    fn number_base16(&mut self) -> Token {
        let at = self.scanner().at();
        let mut lexeme = self.number_prefix("0x");
        while is_base16digit(self.scanner().current()) {
            lexeme.push(self.scanner().current());
            self.scanner().advance();    
        }
        return Token::new_at(TokenKind::Base16Number, &lexeme, at);
    }


    fn escape_sequence(&mut self) -> Result<String, String> {
        let c = self.scanner().current();
        self.scanner().advance(); // Consume c
        match c {
            't'		=> return Ok("\t".to_string()),
            'n' 	=> return Ok("\n".to_string()),
            'r' 	=> return Ok("\r".to_string()),
            '"' 	=> return Ok("\"".to_string()),
            '\'' 	=> return Ok("'".to_string()),
            '\\' 	=> return Ok("\\".to_string()),
            _		=> {
                return Err(format!("Character sequence not supported: '\\{}'", c));
            }
        }
    }

    // Single or double quoted string
    fn string_token(&mut self) -> Token {
        let at = self.scanner().at();
        let quote = self.scanner().current();
        let mut string = String::new();
        self.scanner().advance(); // Consume leading quote
        while self.scanner().current() != quote {
            if self.scanner().eof() {
                return Token::new_at(TokenKind::Error, "Unterminated string", at);
            }
            let c = self.scanner().current();
            if c == '\\' {
                self.scanner().advance(); // Consume backslash
                let result = self.escape_sequence();
                match result {
                    Ok(unescaped) => {
                        string = string + unescaped.as_str();
                    }
                    Err(msg) => {
                        return Token::new_at(TokenKind::Error, msg.as_str(), at);
                    }
                }
            } else {
                string.push(c);
                self.scanner().advance(); // Consume c
            }
        }
        self.scanner().advance(); // Consume trailing quote
        return Token::new_at(TokenKind::String, string.as_str(), at);
    }

    // First character is not alphanumerical so it must be a symbol
    fn symbol_token(&mut self) -> Token {
        let at = self.scanner().at();
        match self.scanner().current() {
            ',' => return self.make_token_at(",", TokenKind::Comma, at),
            '.' => return self.make_token_at(".", TokenKind::Dot, at),
            '+' => return self.make_token_at("+", TokenKind::Plus, at),
            '-' => return self.make_token_at("-", TokenKind::Minus, at),
            '*' => return self.make_token_at("*", TokenKind::Star, at),
            '/' => return self.make_token_at("/", TokenKind::Slash, at),
            '%' => return self.make_token_at("%", TokenKind::Percent, at),
            ';' => return self.make_token_at(";", TokenKind::Semicolon, at),
            '[' => return self.make_token_at("[", TokenKind::LeftBracket, at),
            '{' => return self.make_token_at("{", TokenKind::LeftCurly, at),
            '(' => return self.make_token_at("(", TokenKind::LeftParen, at),
            ']' => return self.make_token_at("]", TokenKind::RightBracket, at),
            '}' => return self.make_token_at("}", TokenKind::RightCurly, at),
            ')' => return self.make_token_at(")", TokenKind::RightParen, at),
            '&' => {
                match self.scanner().peek() {
                    '&' => return self.make_token_at("&&", TokenKind::AmpAmp, at),
                    _ => return self.make_token_at("&", TokenKind::Amp, at),
                }
            }
            '|' => {
                match self.scanner().peek() {
                    '|' => return self.make_token_at("||", TokenKind::PipePipe, at),
                    _ => return self.make_token_at("|", TokenKind::Pipe, at),
                }
            }
            '>' => {
                match self.scanner().peek() {
                    '=' => return self.make_token_at(">=", TokenKind::GreaterEqual, at),
                    _ => return self.make_token_at(">", TokenKind::Greater, at),
                }
            }
            '<' => {
                match self.scanner().peek() {
                    '=' => return self.make_token_at("<=", TokenKind::LessEqual, at),
                    _ => return self.make_token_at("<", TokenKind::Less, at),
                }
            }
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
            '"' | '\'' => return self.string_token(),
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

// Return true if c is 0..9
pub fn is_dot(c: char) -> bool {
    return c == '.';
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

