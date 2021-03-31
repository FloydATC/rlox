
//use core::cmp::Ordering;

#[allow(unused_imports)]
use super::token::{Token, TokenKind};
#[allow(unused_imports)]
use super::value::Value;
use super::tokenizer::Tokenizer;
use super::opcode::OpCode;
use super::compiler::Compiler;

#[allow(dead_code)]
#[repr(u8)]
#[derive(PartialOrd,PartialEq)]
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


impl ParserPrec {
    fn next(&self) -> ParserPrec {
        match self {
            ParserPrec::None		=> ParserPrec::Assignment,
            ParserPrec::Assignment	=> ParserPrec::Conditional,
            ParserPrec::Conditional	=> ParserPrec::Or,
            ParserPrec::Or		=> ParserPrec::And,
            ParserPrec::And		=> ParserPrec::BinOr,
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

type ParserFn = fn(&mut Parser, bool);


#[allow(dead_code)]
struct ParserRule {
    prefix: 	Option<ParserFn>,
    infix: 	Option<ParserFn>,
    precedence: ParserPrec,
}


impl ParserRule {
    fn null() -> ParserRule {
        ParserRule {
            prefix:	None,
            infix:	None,
            precedence:	ParserPrec::None,
        }
    }
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
        
        loop {
            //println!("Parser::parse() loop begins");
            if self.tokenizer().eof() { break; }
            self.declaration();
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
    fn mut_tokenizer(&mut self) -> &mut Tokenizer {
        match &self.tokenizer {
            Some(_) => self.tokenizer.as_mut().unwrap(),
            None => panic!("Internal Error; No Tokenizer"),
        }
    }

    fn tokenizer(&self) -> &Tokenizer {
        match &self.tokenizer {
            Some(_) => self.tokenizer.as_ref().unwrap(),
            None => panic!("Internal Error; No Tokenizer"),
        }
    }

    // Shorthand
    fn consume(&mut self, kind: TokenKind, errmsg: &str) {
        if !self.mut_tokenizer().advance_on(kind) {
            // TODO: Proper error handling
            panic!("{}, got\n{:#?}", errmsg, self.tokenizer().current());
        }
    }


    fn compiler(&mut self) -> &mut Compiler {
        match &self.compiler {
            Some(_) => self.compiler.as_mut().unwrap(),
            None => panic!("Internal Error; No Compiler"),
        }
    }
    
    // Shorthand
    fn tokenkind(&mut self) -> TokenKind {
        return self.tokenizer().current().kind();
    }
    
    // Shorthand
    fn emit_op(&mut self, opcode: OpCode) {
        self.compiler().emit_op(opcode);
    }
    
    // Shorthand
    fn emit_byte(&mut self, byte: u8) {
        self.compiler().emit_byte(byte);
    }
    
    // Shorthand
    fn emit_word(&mut self, word: u16) {
        self.compiler().emit_word(word);
    }
    
    // Shorthand
    fn emit_dword(&mut self, dword: u32) {
        self.compiler().emit_dword(dword);
    }
    
    fn make_constant(&mut self, _value: Value) -> u32 {
        return 123;
    }
    
    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        match constant {
            0..=255 => {
                self.emit_op(OpCode::Const8);
                self.emit_byte(constant as u8);
            }
            256..=65535 => {
                self.emit_op(OpCode::Const16);
                self.emit_word(constant as u16);
            }
            _ => {
                self.emit_op(OpCode::Const32);
                self.emit_dword(constant);
            }
        }
    }
    
    fn current_token_rule(&self) -> ParserRule {
        let kind = self.tokenizer().current().kind();
        return self.rule(&kind);
    }
    
    fn previous_token_rule(&self) -> ParserRule {
        let kind = self.tokenizer().previous().kind();
        return self.rule(&kind);
    }
    
    // This is the core of the expression parser
    // The code is my amateurish attempt to re-implement in Rust
    // compiler.c:parsePrecedence() from Robert Nystrom's excellent book
    // http://craftinginterpreters.com
    // Please accept my apologies.
    fn parse_precedence(&mut self, precedence: ParserPrec) {
        //println!("Parser.parse_precedence()");
    
        self.mut_tokenizer().advance();
        let rule = self.previous_token_rule();
        
        match rule.prefix {
            Some(method) => {
                let can_assign = precedence <= ParserPrec::Assignment;
                method(self, can_assign); // Call the Compiler method pointer
                
                loop {
                    let rule = self.current_token_rule();
                    if precedence > rule.precedence { break; }
                    
                    self.mut_tokenizer().advance();

                    match rule.infix {
                        Some(method) => {
                            method(self, can_assign); // Call the Compiler method pointer
                        }
                        None => {
                            // TODO: Proper error handling
                            // Not sure if this is even reachable
                            panic!("Expect expression.");
                        }
                    }
                }
                
                if can_assign && self.tokenizer().matches(TokenKind::Equal) {
                    // TODO: Proper error handling
                    panic!("Invalid assignment target.");
                }
                
            }
            None => {
                // TODO: Proper error handling
                panic!("Expect expression.");
            }
        }
    }
}


// ======== Statements ========
#[allow(dead_code)]
impl Parser {
    fn statement(&mut self) {
        //println!("Parser.statement()");
        match self.tokenkind() {
            //TokenKind::Print	=> self.print_statement(),
            _			=> self.expression_statement(),
        }
    }
    fn expression_statement(&mut self) {
        //println!("Parser.expression_statement()");
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after expression.");
        self.emit_op(OpCode::Pop); // Discard result
    }
}

// ======== Declarations ========
#[allow(dead_code)]
impl Parser {
    fn declaration(&mut self) {
        //println!("Parser.declaration()");
        match self.tokenkind() {
            //TokenKind::Class 	=> self.class_declaration(),
            //TokenKind::Fun 	=> self.fun_declaration(),
            //TokenKind::Var	=> self.var_declaration(),
            _			=> self.statement(),
        }
    }
    fn class_declaration(&mut self) {
    }
    fn fun_declaration(&mut self) {
    }
    fn var_declaration(&mut self) {
    } 
}


// ======== Expressions ========
#[allow(dead_code)]
impl Parser {
    fn expression(&mut self) {
        //println!("Parser.expression()");
        self.parse_precedence(ParserPrec::Assignment);    
    }
    fn and_(&mut self, _can_assign: bool) {
    }
    fn array(&mut self, _can_assign: bool) {
    }
    fn base2number(&mut self, _can_assign: bool) {
        let lexeme = self.tokenizer().previous().lexeme();
        let without_prefix = lexeme.trim_start_matches("0b");
        let float = i64::from_str_radix(without_prefix, 2).unwrap() as f64;
        self.emit_constant(Value::number(float));
    }
    fn base8number(&mut self, _can_assign: bool) {
        let lexeme = self.tokenizer().previous().lexeme();
        let float = i64::from_str_radix(lexeme, 8).unwrap() as f64;
        self.emit_constant(Value::number(float));
    }
    fn base10number(&mut self, _can_assign: bool) {
        let lexeme = self.tokenizer().previous().lexeme();
        let float: f64 = lexeme.parse().unwrap();
        self.emit_constant(Value::number(float));
    }
    fn base16number(&mut self, _can_assign: bool) {
        let lexeme = self.tokenizer().previous().lexeme();
        let without_prefix = lexeme.trim_start_matches("0x");
        let float = i64::from_str_radix(without_prefix, 16).unwrap() as f64;
        self.emit_constant(Value::number(float));
    }
    fn binary(&mut self, _can_assign: bool) {
        //println!("Parser.binary()");

        let operator = self.tokenizer().previous().kind();
        let rule = self.rule(&operator);

        self.parse_precedence(rule.precedence.next());
        
        match operator {
            TokenKind::Plus	=> self.emit_op(OpCode::Add),
            _ => {
                panic!("Unhandled binary operator {:?}", operator);
            }
        }
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
    fn rule(&self, kind: &TokenKind) -> ParserRule {
        //println!("Parser.rule() kind={:?}", kind);
        match kind {

            // Single character symbols
            TokenKind::Bang => return ParserRule {
                prefix: 	Some(Parser::unary), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Equal => return ParserRule::null(),
            TokenKind::Minus => return ParserRule {
                prefix: 	Some(Parser::unary), 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Term,
            },
            TokenKind::Plus => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Term,
            },
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
            TokenKind::Identifier => return ParserRule {
                prefix: 	Some(Parser::variable), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },

            // Keywords
            TokenKind::Return => return ParserRule::null(),
            
            // Internal
            TokenKind::Error => return ParserRule::null(),
            TokenKind::EOF => return ParserRule::null(),
        }
    }
}


impl Drop for Parser {
    fn drop(&mut self) {
        println!("Parser.drop()");
    }
}
