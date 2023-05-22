

use std::{rc::Rc, cell::RefCell};


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct At {
    pos: usize,
    lineno: usize,
    charno: usize, 
    filename: Rc<RefCell<String>>,
}


impl At {

    pub fn new(filename: &str) -> At {
        return At {
            pos: 0,
            lineno: 1,
            charno: 1,
            filename: Rc::new(RefCell::new(String::from(filename))),
        };
    }


    pub fn incr_line(&mut self) {
        self.charno = 1;
        self.lineno = self.lineno + 1;
        self.pos = self.pos + 1;
    }


    pub fn incr_char(&mut self) {
        self.charno = self.charno + 1;
        self.pos = self.pos + 1;
    }


    pub fn lineno(&self) -> usize {
        self.lineno
    }


    pub fn charno(&self) -> usize {
        self.charno
    }


    pub fn filename(&self) -> std::cell::Ref<String> {
        self.filename.borrow()
    }

}


impl std::fmt::Display for At {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {} char {} of {}", self.lineno(), self.charno(), self.filename())
    }
}
