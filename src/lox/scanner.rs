
#[allow(dead_code)]
pub struct Scanner {
    
}


#[allow(dead_code)]
impl Scanner {
    pub fn new(_code: &str) -> Scanner {
        println!("Scanner::new()");
        Scanner {}
    }
}


impl Drop for Scanner {
    fn drop(&mut self) {
        println!("Scanner.drop()");
    }
}
