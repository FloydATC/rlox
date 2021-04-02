
#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum TokenKind {
    // Single symbol
    Amp,
    Bang,
    Equal,
    Greater,
    LeftBracket,
    LeftCurly,
    LeftParen,
    Less,
    Minus,
    Percent,
    Pipe,
    Plus,
    RightBracket,
    RightCurly,
    RightParen,
    Semicolon,
    Slash,
    Star,

    // Double symbol
    AmpAmp,
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
    PipePipe,

    // Literals
    Base2Number,
    Base8Number,
    Base10Number,
    Base16Number,
    False,
    Identifier,
    Null,
    String,
    True,
    
    // Keywords
    Return,
    Var,
    
    // Internal
    Error,
    EOF,
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct At {
    fileno: usize,
    lineno: usize,
    charno: usize, 
}


impl At {
    fn new(at: (usize, usize, usize)) -> At {
        return At {
            fileno:	at.0,
            lineno:	at.1,
            charno:	at.2,
        };
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
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
    
    
    pub fn new_at(kind: TokenKind, lexeme: &str, at: (usize, usize, usize)) -> Token {
        let at = At::new(at);
        Token {
            kind,
            lexeme:	lexeme.to_string(),
            at:		Some(at),
        }
    }
    
    
    pub fn matches(&self, kind: TokenKind) -> bool {
        return self.kind == kind;
    }
    
    
    pub fn kind(&self) -> TokenKind {
        return self.kind;
    }
    
    
    pub fn lexeme(&self) -> &str {
        return &self.lexeme;
    } 
}


impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        
            // Single character symbols
            TokenKind::Amp		=> write!(f, "Amp"),
            TokenKind::Bang 		=> write!(f, "Bang"),
            TokenKind::Equal 		=> write!(f, "Equal"),
            TokenKind::Greater		=> write!(f, "Greater"),
            TokenKind::LeftBracket	=> write!(f, "LeftBracket"),
            TokenKind::LeftCurly	=> write!(f, "LeftCurly"),
            TokenKind::LeftParen	=> write!(f, "LeftParen"),
            TokenKind::Less		=> write!(f, "Less"),
            TokenKind::Minus 		=> write!(f, "Minus"),
            TokenKind::Percent          => write!(f, "Percent"),
            TokenKind::Pipe		=> write!(f, "Pipe"),
            TokenKind::Plus 		=> write!(f, "Plus"),
            TokenKind::RightBracket	=> write!(f, "RightBracket"),
            TokenKind::RightCurly	=> write!(f, "RightCurly"),
            TokenKind::RightParen	=> write!(f, "RightParen"),
            TokenKind::Semicolon	=> write!(f, "Semicolon"),
            TokenKind::Slash 		=> write!(f, "Slash"),
            TokenKind::Star 		=> write!(f, "Star"),
            
            // Double character symbols
            TokenKind::AmpAmp		=> write!(f, "AmpAmp"),
            TokenKind::BangEqual 	=> write!(f, "BangEqual"),
            TokenKind::EqualEqual 	=> write!(f, "EqualEqual"),
            TokenKind::GreaterEqual	=> write!(f, "GreaterEqual"),
            TokenKind::LessEqual	=> write!(f, "LessEqual"),
            TokenKind::PipePipe		=> write!(f, "PipePipe"),
            
            // Literals
            TokenKind::Base2Number 	=> write!(f, "Base2Number"),
            TokenKind::Base8Number 	=> write!(f, "Base8Number"),
            TokenKind::Base10Number 	=> write!(f, "Base10Number"),
            TokenKind::Base16Number 	=> write!(f, "Base16Number"),
            TokenKind::False		=> write!(f, "False"),
            TokenKind::Identifier 	=> write!(f, "Identifier"),
            TokenKind::Null		=> write!(f, "Null"),
            TokenKind::String		=> write!(f, "String"),
            TokenKind::True		=> write!(f, "True"),
            
            // Keywords
            TokenKind::Return 		=> write!(f, "Return"),
            TokenKind::Var		=> write!(f, "Var"),
            
            // Internal
            TokenKind::Error 		=> write!(f, "Error"),
            TokenKind::EOF 		=> write!(f, "EOF"),
            
        }
    }
}
