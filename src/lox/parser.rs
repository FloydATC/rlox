
//use core::cmp::Ordering;

#[allow(unused_imports)]
use super::token::{Token, TokenKind};
#[allow(unused_imports)]
use super::value::Value;
use super::constants::Constants;
use super::tokenizer::Tokenizer;
use super::opcode::{OpCode, OpCodeSet};
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
        self.emit_return(output);
        
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
    
    fn emit_return(&self, output: &mut ParserOutput) {
        //if compiler.type == TYPE_INITIALIZER {
        //    output.compiler.emit_op(&OpCode::GetLocal);
        //    output.compiler.emit_byte(0);
        //} else {
        output.compiler.emit_op(&OpCode::Null);
        //}
        output.compiler.emit_op(&OpCode::Return);
    }

    fn emit_constant(&self, value: Value, output: &mut ParserOutput) {
        let id = output.constants.make(value);
        let ops = OpCodeSet::getconst();
        output.compiler.emit_op_variant(&ops, id as u64);
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
    
    fn parse_variable(&mut self, errmsg: &str, input: &mut ParserInput, output: &mut ParserOutput) -> usize {
        //println!("Parser.parse_variable()");
        
        self.consume(TokenKind::Identifier, errmsg, input, output);
        
        self.declare_variable(input, output);
        // if scope_depth > 0 { return 0 } // Pseudocode
        
        return self.identifier_constant(input.tokenizer.previous(), output); 
    }
    
    // Make a constant containing the variable name as a Value::String
    fn identifier_constant(&mut self, token: &Token, output: &mut ParserOutput) -> usize {
        //println!("Parser.identifier_constant()");
        let name = Value::string(token.lexeme());  
        return output.constants.make(name);
    }
 
    fn declare_variable(&mut self, _input: &mut ParserInput, _output: &mut ParserOutput) {
        //println!("Parser.declare_variable()");
        
        //if scope_depth == 0 { // Pseudocode
        return; 
        //}
        
        //let var_name = input.tokenizer.previous().lexeme();
        //if locals.has(var_name) {
        //    panic!("Variable with this name already declared");
        //} else {
        //    add_local(var_name)
        //}
    }
    
    fn define_variable(&mut self, id: usize, output: &mut ParserOutput) {
        //println!("Parser.define_variable()");
        
        //if scope_depth > 0 {
        //    self.mark_initialized();	// Pseudocode
        //    return;
        //}
        let variable = id as u64;
        match variable {
            0..=0xff => {
                output.compiler.emit_op(&OpCode::DefGlobal8);
                output.compiler.emit_byte(variable as u8);
            }
            0x100..=0xffff => {
                output.compiler.emit_op(&OpCode::DefGlobal16);
                output.compiler.emit_word(variable as u16);
            }
            0x10000..=0xffffffff => {
                output.compiler.emit_op(&OpCode::DefGlobal32);
                output.compiler.emit_dword(variable as u32);
            }
            _ => {
                panic!("4.2 billion globals should be enough for everyone.");
            }
        }
    }

    fn resolve_local(&mut self, _name_token: &Token) -> Option<usize> {
        eprintln!("WARNING: resolve_local() not yet implemented.");
        return None;
    }

    fn resolve_upvalue(&mut self, _name_token: &Token) -> Option<usize> {
        eprintln!("WARNING: resolve_upvalue() not yet implemented.");
        return None;
    }
    
    fn variable_opcodes(&mut self, name_token: &Token, output: &mut ParserOutput) -> (OpCodeSet, OpCodeSet, usize) {
        let mut result;
        
        result = self.resolve_local(name_token);
        match result {
            Some(id) => {
                return (
                    OpCodeSet::getlocal(),
                    OpCodeSet::setlocal(),
                    id
                );
            }
            None => {}
        }
        
        result = self.resolve_upvalue(name_token);
        match result {
            Some(id) => {
                return (
                    OpCodeSet::getupvalue(),
                    OpCodeSet::setupvalue(),
                    id
                );
            }
            None => {}
        }
        
        let id = self.identifier_constant(name_token, output);
        return (
            OpCodeSet::getglobal(),
            OpCodeSet::setglobal(),
            id
        );
    }
    
    fn named_variable(&mut self, name_token: &Token, can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        // Get opcodes for get/set and id of local, upvalue or global
        let (get_ops, set_ops, id) = self.variable_opcodes(name_token, output);

        // Pick set or get based on context
        if can_assign && input.tokenizer.advance_on(TokenKind::Equal) {
            self.expression(input, output);
            output.compiler.emit_op_variant(&set_ops, id as u64);
        } else {
            output.compiler.emit_op_variant(&get_ops, id as u64);
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
        self.consume(TokenKind::Semicolon, "Expect ';' after expression", input, output);
        output.compiler.emit_op(&OpCode::Pop); // Discard result
    }
}

// ======== Declarations ========
#[allow(dead_code)]
impl Parser {
    fn declaration(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.declaration() begin");
        match input.tokenizer.current().kind() {
            //TokenKind::Class 	=> self.class_declaration(input, output),
            //TokenKind::Fun 	=> self.fun_declaration(input, output),
            TokenKind::Var	=> self.var_declaration(input, output),
            _			=> self.statement(input, output),
        }
        //println!("Parser.declaration() end");
    }
    fn class_declaration(&mut self, input: &mut ParserInput, _output: &mut ParserOutput) {
        input.tokenizer.advance(); // Consume Class token
    }
    fn fun_declaration(&mut self, input: &mut ParserInput, _output: &mut ParserOutput) {
        input.tokenizer.advance(); // Consume Fun token
    }
    fn var_declaration(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        input.tokenizer.advance(); // Consume Var token
        let name_id = self.parse_variable("Expect variable name", input, output);
        
        if input.tokenizer.advance_on(TokenKind::Equal) {
            self.expression(input, output);
        } else {
            output.compiler.emit_op(&OpCode::Null);
        }
        self.consume(TokenKind::Semicolon, "Expect ';' after variable declaration", input, output);
        
        self.define_variable(name_id, output);
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
            TokenKind::Plus	=> output.compiler.emit_op(&OpCode::Add),
            TokenKind::Minus	=> output.compiler.emit_op(&OpCode::Sub),
            TokenKind::Star	=> output.compiler.emit_op(&OpCode::Mul),
            TokenKind::Slash	=> output.compiler.emit_op(&OpCode::Div),
            TokenKind::Percent	=> output.compiler.emit_op(&OpCode::Mod),
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
    fn grouping(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        self.expression(input, output);
        self.consume(TokenKind::RightParen, "Expect ')' after expression", input, output);
    }
    fn literal(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let literal = input.tokenizer.previous().kind();
        match literal {
            TokenKind::False	=> output.compiler.emit_op(&OpCode::False),
            TokenKind::Null	=> output.compiler.emit_op(&OpCode::Null),
            TokenKind::True	=> output.compiler.emit_op(&OpCode::True),
            _ => {
                panic!("Unhandled literal {:?}", literal);
            }
        }
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
    fn variable(&mut self, can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let token = input.tokenizer.previous().clone();
        self.named_variable(&token, can_assign, input, output);
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
            TokenKind::Null => return ParserRule {
                prefix: 	Some(Parser::literal), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::True => return ParserRule {
                prefix: 	Some(Parser::literal), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },

            // Keywords
            TokenKind::Return => return ParserRule::null(),
            TokenKind::Var => return ParserRule::null(),
            
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
