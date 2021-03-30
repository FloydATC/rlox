
#[allow(unused_imports)]
use super::token::{Token, TokenKind};
#[allow(unused_imports)]
use super::tokenizer::Tokenizer;
use super::opcode::OpCode;
use super::compiler::Compiler;

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
    tokenizer: Option<Tokenizer>,
    compiler: Option<Compiler>,
}


#[allow(dead_code)]
impl Parser {
    pub fn new(tokenizer: Tokenizer, compiler: Compiler) -> Parser {
        println!("Parser::new()");
        Parser {
            tokenizer: Some(tokenizer),
            compiler: Some(compiler),
        }
    }
    
    
    pub fn parse(&mut self) -> Result<(), String> {
        println!("Parser::parse()");
        
        loop {

            // Placeholder for actual parser, just to verify tokenizer
            match self.tokenizer().current().kind() {
                TokenKind::EOF => {
                    break;
                }
                TokenKind::Return => {
                    self.compiler().emit(OpCode::Return);
                }
                TokenKind::Plus => {
                    self.compiler().emit(OpCode::Add);
                }
                TokenKind::Base10Number => {
                    self.compiler().emit(OpCode::Push);
                    let byte: u8 = self.tokenizer().current().lexeme().parse().unwrap();
                    self.compiler().emit_byte(byte);
                }
                _ => {}
            }
            self.tokenizer().advance();

        }
        
        return Ok(());
    }
    
    
    pub fn take_tokenizer(&mut self) -> Tokenizer {
        let tokenizer = self.tokenizer.take().unwrap();
        return tokenizer;
    }


    pub fn take_compiler(&mut self) -> Compiler {
        let compiler = self.compiler.take().unwrap();
        return compiler;
    }
}


#[allow(dead_code)]
impl Parser {
    fn tokenizer(&mut self) -> &mut Tokenizer {
        match &self.tokenizer {
            Some(_) => self.tokenizer.as_mut().unwrap(),
            None => panic!("Internal Error; No Tokenizer"),
        }
    }


    fn compiler(&mut self) -> &mut Compiler {
        match &self.compiler {
            Some(_) => self.compiler.as_mut().unwrap(),
            None => panic!("Internal Error; No Compiler"),
        }
    }
}


#[allow(dead_code)]
impl Parser {
    fn and_(&mut self, _can_assign: bool) {
    }
    fn array(&mut self, _can_assign: bool) {
    }
    fn b10number(&mut self, _can_assign: bool) {
    }
    fn binary(&mut self, _can_assign: bool) {
    }
    fn call(&mut self, _can_assign: bool) {
    }
    fn dot(&mut self, _can_assign: bool) {
    }
    fn grouping(&mut self, _can_assign: bool) {
    }
    fn literal(&mut self, _can_assign: bool) {
    }
    fn or_(&mut self, _can_assign: bool) {
    }
    fn string(&mut self, _can_assign: bool) {
    }
    fn subscr(&mut self, _can_assign: bool) {
    }
    fn super_(&mut self, _can_assign: bool) {
    }
    fn ternary(&mut self, _can_assign: bool) {
    }
    fn this_(&mut self, _can_assign: bool) {
    }
    fn unary(&mut self, _can_assign: bool) {
    }
    fn variable(&mut self, _can_assign: bool) {
    }
}


#[allow(dead_code)]
impl Parser {
    fn rule(&self, kind: TokenKind) -> ParserRule {
        match kind {
            TokenKind::Minus => return ParserRule {
                prefix: 	Some(Parser::unary), 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Term
            },
            TokenKind::Plus => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Term
            },
            TokenKind::Slash => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Factor
            },
            TokenKind::Star => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Factor
            },
            // Default handler
            _ => {
                eprintln!("Warning: No ParserRule for TokenKind {:?}", kind);
                return ParserRule {
                    prefix: 	None, 
                    infix: 		None, 
                    precedence: 	ParserPrec::None
                }
            },
        }
    }
}


impl Drop for Parser {
    fn drop(&mut self) {
        println!("Parser.drop()");
    }
}
