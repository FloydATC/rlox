

#[cfg(test)]
mod test;


use super::Scan;


pub struct Scanners<'a> {
    scanners: Vec<Box<dyn Scan + 'a>>,
}


impl<'a> Scanners<'a> {

    pub fn new(scanner: impl Scan + 'a) -> Self {
        Scanners { 
            scanners: vec![Box::new(scanner)], 
        }
    }


    pub fn include(&mut self, scanner: impl Scan + 'a) {
        self.scanners.push(Box::new(scanner));
    }


    fn current_scanner(&self) -> & (dyn Scan + 'a) {
        return self.scanners.last().expect("Internal error: No current scanner").as_ref();
    }


    fn current_scanner_mut(&mut self) -> &mut (dyn Scan + 'a) {
        return self.scanners.last_mut().expect("Internal error: No current scanner").as_mut();
    }

}


impl<'a> Scan for Scanners<'a> {

    fn advance(&mut self) {
        self.current_scanner_mut().advance();
        while self.scanners.len() > 1 && self.current_scanner_mut().eof() {
            self.scanners.pop();
            println!("Discarded scanner, {} left", self.scanners.len());
        }
    }

    fn at(&self) -> (usize, usize, usize) {
        return self.current_scanner().at();
    }

    fn current(&mut self) -> char {
        return self.current_scanner_mut().current();
    }

    fn peek(&mut self) -> char {
        let mut skip = 1;
        for scanner in self.scanners.iter_mut().rev() {
            if scanner.current() != '\0' {
                if skip > 0 { skip = skip - 1; } else { return scanner.current(); }
            }
            if scanner.peek() != '\0' { return scanner.peek() }
        }
        return '\0';
    }

    fn peek_next(&mut self) -> char {
        let mut skip = 2;
        for scanner in self.scanners.iter_mut().rev() {
            if scanner.current() != '\0' {
                if skip > 0 { skip = skip - 1; } else { return scanner.current(); }
            }
            if scanner.peek() != '\0' { 
                if skip > 0 { skip = skip - 1; } else { return scanner.peek(); }
            }
            if scanner.peek_next() != '\0' { return scanner.peek_next() }
        }
        return '\0';
    }

    fn matches(&mut self, c: char) -> bool {
        return self.current_scanner_mut().matches(c);
    }

    fn skip(&mut self, c: char) {
        self.current_scanner_mut().skip(c);
    }

    fn eof(&mut self) -> bool {
        return self.scanners.len() == 1 && self.current_scanner_mut().eof();
    }

}
