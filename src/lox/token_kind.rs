

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum TokenKind {
    // Single symbol
    Amp,
    Bang,
    Comma,
    Dot,
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
    Inf,
    Nan,
    Null,
    String,
    True,
    
    // Keywords
    Break,
    Class,
    Continue,
    Debug,
    Else,
    Exit,
    Fun,
    If,
    Is,
    Of,
    Print,
    Return,
    Super,
    This,
    Var,
    While,
    
    // Internal
    Error,
    EOF,
}
