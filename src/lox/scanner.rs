
#[cfg(test)]
mod test;

// Scanner object takes input &str, 
// stores it internally as Vec<char>

// Returns char at pos+0, pos+1 or pos+2 
// while tracking current lineno and charno

// Note: Peeking beyond EOF is okay, returns '\0'

// ======== Layout ========
pub struct Scanner {
    pos:	usize,
    len:	usize,
    chars:	Vec<char>,
    fileno:	usize,
    lineno:	usize,
    charno:	usize,
}


// ======== Public interface ========
#[allow(dead_code)]
impl Scanner {

    // Constructor
    pub fn str(code: &str) -> Scanner {
        let vec: Vec<char> = code.chars().collect();
        Scanner {
            pos:	0,
            len:	vec.len(),
            chars:	vec,
            fileno:	0,
            lineno:	1,
            charno:	1,
        }
    }

    // Increment pos unless we have reached eof
    pub fn advance(&mut self) {
        if !self.eof() {
            // Track lineno, charno
            if self.current() == '\n' {
                self.lineno = self.lineno + 1;
                self.charno = 1;
            } else {
                self.charno = self.charno + 1;
            }
            self.pos = self.pos + 1;
        }
    }
    
    // Return a tuple with current (fileno, lineno, charno)
    pub fn at(&self) -> (usize, usize, usize) {
        return (self.fileno, self.lineno, self.charno);
    }
    
    // Return char at pos+0 (or zero if eof)    
    pub fn current(&self) -> char {
        if self.pos+0 < self.len {
            return self.chars[self.pos+0];
        } else {
            return '\0';
        }
    }
    
    // Return char at pos+1 (or zero if eof)
    pub fn peek(&self) -> char {
        if self.pos+1 < self.len {
            return self.chars[self.pos+1];
        } else {
            return '\0';
        }
    }
    
    // Return char at pos+2 (or zero if eof)
    pub fn peek_next(&self) -> char {
        if self.pos+2 < self.len {
            return self.chars[self.pos+2];
        } else {
            return '\0';
        }
    }

    // Return true if current() char matches
    pub fn matches(&self, c: char) -> bool {
        return self.current() == c;
    }
    
    // Skip char c, panic if current char does not match
    pub fn skip(&mut self, c: char) {
        if self.matches(c) {
            self.advance();
        } else {
            panic!("Current char is {} not {}", self.current(), c);
        }
    }

    // Return true if pos is at eof
    pub fn eof(&self) -> bool {
        return self.pos >= self.len;
    }
}
