
#[cfg(test)]
mod test;


use utf8conv::FromUtf8;


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
impl<R: std::io::BufRead> Scanner<R> {

    // Constructor
    pub fn new(filename: &str, reader: R) -> Scanner<R> {
        Scanner {
            reader,
            at: At::new(filename),
        }
    }

}


// ======== Private ========
impl<R: std::io::BufRead> Scanner<R> {

    fn nth_char(&mut self, count: usize) -> char {
        match self.reader.fill_buf() {
            Ok(buffer) => {

                if buffer.len() == 0 { return '\0'; } // Fail early
                let mut utf8_parser = FromUtf8::new();
                let mut buf_iter = buffer.iter();
                let mut char_iter = utf8_parser.utf8_ref_to_char_with_iter(&mut buf_iter);
                match char_iter.nth(count) {
                    Some(ch) => return ch,
                    None => return '\0',
                }

            }
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }
    }

}


impl<R: std::io::BufRead> Scan for Scanner<R> {

    // Increment pos unless we have reached eof
    fn advance(&mut self) {
        if !self.eof() {
            // Track lineno, charno
            let ch = self.current();
            let ch_len = ch.len_utf8();
            if ch == '\n' { self.at.incr_line() } else { self.at.incr_char() }
            self.at.incr_pos(ch_len);
            self.reader.consume(ch_len);
        }
    }
    

    // Return an object describing the current read position in the input stream
    fn at(&self) -> &At {
        return &self.at;
    }
    

    // Return char at pos+0 (or zero if eof)    
    fn current(&mut self) -> char {
        return self.nth_char(0);
    }


    // Return char at pos+1 (or zero if eof)
    fn peek(&mut self) -> char {
        return self.nth_char(1);
    }


    // Return char at pos+2 (or zero if eof)
    fn peek_next(&mut self) -> char {
        return self.nth_char(2);
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
