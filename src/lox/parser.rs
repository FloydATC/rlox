
use super::token::{Token, TokenKind};
use super::function::{Function, FunctionKind, INITIALIZER};
use super::value::Value;
use super::globals::Globals;
use super::hierarchy::Hierarchy;
use super::locals::Locals;
//use super::local::Local;
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
    pub globals: 	&'a mut Globals<Value>,
    pub locals: 	&'a mut Locals,
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
    classes:    Hierarchy<Token>,
    codeloops:	Vec<CodeLoop>,
}


#[allow(dead_code)]
impl Parser {
    pub fn new() -> Parser {
        //println!("Parser::new()");
        Parser {
            scopes: 	vec![],
            classes:    Hierarchy::new(),
            codeloops:	vec![],
        }
    }

    // Parse __main__ function only (See: parse_function())    
    pub fn parse(&mut self, input: &mut ParserInput, output: &mut ParserOutput) -> Result<(), String> {
        
        loop {
            //println!("Parser::parse() loop begins");
            if input.tokenizer.eof() { break; }
            self.declaration(input, output);
        }
        self.emit_exit(output);
        
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
        if output.compiler.function().kind().return_self() {
            output.compiler.emit_op(&OpCode::GetLocal8);
            output.compiler.emit_byte(0);
        } else {
            output.compiler.emit_op(&OpCode::Null);
        }
        //}
        output.compiler.emit_op(&OpCode::Return);
    }

    fn emit_exit(&self, output: &mut ParserOutput) {
        output.compiler.emit_op(&OpCode::Null);
        output.compiler.emit_op(&OpCode::Exit);
    }

    fn emit_constant(&self, value: Value, output: &mut ParserOutput) {
        let id = output.compiler.make_constant(value);
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

    // Note: called instead of parse() to handle functions/methods
    fn parse_function(&mut self, input: &mut ParserInput, output: &mut ParserOutput) -> Result<(), String> {
        self.begin_scope();

        // Parameter list        
        self.consume(TokenKind::LeftParen, "Expect '(' after function name.", input, output);
        let result = self.parse_function_params(input, output);
        match result {
            Ok(arity) => {
                output.compiler.function().set_arity(arity);
            }
            Err(msg) => {        
                // TODO: Proper error handling
                panic!("{}", msg);
            }
        }
        self.consume(TokenKind::RightParen, "Expect ')' after parameters.", input, output);
        
        // Body
        self.consume(TokenKind::LeftCurly, "Expect '{' before function body.", input, output);
        self.block(input, output); // Handles the closing curly
        
        self.emit_return(output);

        self.end_scope(output);
        
        Ok(())
    }
    
    fn parse_function_params(&mut self, input: &mut ParserInput, output: &mut ParserOutput) -> Result<u8, String> {
        let mut arity = 0;
        if !input.tokenizer.matches(TokenKind::RightParen) {
            loop {
                if arity == 255 { 
                    // TODO: Proper error handling
                    panic!("Can't have more than 255 parameters.");
                }
                arity = arity + 1;
                let name_id = self.parse_variable("Expect parameter name", input, output);
                self.define_variable(name_id, output);
                // Keep going?
                if !input.tokenizer.advance_on(TokenKind::Comma) { break; }
            }
        }
        return Ok(arity);
    }

    fn parse_variable(&mut self, errmsg: &str, input: &mut ParserInput, output: &mut ParserOutput) -> usize {
        //println!("Parser.parse_variable()");
        
        self.consume(TokenKind::Identifier, errmsg, input, output);
        
        self.declare_variable(input, output);
        if let Some(_) = self.scope() { return 0; }
        
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
        return output.compiler.make_constant(name);
    }
 
    fn declare_variable(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.declare_variable()");
        
        let scope = self.scope();
        match scope {
            None => { return; } // Global
            Some(_) => {
                let scope_depth = self.scopes.len();
                let name = input.tokenizer.previous().lexeme();

                // Verify variable is not already declared in this scope
                if let Some(id) = output.locals.resolve_local(name) {
                    if output.locals.local_ref_by_id(id).depth() == scope_depth {
                        // TODO: Proper error handling
                        panic!("Variable named {:?} already declared: {:#?}", name, output.locals);
                    }
                }

                output.locals.declare_local(name, scope_depth); // Add local variable
            }
        }
    }
    
    fn define_variable(&mut self, id: usize, output: &mut ParserOutput) {
        //println!("Parser.define_variable()");
        
        if let Some(_) = self.scope() {
            // self.mark_initialized();	// TODO: This needs solving.
            output.locals.last_local().unwrap().define();
            return;
        }
        
        self.define_global(id, output);
        
    }
    
    fn define_global(&mut self, id: usize, output: &mut ParserOutput) {
        output.compiler.emit_op_variant(&OpCodeSet::defglobal(), id as u64);
    }

    fn resolve_global(&mut self, name: &str, output: &mut ParserOutput) -> Option<usize> {
        let result = output.globals.id_by_name(name);
        match result {
            Some(id)	=> Some(id),
            None	=> None,
        }
    }
    
    fn variable_opcodes(&mut self, name_token: &Token, output: &mut ParserOutput) -> (OpCodeSet, OpCodeSet, usize) {
        let mut result;
        
        result = output.locals.resolve_local(name_token.lexeme());
        match result {
            Some(id) => {
                if !output.locals.local_ref_by_id(id).is_defined() {
                    // TODO: Proper error handling
                    panic!("Can't readl local variable in its own initializer.");
                }
                return (
                    OpCodeSet::getlocal(),
                    OpCodeSet::setlocal(),
                    id
                );
            }
            None => {}
        }
        
        result = output.locals.resolve_upvalue(name_token.lexeme());
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
                panic!("Undeclared variable \"{}\"", name_token.lexeme());
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
        let scope_depth = self.scopes.len();
        println!("Parser.end_scope() depth={}", scope_depth);
        loop {
            if output.locals.local_count() == 0 { break; }
            if output.locals.last_local().unwrap().depth() <= scope_depth { break; }
            println!("Parser.end_scope() destroy local variable '{}'", output.locals.last_local().unwrap().name());

            if output.locals.last_local().unwrap().is_captured() {
                println!(" with close upvalue");
                output.compiler.emit_op(&OpCode::CloseUpvalue);
            } else {
                println!(" with pop");
                output.compiler.emit_op(&OpCode::Pop);
            }
            output.locals.pop_local();
        }
    }
    
    fn scope(&mut self) -> Option<&mut Scope> {
        return self.scopes.last_mut(); // None = Global scope
    }

    // Compiling a function means spinning up another Parser, 
    // handing it a new compilation unit (Compiler with a Function object)
    // and letting it borrow our other inputs and outputs.
    fn function(&mut self, name: &str, kind: FunctionKind, input: &mut ParserInput, output: &mut ParserOutput) {
        output.locals.begin_function(kind.has_receiver());
    
        // Create a new compilation unit
        let mut function = Function::new(name, kind);    
        let mut compiler = Compiler::new(function);        
        
        let mut inner_output = ParserOutput {
            compiler:   &mut compiler,
            globals:    output.globals,
            locals:	output.locals,
        };
        
        // Create a new Parser and call parse_function()
        let mut parser = Parser::new();
        parser.classes = self.classes.clone();
        let result = parser.parse_function(input, &mut inner_output);
        if let Err(msg) = result {
            // TODO: Proper error handling
            panic!("{}", msg);
        }
        
        // Wrap the compiled Function in a Closure and store as a constant
        function = inner_output.compiler.take_function();
        let upvalues = output.locals.upvalue_count();
        function.set_upvalue_count(upvalues);
        let value = Value::function(function);
        println!("{:?}", value);
        let constant_id = output.compiler.make_constant(value);
        output.compiler.emit_op_variant(&OpCodeSet::capture(), constant_id as u64);
        for i in 0..upvalues {
            let upvalue = output.locals.upvalue_ref_by_id(i);
            let local_bit = if upvalue.is_local() { 128 } else { 0 };
            let id = upvalue.id();
            let mut id_len = 1;
            if id > 255 { id_len = 2; }
            if id > 65535 { id_len = 4; }
            output.compiler.emit_byte(local_bit + id_len);
            match id_len {
                1 => output.compiler.emit_byte(id as u8),
                2 => output.compiler.emit_word(id as u16),
                4 => output.compiler.emit_dword(id as u32),
                _ => {}, // Impossible
            }
        }
        
        
        output.locals.end_function();
    }
    
    fn method(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        //println!("Parser.method()");
        self.consume(TokenKind::Identifier, "Expect method name", input, output);
        let name_constant = self.identifier_constant(input.tokenizer.previous(), output);
        let name = input.tokenizer.previous().lexeme().to_string();
        //println!("Parser.method() begin compiling method {}", name);
        let kind = if name == INITIALIZER { FunctionKind::Initializer } else { FunctionKind::Method };
        self.function(&name, kind, input, output);
        //println!("Parser.method() finished compiling method {}", name);
        //println!("prev={:?}", input.tokenizer.previous());
        //println!("curr={:?}", input.tokenizer.current());
        output.compiler.emit_op_variant(&OpCodeSet::method(), name_constant as u64);
    }

    // Parse arguments passed when calling a callee
    fn argument_list(&mut self, input: &mut ParserInput, output: &mut ParserOutput) -> Result<u8, String> {
        let mut arg_count = 0;
        if !input.tokenizer.matches(TokenKind::RightParen) {
            loop {
                self.expression(input, output);
                arg_count = arg_count + 1;
                // Keep going?
                if !input.tokenizer.advance_on(TokenKind::Comma) { break; }
            }
        }
        
        self.consume(TokenKind::RightParen, "Expect ')' after arguments", input, output);
        return Ok(arg_count);
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
        } else if input.tokenizer.advance_on(TokenKind::Debug) {
            self.debug_statement(input, output);
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
        } else if input.tokenizer.advance_on(TokenKind::Return) {
            self.return_statement(input, output);
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

    fn debug_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        self.expression(input, output);
        self.consume(TokenKind::Semicolon, "Expect ';' after expression", input, output);
        output.compiler.emit_op(&OpCode::Debug); // Print result using std::fmt::Debug
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
        output.compiler.emit_op(&OpCode::Print); // Print result using std::fmt::Display
    }
    
    fn return_statement(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        let function_kind = output.compiler.function().kind();
        if function_kind.is_toplevel() { panic!("Can't return from top level code."); }
        if input.tokenizer.advance_on(TokenKind::Semicolon) {
            self.emit_return(output); // Pushes 'this' or null as needed
        } else {
            if function_kind.return_self() { panic!("Can't return a value from initializer."); }
            self.expression(input, output);
            self.consume(TokenKind::Semicolon, "Expect ';' after expression", input, output);
        }
        output.compiler.emit_op(&OpCode::Return);
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
        let inner_loop = self.inner_loop();
        match inner_loop {
            Some(codeloop) => {
                // Reset to loop's scope
                let end_scopes = scope_depth - codeloop.scope_depth();
                for _i in 0..end_scopes {
                    self.end_scope(output);
                }
            }
            None => {
                // TODO: Proper error handling
                panic!("'continue' not allowed here");
            }
        }
        // At this point we know the inner_loop exists
        let codeloop = self.inner_loop().unwrap();
        output.compiler.emit_op(&OpCode::Jmp);
        output.compiler.emit_dword(codeloop.continue_addr());
    }
    
    fn break_loop(&mut self, output: &mut ParserOutput) {
        let scope_depth = self.scopes.len();
        let inner_loop = self.inner_loop();
        match inner_loop {
            Some(codeloop) => {
                // Reset to loop's scope
                let end_scopes = scope_depth - codeloop.scope_depth();
                for _i in 0..end_scopes {
                    self.end_scope(output);
                }
            }
            None => {
                // TODO: Proper error handling
                panic!("'break' not allowed here");
            }
        }
        // At this point we know the inner_loop exists
        let codeloop = self.inner_loop().unwrap();
        codeloop.add_break(output.compiler.emit_jmp(&OpCode::Jmp));
    }
    
    fn end_loop(&mut self, output: &mut ParserOutput) -> u32 {
        match self.codeloops.pop() {
            Some(codeloop) => {
                // Jump back to the beginning of the loop
                output.compiler.emit_op(&OpCode::Jmp);
                output.compiler.emit_dword(codeloop.continue_addr());
                
                // Then patch any 'break' statements to jump here
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
            TokenKind::Class 	=> self.class_declaration(input, output),
            TokenKind::Fun 	=> self.fun_declaration(input, output),
            TokenKind::Var	=> self.var_declaration(input, output),
            _			=> self.statement(input, output),
        }
        //println!("Parser.declaration() end");
    }

    fn class_declaration(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        input.tokenizer.advance(); // Consume Class token
        let name_id = self.parse_variable("Expect class name", input, output);
        let name_token = input.tokenizer.previous().clone();
        let name_constant = self.identifier_constant(&name_token, output);
        self.classes.push(name_token.lexeme(), name_token.clone());
        //self.declare_variable(input, output); // Already declared in parse_variable()!
        output.compiler.emit_op_variant(&OpCodeSet::class(), name_constant as u64);
        self.define_variable(name_id, output);
        // At runtime, load the class onto the stack so we can manipulate it
        self.named_variable(&name_token, false, input, output);
        self.consume(TokenKind::LeftCurly, "Expect '{' after class name", input, output);
        //println!("Parser.class_declaration() begin parsing methods");
        loop {
            if input.tokenizer.matches(TokenKind::RightCurly) { break; }
            if input.tokenizer.eof() { break; }
            // We don't have field declarations, only methods
            self.method(input, output);
        }
        //println!("Parser.class_declaration() finished parsing methods");
        self.consume(TokenKind::RightCurly, "Expect '}' after class body", input, output);
        // We're done manipulating the class
        //println!("defined new class: {:?}", self.classes.current_path());
        self.classes.pop();
        output.compiler.emit_op(&OpCode::Pop);
    }

    fn fun_declaration(&mut self, input: &mut ParserInput, output: &mut ParserOutput) {
        input.tokenizer.advance(); // Consume Fun token
        let name_id = self.parse_variable("Expect function name", input, output);
        let name = input.tokenizer.previous().lexeme().to_string();
        //mark_initialized();
        self.function(&name, FunctionKind::Function, input, output);
        self.define_variable(name_id, output);
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

    fn call(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        let result = self.argument_list(input, output);
        match result {
            Ok(arg_count) => {
                output.compiler.emit_op(&OpCode::Call);
                output.compiler.emit_byte(arg_count);
            }
            Err(msg) => {
                // TODO: Proper error handling
                panic!("{}", msg);
            }        
        }
    }

    fn dot(&mut self, can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        self.consume(TokenKind::Identifier, "Expect property name after '.'", input, output);
        let name_id = self.identifier_constant(input.tokenizer.previous(), output);
    
        if can_assign && input.tokenizer.advance_on(TokenKind::Equal) {
            self.expression(input, output);
            output.compiler.emit_op_variant(&OpCodeSet::setproperty(), name_id as u64);
        } else {
            output.compiler.emit_op_variant(&OpCodeSet::getproperty(), name_id as u64);
        }
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
    fn this_(&mut self, _can_assign: bool, input: &mut ParserInput, output: &mut ParserOutput) {
        if self.classes.current_name().is_none() { panic!("Can not use 'this' outside of a class."); }
        //panic!("Not yet implemented.");
        self.variable(false, input, output)
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
            TokenKind::Print => return ParserRule::null(),
            TokenKind::Return => return ParserRule::null(),
            TokenKind::This => return ParserRule {
                prefix: 	Some(Parser::this_), 
                infix: 		None, 
                precedence: 	ParserPrec::None,
            },
            TokenKind::Var => return ParserRule::null(),
            TokenKind::Fun => return ParserRule::null(),
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
