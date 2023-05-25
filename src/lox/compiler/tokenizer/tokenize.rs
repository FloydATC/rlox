

use crate::lox::compiler::{Token, TokenKind};


pub trait Tokenize {
    fn current(&self) -> &Token;
    fn previous(&self) -> &Token;
    fn eof(&self) -> bool;
    fn advance(&mut self);
    fn matches(&self, kind: TokenKind) -> bool;
    fn advance_on(&mut self, kind: TokenKind) -> bool;
}

