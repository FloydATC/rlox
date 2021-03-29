
#[allow(dead_code)]
pub enum TokenKind {
    // Single symbol
    PLUS,
    MINUS,
    STAR,
    SLASH,

    // Double symbol

    // Literals
    BASE10NUMBER,
    
    // Keywords
    RETURN,
    
    // Internal
    ERROR,
    EOF,
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


impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::RETURN => write!(f, "RETURN"),
            _ => write!(f, "**BAD**"),
        }
    }
}
