
#[allow(dead_code)]
pub struct Scanner {
    code:	String,
}


#[allow(dead_code)]
impl Scanner {
    pub fn str(code: &str) -> Scanner {
        println!("Scanner::new()");
        let mut scanner = Scanner {
            code:	code.to_string(),
        };
        scanner.advance();
        return scanner;
    }
}
    
    
#[allow(dead_code)]
impl Scanner {
//    pub fn is_whitespace(c: char) -> bool {
//    }
    
    
//    pub fn is_alpha(c: char) -> bool {
//    }
    
    
//    pub fn is_b10num(c: char) -> bool {
//    }
    
    
//    pub fn is_alphanum(c: char) -> bool {
//        return is_alpha(c) || is_b10num(c);
//    }
}


#[allow(dead_code)]
impl Scanner {
    pub fn advance(&mut self) {
    }
    
    
//    pub fn current(&self) -> char {
//    }
    
    
//    pub fn peek(&self) -> char {
//    }
    
    
//    pub fn peek_next(&self) -> char {
//    }
    
    
//    pub fn eof(&self) -> bool {
//    }
}


impl Drop for Scanner {
    fn drop(&mut self) {
        println!("Scanner.drop()");
    }
}
