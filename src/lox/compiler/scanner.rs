
#[cfg(test)]
mod test;


use crate::lox::common::At;


// Note: Peeking beyond EOF is okay, returns '\0'


pub trait Scan {
    fn advance(&mut self);
    fn at(&self) -> &At;
    fn current(&mut self) -> char;
    fn peek(&mut self) -> char;
    fn peek_next(&mut self) -> char;
    fn matches(&mut self, c: char) -> bool;
    fn skip(&mut self, c: char);
    fn eof(&mut self) -> bool;
}



// ======== Layout ========
pub struct Scanner<R> {
    reader: R,
    at: At,
}


// ======== Public interface ========
#[allow(dead_code)]
impl<R: std::io::BufRead+std::io::Read> Scanner<R> {

    // Constructor
    pub fn new(filename: &str, reader: R) -> Scanner<R> {
        Scanner {
            reader,
            at: At::new(filename),
        }
    }

}


impl<R: std::io::BufRead+std::io::Read> Scan for Scanner<R> {

    // Increment pos unless we have reached eof
    fn advance(&mut self) {
        if !self.eof() {
            // Track lineno, charno
            if self.current() == '\n' { self.at.incr_line() } else { self.at.incr_char() }
            let mut buf = [0x00u8];
            self.reader.read_exact(&mut buf)
                .unwrap_or_else(|io_error| panic!("read_exact() returned {}", io_error))
        }
    }
    

    // Return an object describing the current read position in the input stream
    fn at(&self) -> &At {
        return &self.at;
    }
    

    // Return char at pos+0 (or zero if eof)    
    fn current(&mut self) -> char {
        match self.reader.fill_buf() {
            Ok(buffer) => {
                if buffer.len() >= 1 { buffer[0] as char } else { '\0' }
            }
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }
    }


    // Return char at pos+1 (or zero if eof)
    fn peek(&mut self) -> char {
        match self.reader.fill_buf() {
            Ok(buffer) => {
                if buffer.len() >= 2 { buffer[1] as char } else { '\0' }
            }
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }
    }


    // Return char at pos+2 (or zero if eof)
    fn peek_next(&mut self) -> char {
        match self.reader.fill_buf() {
            Ok(buffer) => {
                if buffer.len() >= 3 { buffer[2] as char } else { '\0' }
            }
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }
    }


    // Return true if current() char matches
    fn matches(&mut self, c: char) -> bool {
        return self.current() == c;
    }


    // Skip char c, panic if current char does not match
    fn skip(&mut self, c: char) {
        if self.matches(c) {
            self.advance();
        } else {
            panic!("Current char is {} not {}", self.current(), c);
        }
    }


    // Return true if pos is at eof
    fn eof(&mut self) -> bool {
        match self.reader.fill_buf() {
            Ok(buffer) => buffer.len() == 0,
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }
    }

}