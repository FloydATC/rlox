
#[allow(unused_imports)]
use super::token::{Token, TokenKind};
use super::scanner::Scanner;


#[allow(dead_code)]
enum ParserPrec {
    None,		// Lowest = do last
    Assignment,		// =
    Conditional,	// ?:
    Or,			// or
    And,		// and
    BinOr,		// |
    BinXor,		// ^
    BinAnd,		// &
    Equality,		// == !=
    Comparison,		// < > <= >=
    Shift,		// << >>
    Term,		// + -
    Factor,		// * / %
    Unary,		// ! - ~
    Subscript,		// []
    Call,		// . ()
    Primary,		// Highest = do first
}


type ParserFn = fn(&mut Parser, bool);


#[allow(dead_code)]
struct ParserRule {
    prefix: 	Option<ParserFn>,
    infix: 	Option<ParserFn>,
    precedence: ParserPrec,
}




#[allow(dead_code)]
pub struct Parser {
    scanner: Scanner,
}


//#[allow(dead_code)]
impl Parser {
    pub fn new(scanner: Scanner) -> Parser {
        println!("Parser::new()");
        Parser {
            scanner,
        }
    }
}


#[allow(dead_code)]
impl Parser {
    fn binary(&mut self, _can_assign: bool) {
    }
    fn rule(&self, kind: TokenKind) -> ParserRule {
        match kind {
            TokenKind::PLUS => return ParserRule {
                prefix: None,  infix: Some(Parser::binary),  precedence: ParserPrec::Term
            },
            _ => panic!("No ParserRule defined for TokenKind {:?}", kind)
        }
    }
}


impl Drop for Parser {
    fn drop(&mut self) {
        println!("Parser.drop()");
    }
}
