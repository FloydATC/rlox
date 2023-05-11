

use crate::lox::{At, TokenKind};


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
    

    pub fn synthetic(&self, lexeme: &str, kind: TokenKind) -> Self {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            at: self.at.clone(),
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


    pub fn get_at(&self) -> Option<&At> {
        return self.at.as_ref();
    }

}


impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        
            // Single character symbols
            TokenKind::Amp		=> write!(f, "Amp"),
            TokenKind::Bang 		=> write!(f, "Bang"),
            TokenKind::Comma 		=> write!(f, "Comma"),
            TokenKind::Dot 		=> write!(f, "Dot"),
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
            TokenKind::Break		=> write!(f, "Break"),
            TokenKind::Class		=> write!(f, "Class"),
            TokenKind::Continue		=> write!(f, "Continue"),
            TokenKind::Debug		=> write!(f, "Debug"),
            TokenKind::Else		=> write!(f, "Else"),
            TokenKind::Exit		=> write!(f, "Exit"),
            TokenKind::Fun		=> write!(f, "Fun"),
            TokenKind::If		=> write!(f, "If"),
            TokenKind::Of		=> write!(f, "Of"),
            TokenKind::Print		=> write!(f, "Print"),
            TokenKind::Return 		=> write!(f, "Return"),
            TokenKind::Super 		=> write!(f, "Super"),
            TokenKind::This 		=> write!(f, "This"),
            TokenKind::Var		=> write!(f, "Var"),
            TokenKind::While		=> write!(f, "While"),
            
            // Internal
            TokenKind::Error 		=> write!(f, "Error"),
            TokenKind::EOF 		=> write!(f, "EOF"),
            
        }
    }
}
