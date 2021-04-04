
//use core::cmp::Ordering;

#[allow(unused_imports)]
use super::token::{Token, TokenKind};
#[allow(unused_imports)]
use super::value::Value;
use super::constants::Constants;
use super::globals::Globals;
use super::local::Local;
use super::scope::Scope;
use super::codeloop::CodeLoop;
use super::tokenizer::Tokenizer;
use super::opcode::{OpCode, OpCodeSet};
use super::compiler::Compiler;

pub struct ParserInput<'a> {
    pub tokenizer: &'a mut Tokenizer,
}

pub struct ParserOutput<'a> {
    pub compiler: 	&'a mut Compiler,
    pub constants: 	&'a mut Constants<Value>,
    pub globals: 	&'a mut Globals<Value>,
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
    scopes: 	Vec<Scope>,
    locals: 	Vec<Local>,
    codeloops:	Vec<CodeLoop>,
}


#[allow(dead_code)]
impl Parser {
    pub fn new() -> Parser {
        //println!("Parser::new()");
        Parser {
            scopes: 	vec![],
            locals: 	vec![],
            codeloops:	vec![],
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
        if let Some(_) = self.scope() { return 0; }
        
        //return self.identifier_constant(input.tokenizer.previous(), output); 
        let name = input.tokenizer.previous().lexeme();
        let res = output.globals.declare(name);
        match res {
            Ok(id) => return id,
            Err(msg) => panic!("{}", msg),
        }
    }
    
    // Make a constant containing the variable name as a Value::String
    fn identifier_constant(&mut self, token: &Token, output: &mut ParserOutput) -> usize {
        //println!("Parser.identifier_constant()");
        let name = Value::string(token.lexeme());  
        return output.constants.make(name);
    }
 
    fn declare_variable(&mut self, input: &mut ParserInput, _output: &mut ParserOutput) {
        //println!("Parser.declare_variable()");
        
        let scope = self.scope();
        match scope {
            None => { return; } // Global
            Some(_) => {
        
                let name = input.tokenizer.previous().lexeme();
                // Verify variable is not already declared in this scope
                if let Some(id) = self.resolve_local(name) {
                    if self.locals[id as usize].depth() as usize == self.scopes.len() {
                        // TODO: Proper error handling
                        panic!("Variable with this name already declared");
                    }
                }
                self.declare_local(name); // Add local variable
            }
        }
    }
    
    fn define_variable(&mut self, id: usize, output: &mut ParserOutput) {
        //println!("Parser.define_variable()");
        
        if let Some(_) = self.scope() {
        //    self.mark_initialized();	// Pseudocode
            return;
        }
        
        self.define_global(id, output);
        
    }
    
    fn define_global(&mut self, id: usize, output: &mut ParserOutput) {
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

    pub fn resolve_local(&self, name: &str) -> Option<u32> {
        for i in (0..self.locals.len()).rev() {
            if self.locals[i].name() == name { return Some(i as u32); }
        }
        return None;
    }
    
    pub fn declare_local(&mut self, name: &str) {
        let depth = self.scopes.len() as u32;
        //println!("Parser.declare_local() name={} depth={}", name, depth);
        self.locals.push(Local::new(name, depth));
    }

    fn resolve_upvalue(&mut self, _name: &str) -> Option<u32> {
        eprintln!("WARNING: resolve_upvalue() not yet implemented.");
        return None;
    }
    
    fn resolve_global(&mut self, name: &str, output: &mut ParserOutput) -> Option<u32> {
        let result = output.globals.id_by_name(name);
        match result {
            Some(id)	=> Some(id as u32),
            None	=> None,
        }
    }
    
    fn variable_opcodes(&mut self, name_token: &Token, output: &mut ParserOutput) -> (OpCodeSet, OpCodeSet, u32) {
        let mut result;
        
        result = self.resolve_local(name_token.lexeme());
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
        
        result = self.resolve_upvalue(name_token.lexeme());
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
        
        //let id = self.identifier_constant(name_token, output);
        result = self.resolve_global(name_token.lexeme(), output);
        match result {
            Some(id) => {
                return (
                    OpCodeSet::getglobal(),
                    OpCodeSet::setglobal(),
                    id
                );
            }
            None => {
                // TODO: Proper error handling
                panic!("Undeclared variable");
            }
        }
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
    
    fn begin_scope(&mut self) {
        let depth = self.scopes.len() as u32;
        self.scopes.push(Scope::new(depth));
    }

    fn end_scope(&mut self, output: &mut ParserOutput) {
        self.scopes.pop();
        let depth = self.scopes.len() as u32;
        //println!("Parser.end_scope() depth={}", depth);
        loop {
            if self.locals.len() == 0 { break; }
            if self.locals.last().unwrap().depth() <= depth { break; }
            //println!("Parser.end_scope() destroy local variable '{}'", self.locals.last().unwrap().name());

            // Pseudocode for upvalues, TBD
            //if is_captured(i) {
                //emit_op(&OpCode::CloseUpvalue); 
            //} else {
            output.compiler.emit_op(&OpCode::Pop);
            //}
            self.locals.pop();
        }
    }
    
    fn scope(&mut self) -> Option<&mut Scope> {
        return self.scopes.last_mut(); // None = Global scope
    }
    
}


// ======== Statements ========
#[allow(dead_code)]
impl Parser {
    fn statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.statement()");
        if input.tokenizer.advance_on(TokenKind::Break) {
            self.break_statement(input, output);
        } else if input.tokenizer.advance_on(TokenKind::Continue) {
            self.continue_statement(input, output);
        } else if input.tokenizer.advance_on(TokenKind::Exit) {
            self.exit_statement(input, output);
        } else if input.tokenizer.advance_on(TokenKind::If) {
            self.if_statement(input, output);
        } else if input.tokenizer.advance_on(TokenKind::LeftCurly) {
            self.begin_scope();
            self.block(input, output);
            self.end_scope(output);
        } else if input.tokenizer.advance_on(TokenKind::Print) {
            self.print_statement(input, output);
        } else if input.tokenizer.advance_on(TokenKind::While) {
            self.while_statement(input, output);
        } else {
            self.expression_statement(input, output);
        }
    }

    fn block(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        loop {
            if input.tokenizer.eof() { break; }
            if input.tokenizer.matches(TokenKind::RightCurly) { break; }
            self.declaration(input, output);
        }
        self.consume(TokenKind::RightCurly, "Expect '}' after block", input, output);
    }
    
    fn break_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        self.break_loop(output);
        self.consume(TokenKind::Semicolon, "Expect ';' after 'break'", input, output);
    }

    fn continue_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        self.continue_loop(output);
        self.consume(TokenKind::Semicolon, "Expect ';' after 'continue'", input, output);
    }

    fn expression_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.expression_statement()");
        self.expression(input, output);
        self.consume(TokenKind::Semicolon, "Expect ';' after expression", input, output);
        output.compiler.emit_op(&OpCode::Pop); // Discard result
    }

    fn exit_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        if input.tokenizer.advance_on(TokenKind::Semicolon) {
            // No expression after 'exit'
            output.compiler.emit_op(&OpCode::Null);
        } else {
            self.expression(input, output);
            self.consume(TokenKind::Semicolon, "Expect ';' after expression", input, output);
        }
        output.compiler.emit_op(&OpCode::Exit);
    }

    fn if_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        // if..
        self.consume(TokenKind::LeftParen, "Expect '(' after 'if'", input, output);
        self.expression(input, output);
        self.consume(TokenKind::RightParen, "Expect ')' after condition", input, output);
        
        let else_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseP);
        // ..then
        self.statement(input, output);
        let end_jmp = output.compiler.emit_jmp(&OpCode::Jmp);
        output.compiler.patch_jmp(else_jmp);
        if input.tokenizer.advance_on(TokenKind::Else) {
            // ..else
            self.statement(input, output);
        }
        output.compiler.patch_jmp(end_jmp);
    }
    
    fn print_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        self.expression(input, output);
        self.consume(TokenKind::Semicolon, "Expect ';' after expression", input, output);
        output.compiler.emit_op(&OpCode::Print); // Print result
    }
    
    fn while_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        self.begin_loop(output);
        
        // while..
        self.consume(TokenKind::LeftParen, "Expect '(' after 'if'", input, output);
        self.expression(input, output);
        self.consume(TokenKind::RightParen, "Expect ')' after condition", input, output);
        
        let end_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseP);
        // ..do
        self.statement(input, output);

        self.end_loop(output);
        output.compiler.patch_jmp(end_jmp);
    }
}


// ======== Loop break/continue handling ========
#[allow(dead_code)]
impl Parser {
    
    fn begin_loop(&mut self, output: &mut ParserOutput) -> u32 {
        let continue_addr = output.compiler.current_ip();
        let scope_depth = self.scopes.len();
        self.codeloops.push(CodeLoop::new(continue_addr, scope_depth));
        return 0;
    }
    
    fn inner_loop(&mut self) -> Option<&mut CodeLoop> {
        return self.codeloops.last_mut();
    }
    
    fn continue_loop(&mut self, output: &mut ParserOutput) {
        let scope_depth = self.scopes.len();
        match self.inner_loop() {
            Some(codeloop) => {

                // TODO: This problem needs to be solved
                if codeloop.scope_depth() != scope_depth {
                    panic!("Can not yet handle 'continue' from inner scopes.");
                }
                
                output.compiler.emit_op(&OpCode::Jmp);
                output.compiler.emit_dword(codeloop.continue_addr());
            }
            None => {
                // TODO: Proper error handling
                panic!("'continue' not allowed here");
            }
        }
    }
    
    fn break_loop(&mut self, output: &mut ParserOutput) {
        let scope_depth = self.scopes.len();
        match self.inner_loop() {
            Some(codeloop) => {
                
                // TODO: This problem needs to be solved
                if codeloop.scope_depth() != scope_depth {
                    panic!("Can not yet handle 'break' from inner scopes.");
                }
                
                codeloop.add_break(output.compiler.emit_jmp(&OpCode::Jmp));
            }
            None => {
                // TODO: Proper error handling
                panic!("'break' not allowed here");
            }
        }
    }
    
    fn end_loop(&mut self, output: &mut ParserOutput) -> u32 {
        match self.codeloops.pop() {
            Some(codeloop) => {
                for address in codeloop.breaks() {
                    output.compiler.patch_jmp(*address);
                }
            }
            None => {
                panic!("Internal Error; end_loop() without a corresponding begin_loop()");
            }
        }
        return output.compiler.current_ip();
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
        self.parse_precedence(ParserPrec::Assignment, input, output);    
    }

    fn and(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let end_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseQ);
        output.compiler.emit_op(&OpCode::Pop);
        self.parse_precedence(ParserPrec::And, input, output);
        output.compiler.patch_jmp(end_jmp);
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
            // Single symbol
            TokenKind::Greater		=> output.compiler.emit_op(&OpCode::Greater),
            TokenKind::Less		=> output.compiler.emit_op(&OpCode::Less),
            TokenKind::Minus		=> output.compiler.emit_op(&OpCode::Sub),
            TokenKind::Percent		=> output.compiler.emit_op(&OpCode::Mod),
            TokenKind::Plus		=> output.compiler.emit_op(&OpCode::Add),
            TokenKind::Star		=> output.compiler.emit_op(&OpCode::Mul),
            TokenKind::Slash		=> output.compiler.emit_op(&OpCode::Div),
            
            // Double symbol
            TokenKind::BangEqual	=> output.compiler.emit_op(&OpCode::NotEqual),
            TokenKind::EqualEqual	=> output.compiler.emit_op(&OpCode::Equal),
            TokenKind::GreaterEqual	=> output.compiler.emit_op(&OpCode::GreaterEqual),
            TokenKind::LessEqual	=> output.compiler.emit_op(&OpCode::LessEqual),
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

    fn or(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let else_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseQ);
        let end_jmp = output.compiler.emit_jmp(&OpCode::Jmp);
        output.compiler.patch_jmp(else_jmp);
        output.compiler.emit_op(&OpCode::Pop);
        self.parse_precedence(ParserPrec::Or, input, output);
        output.compiler.patch_jmp(end_jmp);
    }

    fn string(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let value = Value::string(input.tokenizer.previous().lexeme());
        self.emit_constant(value, output);
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

    fn unary(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let operator = input.tokenizer.previous().kind();
        self.parse_precedence(ParserPrec::Unary, input, output);
        match operator {
            TokenKind::Bang 	=> output.compiler.emit_op(&OpCode::Not),
            TokenKind::Minus 	=> output.compiler.emit_op(&OpCode::Negate),
            _ => {
                panic!("Unhandled unary operator {:?}", operator);
            }
        }
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
            TokenKind::Continue => return ParserRule::null(),
            TokenKind::Else => return ParserRule::null(),
            TokenKind::Exit => return ParserRule::null(),
            TokenKind::If => return ParserRule::null(),
            TokenKind::Print => return ParserRule::null(),
            TokenKind::Return => return ParserRule::null(),
            TokenKind::Var => return ParserRule::null(),
            TokenKind::While => return ParserRule::null(),
            
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
