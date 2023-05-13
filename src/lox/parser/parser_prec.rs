

#[cfg(test)]
mod test;


#[repr(u8)]
#[derive(PartialOrd, PartialEq, Debug)]
pub enum ParserPrec {
    None,		    // Lowest = do last
    Assignment,		// =
    Conditional,	// ?:
    Or,			    // or
    And,		    // and
    BinOr,		    // |
    BinXor,		    // ^
    BinAnd,		    // &
    Equality,		// == !=
    Comparison,		// < > <= >=
    Shift,		    // << >>
    Term,		    // + -
    Factor,		    // * / %
    Unary,		    // ! - ~
    Subscript,		// []
    Call,		    // . ()
    Primary,		// Highest = do first
}


impl ParserPrec {
    pub fn next(&self) -> ParserPrec {
        match self {
            ParserPrec::None		=> ParserPrec::Assignment,
            ParserPrec::Assignment	=> ParserPrec::Conditional,
            ParserPrec::Conditional	=> ParserPrec::Or,
            ParserPrec::Or		    => ParserPrec::And,
            ParserPrec::And		    => ParserPrec::BinOr,
            ParserPrec::BinOr		=> ParserPrec::BinAnd,
            ParserPrec::BinAnd		=> ParserPrec::BinXor,
            ParserPrec::BinXor		=> ParserPrec::Equality,
            ParserPrec::Equality	=> ParserPrec::Comparison,
            ParserPrec::Comparison	=> ParserPrec::Shift,
            ParserPrec::Shift		=> ParserPrec::Term,
            ParserPrec::Term		=> ParserPrec::Factor,
            ParserPrec::Factor		=> ParserPrec::Unary,
            ParserPrec::Unary		=> ParserPrec::Subscript,
            ParserPrec::Subscript	=> ParserPrec::Call,
            ParserPrec::Call		=> ParserPrec::Primary,
            ParserPrec::Primary		=> ParserPrec::Primary,
        }
    }
}
