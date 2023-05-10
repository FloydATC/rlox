

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct At {
    pub fileno: usize,
    pub lineno: usize,
    pub charno: usize, 
}


impl At {
    pub fn new(at: (usize, usize, usize)) -> At {
        return At {
            fileno:	at.0,
            lineno:	at.1,
            charno:	at.2,
        };
    }
}


impl std::fmt::Display for At {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "file {} line {} char {}", self.fileno, self.lineno, self.charno)
    }
}
