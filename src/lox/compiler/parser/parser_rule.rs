

use crate::lox::compiler::{CompileError, TokenKind, Tokenize, ParserOutput};


use super::{Parser, ParserPrec};


type ParserFn<I> = fn(&mut Parser<I>, bool, &mut I, &mut ParserOutput) -> Result<(), CompileError>;


#[allow(dead_code)]
pub struct ParserRule<I: Tokenize> {
    pub prefix:	Option<ParserFn<I>>,
    pub infix: Option<ParserFn<I>>,
    pub precedence: ParserPrec,
}


impl<I: Tokenize> ParserRule<I> {

    pub fn null() -> ParserRule<I> {
        ParserRule {
            prefix:	None,
            infix:	None,
            precedence:	ParserPrec::None,
        }
    }

    // ParserRule dispatcher

    pub fn for_token(kind: &TokenKind) -> ParserRule<I> {
        //println!("Parser.rule() kind={:?}", kind);
        match kind {

            // Single character symbols
            TokenKind::Amp => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::BinAnd,
            },
            TokenKind::Bang => return ParserRule {
                prefix: 	Some(Parser::unary), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Comma => return ParserRule::null(),
            TokenKind::Dot => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::dot), 
                precedence: 	ParserPrec::Call,
            },
            TokenKind::Equal => return ParserRule::null(),
            TokenKind::Greater => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Comparison,
            },
            TokenKind::LeftBracket => return ParserRule {
                prefix:		Some(Parser::array), 
                infix: 		Some(Parser::subscr), 
                precedence: 	ParserPrec::Subscript,
            },
            TokenKind::LeftCurly => return ParserRule::null(),
            TokenKind::LeftParen => return ParserRule {
                prefix: 	Some(Parser::grouping), 
                infix: 		Some(Parser::call), 
                precedence: 	ParserPrec::Call,
            },            
            TokenKind::Less => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Comparison,
            },
            TokenKind::Minus => return ParserRule {
                prefix: 	Some(Parser::unary), 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Term,
            },
            TokenKind::Percent => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Factor,
            },
            TokenKind::Pipe => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::BinOr,
            },
            TokenKind::Plus => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Term,
            },
            TokenKind::RightBracket => return ParserRule::null(),
            TokenKind::RightCurly => return ParserRule::null(),
            TokenKind::RightParen => return ParserRule::null(),
            TokenKind::Slash => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Factor,
            },
            TokenKind::Star => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Factor,
            },
            TokenKind::Semicolon => return ParserRule::null(),
            
            // Double character symbols
            TokenKind::AmpAmp => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::and), 
                precedence: 	ParserPrec::And,
            },
            TokenKind::BangEqual => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Equality,
            },
            TokenKind::EqualEqual => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Equality,
            },
            TokenKind::GreaterEqual => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Comparison,
            },
            TokenKind::LessEqual => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Comparison,
            },
            TokenKind::PipePipe => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::or), 
                precedence: 	ParserPrec::Or,
            },

            // Literals
            TokenKind::Base2Number => return ParserRule {
                prefix: 	Some(Parser::base2number), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Base8Number => return ParserRule {
                prefix: 	Some(Parser::base8number), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Base10Number => return ParserRule {
                prefix: 	Some(Parser::base10number), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Base16Number => return ParserRule {
                prefix: 	Some(Parser::base16number), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::False => return ParserRule {
                prefix: 	Some(Parser::literal), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Identifier => return ParserRule {
                prefix: 	Some(Parser::variable), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Inf => return ParserRule {
                prefix: 	Some(Parser::literal), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Nan => return ParserRule {
                prefix: 	Some(Parser::literal), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Null => return ParserRule {
                prefix: 	Some(Parser::literal), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::String => return ParserRule {
                prefix: 	Some(Parser::string), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::True => return ParserRule {
                prefix: 	Some(Parser::literal), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },

            // Keywords
            TokenKind::Break => return ParserRule::null(),
            TokenKind::Class => return ParserRule::null(),
            TokenKind::Continue => return ParserRule::null(),
            TokenKind::Debug => return ParserRule::null(),
            TokenKind::Else => return ParserRule::null(),
            TokenKind::Exit => return ParserRule::null(),
            TokenKind::If => return ParserRule::null(),
            TokenKind::In => return ParserRule::null(),
            TokenKind::Is => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Equality,
            },
            TokenKind::Not => return ParserRule::null(),
            TokenKind::Of => return ParserRule::null(),
            TokenKind::Print => return ParserRule::null(),
            TokenKind::Return => return ParserRule::null(),
            TokenKind::Super => return ParserRule {
                prefix: 	Some(Parser::super_), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            }, 
            TokenKind::This => return ParserRule {
                prefix: 	Some(Parser::this_), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Var => return ParserRule::null(),
            TokenKind::For => return ParserRule::null(),
            TokenKind::Fun => return ParserRule::null(),
            TokenKind::While => return ParserRule::null(),
            
            // Internal
            TokenKind::Error => return ParserRule::null(),
            TokenKind::EOF => return ParserRule::null(),
        }
    }

}


