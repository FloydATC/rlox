

#[allow(unused_imports)]
use crate::lox::scanner::Scanner;
#[allow(unused_imports)]
use super::Tokenizer;


#[test]
fn tokenizer_new() {
    let scanner = Scanner::str("");
    let _tokenizer = Tokenizer::new(scanner);
}



