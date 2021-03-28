
use super::scanner::Scanner;


#[allow(dead_code)]
pub struct Parser {
    scanner: Scanner,
}


#[allow(dead_code)]
impl Parser {
    pub fn new(scanner: Scanner) -> Parser {
        println!("Parser::new()");
        Parser {
            scanner,
        }
    }
}


impl Drop for Parser {
    fn drop(&mut self) {
        println!("Parser.drop()");
    }
}
