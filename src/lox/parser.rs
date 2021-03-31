
//use core::cmp::Ordering;

#[allow(unused_imports)]
use super::token::{Token, TokenKind};
#[allow(unused_imports)]
use super::value::Value;
use super::constants::Constants;
use super::tokenizer::Tokenizer;
use super::opcode::OpCode;
use super::compiler::Compiler;

pub struct ParserInput<'a> {
    pub tokenizer: &'a mut Tokenizer,
}

pub struct ParserOutput<'a> {
    pub compiler: &'a mut Compiler,
    pub constants: &'a mut Constants,
}

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

type ParserFn = fn(&mut Parser, bool, &mut ParserInput, &mut ParserOutput);


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
}


#[allow(dead_code)]
impl Parser {
    pub fn new() -> Parser {
        //println!("Parser::new()");
        Parser {
        }
    }
    
    pub fn parse(&mut self, input: &mut ParserInput, output: &mut ParserOutput) -> Result<(), String> {
        
        loop {
            //println!("Parser::parse() loop begins");
            if input.tokenizer.eof() { break; }
            self.declaration(input, output);
        }
        
        return Ok(());
    }
}


#[allow(dead_code)]
impl Parser {

    // Shorthand
    fn consume(&self, kind: TokenKind, errmsg: &str, input: &mut ParserInput, _output: &mut ParserOutput) {
        if !input.tokenizer.advance_on(kind) {
            // TODO: Proper error handling
            panic!("{}, got\n{:#?}", errmsg, input.tokenizer.current());
        }
    }

    // Shorthand
    fn emit_op(&self, opcode: OpCode, output: &mut ParserOutput) {
        output.compiler.emit_op(opcode);
    }
    
    // Shorthand
    fn emit_byte(&self, byte: u8, output: &mut ParserOutput) {
        output.compiler.emit_byte(byte);
    }
    
    // Shorthand
    fn emit_word(&self, word: u16, output: &mut ParserOutput) {
        output.compiler.emit_word(word);
    }
    
    // Shorthand
    fn emit_dword(&self, dword: u32, output: &mut ParserOutput) {
        output.compiler.emit_dword(dword);
    }
    
    fn emit_constant(&self, value: Value, output: &mut ParserOutput) {
//        let constant = self.make_constant(value) as u64;
        let constant = output.constants.make(value) as u64;
        match constant {
            0..=0xff => {
                output.compiler.emit_op(OpCode::Const8);
                output.compiler.emit_byte(constant as u8);
            }
            0x100..=0xffff => {
                output.compiler.emit_op(OpCode::Const16);
                output.compiler.emit_word(constant as u16);
            }
            0x10000..=0xffffffff => {
                output.compiler.emit_op(OpCode::Const32);
                output.compiler.emit_dword(constant as u32);
            }
            _ => {
                panic!("4.2 billion constants should be enough for everyone.");
            }
        }
    }
    
    fn current_token_rule(&self, input: &mut ParserInput) -> ParserRule {
        let kind = input.tokenizer.current().kind();
        return self.rule(&kind);
    }
    
    fn previous_token_rule(&self, input: &mut ParserInput) -> ParserRule {
        let kind = input.tokenizer.previous().kind();
        return self.rule(&kind);
    }
    
    // This is the core of the expression parser
    // The code is my amateurish attempt to re-implement in Rust
    // compiler.c:parsePrecedence() from Robert Nystrom's excellent book
    // http://craftinginterpreters.com
    // Please accept my apologies.
    fn parse_precedence(&mut self, precedence: ParserPrec, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.parse_precedence()");
    
        input.tokenizer.advance();
        let rule = self.previous_token_rule(input);
        
        match rule.prefix {
            Some(method) => {
                let can_assign = precedence <= ParserPrec::Assignment;
                method(self, can_assign, input, output); // Call the Compiler method pointer
                
                loop {
                    let rule = self.current_token_rule(input);
                    if precedence > rule.precedence { break; }
                    
                    input.tokenizer.advance();

                    match rule.infix {
                        Some(method) => {
                            method(self, can_assign, input, output); // Call the Compiler method pointer
                        }
                        None => {
                            // TODO: Proper error handling
                            // Not sure if this is even reachable; clox does not test for this
                            panic!("Expect expression.");
                        }
                    }
                }
                
                if can_assign && input.tokenizer.matches(TokenKind::Equal) {
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
    fn statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.statement()");
        match input.tokenizer.current().kind() {
            //TokenKind::Print	=> self.print_statement(),
            _			=> self.expression_statement(input, output),
        }
    }
    fn expression_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.expression_statement()");
        self.expression(input, output);
        self.consume(TokenKind::Semicolon, "Expect ';' after expression.", input, output);
        output.compiler.emit_op(OpCode::Pop); // Discard result
    }
}

// ======== Declarations ========
#[allow(dead_code)]
impl Parser {
    fn declaration(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.declaration()");
        match input.tokenizer.current().kind() {
            //TokenKind::Class 	=> self.class_declaration(),
            //TokenKind::Fun 	=> self.fun_declaration(),
            //TokenKind::Var	=> self.var_declaration(),
            _			=> self.statement(input, output),
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
    fn expression(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.expression()");
        self.parse_precedence(ParserPrec::Assignment, input, output);    
    }
    fn and_(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn array(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn base2number(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let lexeme = input.tokenizer.previous().lexeme();
        let without_prefix = lexeme.trim_start_matches("0b");
        let float = i64::from_str_radix(without_prefix, 2).unwrap() as f64;
        self.emit_constant(Value::number(float), output);
    }
    fn base8number(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let lexeme = input.tokenizer.previous().lexeme();
        let float = i64::from_str_radix(lexeme, 8).unwrap() as f64;
        self.emit_constant(Value::number(float), output);
    }
    fn base10number(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let lexeme = input.tokenizer.previous().lexeme();
        let float: f64 = lexeme.parse().unwrap();
        self.emit_constant(Value::number(float), output);
    }
    fn base16number(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let lexeme = input.tokenizer.previous().lexeme();
        let without_prefix = lexeme.trim_start_matches("0x");
        let float = i64::from_str_radix(without_prefix, 16).unwrap() as f64;
        self.emit_constant(Value::number(float), output);
    }
    fn binary(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.binary()");

        let operator = input.tokenizer.previous().kind();
        let rule = self.rule(&operator);

        self.parse_precedence(rule.precedence.next(), input, output);
        
        match operator {
            TokenKind::Plus	=> output.compiler.emit_op(OpCode::Add),
            TokenKind::Minus	=> output.compiler.emit_op(OpCode::Sub),
            TokenKind::Star	=> output.compiler.emit_op(OpCode::Mul),
            TokenKind::Slash	=> output.compiler.emit_op(OpCode::Div),
            TokenKind::Percent	=> output.compiler.emit_op(OpCode::Mod),
            _ => {
                panic!("Unhandled binary operator {:?}", operator);
            }
        }
    }
    fn call(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn dot(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn grouping(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn literal(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn or_(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn string(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn subscr(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn super_(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn ternary(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn this_(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn unary(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
    }
    fn variable(&mut self, _can_assign: bool, _input: &mut ParserInput, _output: &mut ParserOutput) {
        panic!("Not yet implemented.");
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
            TokenKind::Percent => return ParserRule {
                prefix: 	None, 
                infix: 		Some(Parser::binary), 
                precedence: 	ParserPrec::Factor,
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
        //println!("Parser.drop()");
    }
}
