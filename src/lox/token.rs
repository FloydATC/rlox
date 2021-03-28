
#[allow(dead_code)]
pub enum TokenKind {
    Return,
}


#[allow(dead_code)]
pub struct At {
    fileno: u32,
    lineno: u32,
    charno: u32, 
}


#[allow(dead_code)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    at: Option<At>,
}


#[allow(dead_code)]
impl Token {
    pub fn new(kind: TokenKind, lexeme: &str) -> Token {
        Token {
            kind,
            lexeme:	lexeme.to_string(),
            at:		None,
        }
    }
    pub fn new_at(kind: TokenKind, lexeme: &str, at: At) -> Token {
        Token {
            kind,
            lexeme:	lexeme.to_string(),
            at:		Some(at),
        }
    }
}
