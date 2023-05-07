
#[cfg(test)]
mod test;

// Scanner object takes input &str, 
// stores it internally as Vec<char>

// Returns char at pos+0, pos+1 or pos+2 
// while tracking current lineno and charno

// Note: Peeking beyond EOF is okay, returns '\0'


pub trait Scan {
    fn advance(&mut self);
    fn at(&self) -> (usize, usize, usize);
    fn current(&mut self) -> char;
    fn peek(&mut self) -> char;
    fn peek_next(&mut self) -> char;
    fn matches(&mut self, c: char) -> bool;
    fn skip(&mut self, c: char);
    fn eof(&mut self) -> bool;
}



// ======== Layout ========
pub struct Scanner<R> {
    pos:	usize,
    //len:	usize,
    //chars:	Vec<char>,
    reader: R,
    fileno:	usize,
    lineno:	usize,
    charno:	usize,
}


// ======== Public interface ========
#[allow(dead_code)]
impl<R: std::io::BufRead+std::io::Read> Scanner<R> {

    // Constructor
    pub fn str(code: &str) -> Scanner<std::io::Cursor<&str>> {
        Scanner::new(std::io::Cursor::new(code))
    }

    // Constructor
    pub fn new(reader: R) -> Scanner<R> {
        Scanner {
            pos:	0,
            reader,
            fileno:	0,
            lineno:	1,
            charno:	1,
        }
    }

}


impl<R: std::io::BufRead+std::io::Read> Scan for Scanner<R> {

    // Increment pos unless we have reached eof
    fn advance(&mut self) {
        if !self.eof() {
            // Track lineno, charno
            if self.current() == '\n' {
                self.lineno = self.lineno + 1;
                self.charno = 1;
            } else {
                self.charno = self.charno + 1;
            }
            self.pos = self.pos + 1;
            let mut buf = [0x00u8];
            self.reader.read_exact(&mut buf)
                .unwrap_or_else(|io_error| panic!("read_exact() returned {}", io_error))
        }
    }
    

    // Return a tuple with current (fileno, lineno, charno)
    fn at(&self) -> (usize, usize, usize) {
        return (self.fileno, self.lineno, self.charno);
    }
    

    // Return char at pos+0 (or zero if eof)    
    fn current(&mut self) -> char {
        match self.reader.fill_buf() {
            Ok(buffer) => {
                if buffer.len() >= 1 { buffer[0] as char } else { '\0' }
            }
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }

//        if self.pos+0 < self.len {
//            return self.chars[self.pos+0];
//        } else {
//            return '\0';
//        }
    }


    // Return char at pos+1 (or zero if eof)
    fn peek(&mut self) -> char {
        match self.reader.fill_buf() {
            Ok(buffer) => {
                if buffer.len() >= 2 { buffer[1] as char } else { '\0' }
            }
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }

//        if self.pos+1 < self.len {
//            return self.chars[self.pos+1];
//        } else {
//            return '\0';
//        }
    }


    // Return char at pos+2 (or zero if eof)
    fn peek_next(&mut self) -> char {
        match self.reader.fill_buf() {
            Ok(buffer) => {
                if buffer.len() >= 3 { buffer[2] as char } else { '\0' }
            }
            Err(io_error) => panic!("fill_buf() returned {}", io_error),
        }

//        if self.pos+2 < self.len {
//            return self.chars[self.pos+2];
//        } else {
//            return '\0';
//        }
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
        //return self.pos >= self.len;
    }

}
