
#[cfg(test)]
mod test;

use log::{trace, debug};

use super::codeloop::CodeLoop;
use super::compiler::Compiler;
use crate::lox::{CompileError, c_error};
use super::class::Class;
use crate::lox::{Token, TokenKind};
use super::function::Function;
use super::function_kind::FunctionKind;
use super::hierarchy::Hierarchy;
use super::keyword::*;
use super::parser_output::ParserOutput;
use super::scope::Scope;
use super::tokenizer::Tokenize;
use super::opcode::{OpCode, OpCodeSet};
use super::value::Value;


mod parser_prec;
mod parser_rule;

use parser_prec::ParserPrec;
use parser_rule::ParserRule;


#[allow(dead_code)]
pub struct Parser<I> {
    scopes: 	Vec<Scope>,
    classes:    Hierarchy<Class>,
    codeloops:	Vec<CodeLoop>,
    _unused: std::marker::PhantomData<*const I>,
}


#[allow(dead_code)]
impl<I: Tokenize> Parser<I> {
    pub fn new() -> Parser<I> {
        Parser {
            scopes: 	vec![],
            classes:    Hierarchy::new(),
            codeloops:	vec![],
            _unused: std::marker::PhantomData,
        }
    }

    // Parse __main__ function only (See: parse_function())    
    pub fn parse(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<Function, CompileError> {
        
        loop {
            trace!("loop begins");
            if input.eof() { break; }
            self.declaration(input, output)?;
        }
        self.emit_exit(output);
        
        return Ok(output.compiler.take_function());
    }


    // Shorthand
    fn consume(&self, kind: TokenKind, errmsg: &str, input: &mut I, _output: &mut ParserOutput) -> Result<(), CompileError> {
        if input.advance_on(kind) {
            Ok(())
        } else {
            c_error!(format!("{}, got '{}'", errmsg, input.current().lexeme()), input.current())
        }
    }


    fn emit_return(&self, output: &mut ParserOutput) {
        if output.compiler.function().kind().return_self() {
            output.compiler.emit_op(&OpCode::GetLocal8);
            output.compiler.emit_bytes(0, 1);
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
    
    fn current_token_rule(&self, input: &mut I) -> ParserRule<I> {
        let kind = input.current().kind();
        return ParserRule::<I>::for_token(&kind);
    }
    
    fn previous_token_rule(&self, input: &mut I) -> ParserRule<I> {
        let kind = input.previous().kind();
        return ParserRule::<I>::for_token(&kind);
    }
    
    // This is the core of the expression parser
    // The code is my amateurish attempt to re-implement in Rust
    // compiler.c:parsePrecedence() from Robert Nystrom's excellent book
    // http://craftinginterpreters.com
    // Please accept my apologies.
    fn parse_precedence(&mut self, precedence: ParserPrec, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {

        input.advance();
        trace!("begin compiling precedence at token={:?}", input.previous());
        let rule = self.previous_token_rule(input);
        
        match rule.prefix {
            Some(method) => {
                let can_assign = precedence <= ParserPrec::Assignment;
                method(self, can_assign, input, output)?; // Call the Compiler method pointer
                
                loop {
                    let rule = self.current_token_rule(input);
                    if precedence > rule.precedence { break; }
                    
                    input.advance();

                    match rule.infix {
                        Some(method) => {
                            method(self, can_assign, input, output)?; // Call the Compiler method pointer
                        }
                        None => {
                            // Not sure if this is even reachable; clox does not test for this
                            c_error!(format!("Expected expression"), input.current())
                        }
                    }
                }
                
                if can_assign && input.matches(TokenKind::Equal) {
                    c_error!(format!("Invalid assignment target"), input.current())
                }
                
            }
            None => c_error!(format!("Expected expression"), input.current()),
        }
        Ok(())
    }

    // Note: called instead of parse() to handle functions/methods
    fn parse_function(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.begin_scope();

        // Parameter list        
        self.consume(TokenKind::LeftParen, "Expected '(' after function name", input, output)?;
        let arity = self.parse_function_params(input, output)?;
        output.compiler.function().set_arity(arity);

        self.consume(TokenKind::RightParen, "Expected ')' after parameters.", input, output)?;
        
        // Body
        self.consume(TokenKind::LeftCurly, "Expected '{' before function body.", input, output)?;
        self.block(input, output)?; // Handles the closing curly
        
        self.emit_return(output);

        self.end_scope(output);
        
        Ok(())
    }
    
    fn parse_function_params(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<u8, CompileError> {
        let mut arity = 0;
        if !input.matches(TokenKind::RightParen) {
            loop {
                if arity == 255 { c_error!(format!("Can not have more than 255 parameters"), input.current()) }
                arity = arity + 1;
                let name_id = self.parse_variable("Expected parameter name", input, output)?;
                self.define_variable(name_id, output);
                // Keep going?
                if !input.advance_on(TokenKind::Comma) { break; }
                if input.matches(TokenKind::RightParen) { break; } // That was a trailing comma
            }
        }
        return Ok(arity);
    }

    fn parse_variable(&mut self, errmsg: &str, input: &mut I, output: &mut ParserOutput) -> Result<usize, CompileError> {
        
        self.consume(TokenKind::Identifier, errmsg, input, output)?;
        debug!("identifier={}", input.previous().lexeme());
        
        self.declare_variable(input, output)?;
        if let Some(_) = self.scope() { return Ok(0); }
        
        let name = input.previous().lexeme();
        match output.globals.declare(name) {
            Err(mut compile_error) => {
                compile_error.set_at(input.previous().get_at());
                return Err(compile_error);
            }
            Ok(id) => return Ok(id),
        }
    }
    
    // Make a constant containing the variable name as a Value::String
    fn identifier_constant(&mut self, token: &Token, output: &mut ParserOutput) -> usize {
        let name = Value::string(token.lexeme());  
        debug!("make constant={}", name);
        return output.compiler.make_constant(name);
    }
 
    fn declare_variable(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        
        let scope = self.scope();
        match scope {
            None => { return Ok(()); } // Global scope
            Some(_) => {
                let scope_depth = self.scopes.len();
                let name = input.previous().lexeme();
                debug!("add local identifier={}", name);

                // Verify variable is not already declared in this scope
                if let Some(id) = output.locals.resolve_local(name) {
                    if output.locals.local_ref_by_id(id).depth() == scope_depth {
                        c_error!(format!("Variable named '{}' already declared in this scope", name), input.previous())
                    }
                }

                output.locals.declare_local(name, scope_depth); // Add local variable
                Ok(())
            }
        }
    }
    
    fn define_variable(&mut self, id: usize, output: &mut ParserOutput) {
        
        if let Some(_) = self.scope() {
            debug!("define as local");
            output.locals.last_local().unwrap().define();
            return;
        }
        
        debug!("define as global");
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
    
    fn variable_opcodes(&mut self, name_token: &Token, output: &mut ParserOutput) -> Result<(OpCodeSet, OpCodeSet, usize), CompileError> {
        let mut result;
        
        result = output.locals.resolve_local(name_token.lexeme());
        match result {
            Some(id) => {
                if !output.locals.local_ref_by_id(id).is_defined() {
                    c_error!(format!("Can not read local variable in its own initializer"))
                }
                return Ok((
                    OpCodeSet::getlocal(),
                    OpCodeSet::setlocal(),
                    id
                ));
            }
            None => {}
        }
        
        result = output.locals.resolve_upvalue(name_token.lexeme());
        match result {
            Some(id) => {
                return Ok((
                    OpCodeSet::getupvalue(),
                    OpCodeSet::setupvalue(),
                    id
                ));
            }
            None => {}
        }
        
        //let id = self.identifier_constant(name_token, output);
        result = self.resolve_global(name_token.lexeme(), output);
        match result {
            Some(id) => {
                return Ok((
                    OpCodeSet::getglobal(),
                    OpCodeSet::setglobal(),
                    id
                ));
            }
            None => {
                c_error!(format!("Undeclared variable '{}'", name_token.lexeme()))
            }
        }
    }
    
    fn named_variable(&mut self, name_token: &Token, can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        // Get opcodes for get/set and id of local, upvalue or global
        //let (get_ops, set_ops, id) = self.variable_opcodes(name_token, output);
        match self.variable_opcodes(name_token, output) {
            Err(mut compile_error) => {
                compile_error.set_at(input.previous().get_at());
                Err(compile_error)
            }  
            Ok((get_ops, set_ops, id)) => {
                // Pick set or get based on context
                if can_assign && input.advance_on(TokenKind::Equal) {
                    self.expression(input, output)?;
                    output.compiler.emit_op_variant(&set_ops, id as u64);
                } else {
                    output.compiler.emit_op_variant(&get_ops, id as u64);
                }
                Ok(())
            }
        }
    }
    
    fn begin_scope(&mut self) {
        let depth = self.scopes.len() as u32;
        self.scopes.push(Scope::new(depth));
    }

    fn end_scope(&mut self, output: &mut ParserOutput) {
        self.scopes.pop();
        let scope_depth = self.scopes.len();
        debug!("scope depth={}", scope_depth);
        loop {
            if output.locals.local_count() == 0 { break; }
            if output.locals.last_local().unwrap().depth() <= scope_depth { break; }
            debug!("destroy local={}", output.locals.last_local().unwrap().name());

            if output.locals.last_local().unwrap().is_captured() {
                debug!(" with CloseUpvalue");
                output.compiler.emit_op(&OpCode::CloseUpvalue);
            } else {
                debug!(" with pop");
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
    fn function(&mut self, name: &str, kind: FunctionKind, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
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
        parser.parse_function(input, &mut inner_output)?;
        
        // Wrap the compiled Function in a Closure and store as a constant
        function = inner_output.compiler.take_function();
        let upvalues = output.locals.upvalue_count();
        function.set_upvalue_count(upvalues);
        let value = Value::function(function);
        debug!("{:?}", value);
        let constant_id = output.compiler.make_constant(value);
        output.compiler.emit_op_variant(&OpCodeSet::capture(), constant_id as u64);
        for i in 0..upvalues {
            let upvalue = output.locals.upvalue_ref_by_id(i);
            let local_bit = if upvalue.is_local() { 128 } else { 0 };
            let id = upvalue.id();
            let mut id_len = 1;
            if id > 255 { id_len = 2; }
            if id > 65535 { id_len = 4; }
            output.compiler.emit_bytes(local_bit + id_len, 1);
            output.compiler.emit_bytes(id as u32, id_len as usize);
        }
        
        
        output.locals.end_function();
        Ok(())
    }
    
    fn method(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.consume(TokenKind::Identifier, "Expected method name", input, output)?;
        let name_constant = self.identifier_constant(input.previous(), output);
        let name = input.previous().lexeme().to_string();
        debug!("begin compiling method={}", name);
        let kind = if name == KEYWORD_INIT { FunctionKind::Initializer } else { FunctionKind::Method };
        self.function(&name, kind, input, output)?;
        output.compiler.emit_op_variant(&OpCodeSet::method(), name_constant as u64);
        Ok(())
    }

    // Parse arguments passed when calling a callee
    fn argument_list(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<u8, CompileError> {
        let arg_count = self.expressions_until(TokenKind::RightParen, input, output)?;
        if arg_count > 255 { c_error!(format!("Can not have more than 255 arguments")) }
        self.consume(TokenKind::RightParen, "Expected ')' after arguments", input, output)?;
        return Ok(arg_count as u8);
    }


    // ======== Statements ========


    fn statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        trace!("begin compiling statement at token={:?}", input.current());
        if input.advance_on(TokenKind::Break) {
            self.break_statement(input, output)
        } else if input.advance_on(TokenKind::Continue) {
            self.continue_statement(input, output)
        } else if input.advance_on(TokenKind::Debug) {
            self.debug_statement(input, output)
        } else if input.advance_on(TokenKind::Exit) {
            self.exit_statement(input, output)
        } else if input.advance_on(TokenKind::For) {
            self.for_statement(input, output)
        } else if input.advance_on(TokenKind::If) {
            self.if_statement(input, output)
        } else if input.advance_on(TokenKind::LeftCurly) {
            self.begin_scope();
            let result = self.block(input, output);
            self.end_scope(output);
            result
        } else if input.advance_on(TokenKind::Print) {
            self.print_statement(input, output)
        } else if input.advance_on(TokenKind::Return) {
            self.return_statement(input, output)
        } else if input.advance_on(TokenKind::While) {
            self.while_statement(input, output)
        } else if input.advance_on(TokenKind::Else) {
            self.bad_statement(input, output)
        } else {
            self.expression_statement(input, output)
        }
    }

    fn block(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        loop {
            if input.eof() { break; }
            if input.matches(TokenKind::RightCurly) { break; }
            self.declaration(input, output)?;
        }
        self.consume(TokenKind::RightCurly, "Expected '}' after block", input, output)?;
        Ok(())
    }
    
    fn bad_statement(&mut self, input: &mut I, _output: &mut ParserOutput) -> Result<(), CompileError> {
        c_error!(format!("Keyword '{}' is misplaced", input.previous().lexeme()), input.previous())
    }

    fn want_semicolon_after(&mut self, what: &str, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        // Relax the rule if this looks like the end of a statement (required by C-style for loop increment)
        if input.matches(TokenKind::EOF) { return Ok(()) }
        if input.matches(TokenKind::RightCurly) { return Ok(()) }
        if input.matches(TokenKind::RightParen) { return Ok(()) }
        return self.consume(TokenKind::Semicolon, format!("Expected ';' after {}", what).as_str(), input, output);
    }

    fn break_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.break_loop(input, output)?;
        self.want_semicolon_after(KEYWORD_BREAK, input, output)?;
        Ok(())
    }

    fn continue_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.continue_loop(input, output)?;
        self.want_semicolon_after(KEYWORD_CONTINUE, input, output)?;
        Ok(())
    }

    fn debug_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.expression(input, output)?;
        self.want_semicolon_after("expression", input, output)?;
        output.compiler.emit_op(&OpCode::Debug); // Print result using std::fmt::Debug
        Ok(())
    }
    
    fn expression_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.expression(input, output)?;
        self.want_semicolon_after("expression", input, output)?;
        output.compiler.emit_op(&OpCode::Pop); // Discard result
        Ok(())
    }

    fn exit_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        if input.advance_on(TokenKind::Semicolon) {
            // No expression after 'exit'
            output.compiler.emit_op(&OpCode::Null);
        } else {
            self.expression(input, output)?;
            self.want_semicolon_after("expression", input, output)?;
        }
        output.compiler.emit_op(&OpCode::Exit);
        Ok(())
    }

    fn if_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        // if..
        let negate = input.advance_on(TokenKind::Not);
        self.consume(TokenKind::LeftParen, format!("Expected '(' after '{}'", input.previous().lexeme()).as_str(), input, output)?;
        if input.current().matches(TokenKind::RightParen) {
            c_error!(format!("Expected conditional expression, got '{}'", input.current().lexeme()), input.current())
        }
        self.expression(input, output)?;
        self.consume(TokenKind::RightParen, format!("Expected ')' after '{}'-condition", KEYWORD_IF).as_str(), input, output)?;
        if negate { output.compiler.emit_op(&OpCode::Negate) }
        
        let else_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseP);
        // ..then
        self.statement(input, output)?;
        let end_jmp = output.compiler.emit_jmp(&OpCode::Jmp);
        output.compiler.patch_jmp(else_jmp);
        if input.advance_on(TokenKind::Else) {
            // ..else
            self.statement(input, output)?;
        }
        output.compiler.patch_jmp(end_jmp);
        Ok(())
    }
    
    fn print_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.expression(input, output)?;
        self.want_semicolon_after("expression", input, output)?;
        output.compiler.emit_op(&OpCode::Print); // Print result using std::fmt::Display
        Ok(())
    }
    
    fn return_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let function_kind = output.compiler.function().kind();
        if function_kind.is_toplevel() {
            c_error!(format!("Can not '{}' from top level code", KEYWORD_RETURN), input.previous())
        }
        if input.advance_on(TokenKind::Semicolon) {
            self.emit_return(output); // Pushes 'this' or null as needed
        } else {
            if function_kind.return_self() { 
                c_error!(format!("Can not '{}' a value from initializer", KEYWORD_RETURN), input.previous())
            }
            self.expression(input, output)?;
            self.want_semicolon_after("expression", input, output)?;
        }
        output.compiler.emit_op(&OpCode::Return);
        Ok(())
    }

    fn while_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.begin_scope();
        self.begin_loop(output);
        
        // while..
        let negate = input.advance_on(TokenKind::Not);
        self.consume(TokenKind::LeftParen, format!("Expected '(' after '{}'", input.previous().lexeme()).as_str(), input, output)?;
        if input.current().matches(TokenKind::RightParen) {
            c_error!(format!("Expected conditional expression, got '{}'", input.current().lexeme()), input.current())
        }
        if input.advance_on(TokenKind::Var) {
            self.var_expression(input, output)?;
        } else {
            self.expression(input, output)?;
        }
        self.consume(TokenKind::RightParen, format!("Expected ')' after '{}'-condition", KEYWORD_WHILE).as_str(), input, output)?;
        if negate { output.compiler.emit_op(&OpCode::Negate) }
        
        let end_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseP);
        // ..do
        self.statement(input, output)?;

        self.end_loop(output);
        output.compiler.patch_jmp(end_jmp);
        self.end_scope(output);
        Ok(())
    }


    fn for_statement(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        // for..
        if input.advance_on(TokenKind::LeftParen) {
            self.for_statement_cstyle(input, output)?;
        } else {
            c_error!(format!("'{}'-loop syntax error", KEYWORD_FOR))
        }
        Ok(())
    }


    // C-style loop: for (initializer; condition; increment) {}
    fn for_statement_cstyle(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.begin_scope();

        // initializer (if any)
        if !input.advance_on(TokenKind::Semicolon) { 
            self.declaration(input, output)?; // Expects and consumes trailing ';'
        }

        self.begin_loop(output);
        let loop_start = output.compiler.current_ip();

        // condition (if any)
        if !input.advance_on(TokenKind::Semicolon) {
            self.expression(input, output)?;
            self.want_semicolon_after(format!("'{}'-loop condition", KEYWORD_FOR).as_str(), input, output)?;
        } else {
            output.compiler.emit_op(&OpCode::True);
        }
        let end_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseP);
        let loop_body = output.compiler.emit_jmp(&OpCode::Jmp);

        // increment (if any)
        let after_each = output.compiler.current_ip();
        if !input.matches(TokenKind::RightParen) { 
            self.statement(input, output)?;
        }

        output.compiler.emit_op(&OpCode::Jmp);
        output.compiler.emit_bytes(loop_start, OpCode::Jmp.len());
        self.consume(TokenKind::RightParen, format!("Expected ')' after '{}'-loop increment", KEYWORD_FOR).as_str(), input, output)?;

        // loop body
        output.compiler.patch_jmp(loop_body);
        self.statement(input, output)?;
        output.compiler.emit_op(&OpCode::Jmp);
        output.compiler.emit_bytes(after_each, OpCode::Jmp.len());

        self.end_loop(output);
        output.compiler.patch_jmp(end_jmp);
        self.end_scope(output);
        Ok(())
    }

    // ======== Loop break/continue handling ========

    
    fn begin_loop(&mut self, output: &mut ParserOutput) -> u32 {
        let continue_addr = output.compiler.current_ip();
        let scope_depth = self.scopes.len();
        self.codeloops.push(CodeLoop::new(continue_addr, scope_depth));
        return 0;
    }
    
    fn inner_loop(&mut self) -> Option<&mut CodeLoop> {
        return self.codeloops.last_mut();
    }
    
    fn continue_loop(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
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
                c_error!(format!("Keyword '{}' is misplaced", KEYWORD_CONTINUE), input.previous())
            }
        }
        // At this point we know the inner_loop exists
        let codeloop = self.inner_loop().unwrap();
        output.compiler.emit_op(&OpCode::Jmp);
        output.compiler.emit_bytes(codeloop.continue_addr(), OpCode::Jmp.len());
        Ok(())
    }
    
    fn break_loop(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
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
            None => c_error!(format!("Keyword '{}' is misplaced", KEYWORD_BREAK), input.previous()),
        }
        // At this point we know the inner_loop exists
        let codeloop = self.inner_loop().unwrap();
        codeloop.add_break(output.compiler.emit_jmp(&OpCode::Jmp));
        Ok(())
    }
    
    fn end_loop(&mut self, output: &mut ParserOutput) -> u32 {
        match self.codeloops.pop() {
            Some(codeloop) => {
                // Jump back to the beginning of the loop
                output.compiler.emit_op(&OpCode::Jmp);
                output.compiler.emit_bytes(codeloop.continue_addr(), OpCode::Jmp.len());
                
                // Then patch any 'break' statements to jump here
                for address in codeloop.breaks() {
                    output.compiler.patch_jmp(*address);
                }
            }
            None => {
                panic!("Internal Error: end_loop() without a corresponding begin_loop()");
            }
        }
        return output.compiler.current_ip();
    }
    

    // ======== Declarations ========


    fn declaration(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        debug!("begin compiling declaration");
        match input.current().kind() {
            TokenKind::Class 	=> self.class_declaration(input, output),
            TokenKind::Fun 	    => self.fun_declaration(input, output),
            TokenKind::Var	    => self.var_declaration(input, output),
            _			        => self.statement(input, output),
        }
    }

    fn class_declaration(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError>{
        input.advance(); // Consume Class token
        let name_id = self.parse_variable("Expected class name", input, output)?;
        let name_token = input.previous().clone();
        let name_constant = self.identifier_constant(&name_token, output);
        self.classes.push(name_token.lexeme(), Class::new(&name_token));
        output.compiler.emit_op_variant(&OpCodeSet::class(), name_constant as u64);
        self.define_variable(name_id, output); // At this point, the VM will have defined the (empty) class

        // Check for superclass with syntax: class Name of Superclass {}
        if input.advance_on(TokenKind::Of) {
            self.consume(TokenKind::Identifier, "Expected superclass name", input, output)?;
            self.variable(false, input, output)?; // Look up superclass by name, load it on the stack
            let superclass_token = input.previous().clone();
            if name_token.lexeme() == superclass_token.lexeme() { 
                c_error!(format!("Class '{}' can not inherit from itself", name_token.lexeme()), input.previous())
            }
            self.begin_scope();

            // Copy superclass from globals to a local variable 'super'
            output.locals.declare_local(KEYWORD_SUPER, 0);
            self.named_variable(&superclass_token, false, input, output)?;
            self.define_variable(0, output);

            // Load current class onto the stack and copy methods from parent
            self.named_variable(&name_token, false, input, output)?;
            output.compiler.emit_op(&OpCode::Inherit);

            // Mark the current class as having a parent            
            self.classes.current_mut().unwrap().set_parent(input.previous());
        }

        // At runtime, load the class onto the stack so we can manipulate it
        self.named_variable(&name_token, false, input, output)?;
        self.consume(TokenKind::LeftCurly, "Expected '{' after class name", input, output)?;
        debug!("begin parsing methods");
        loop {
            if input.matches(TokenKind::RightCurly) { break; }
            if input.eof() { break; }
            // We don't have field declarations, only methods
            self.method(input, output)?;
        }
        debug!("finished parsing methods");
        self.consume(TokenKind::RightCurly, "Expected '}' after class body", input, output)?;
        // We're done manipulating the class
        debug!("defined new class: {:?}", self.classes.current_path());
        output.compiler.emit_op(&OpCode::Pop);
        if self.classes.current().unwrap().has_parent() { 
            self.end_scope(output); 
        }
        self.classes.pop();
        Ok(())
    }

    fn fun_declaration(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        input.advance(); // Consume Fun token
        let name_id = self.parse_variable("Expected function name", input, output)?;
        let name = input.previous().lexeme().to_string();
        self.function(&name, FunctionKind::Function, input, output)?;
        self.define_variable(name_id, output);
        Ok(())
    }

    fn var_declaration(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        input.advance(); // Consume Var token
        let name_id = self.parse_variable("Expected variable name", input, output)?;
        self.var_initializer(input, output)?;
        self.consume(TokenKind::Semicolon, "Expected ';' after variable declaration", input, output)?;
        self.define_variable(name_id, output);
        Ok(())
    } 


    // ======== Expressions ========

    fn expressions_until(&mut self, kind: TokenKind, input: &mut I, output: &mut ParserOutput) -> Result<u64, CompileError> {
        let mut counter = 0;
        if !input.matches(kind) {
            loop {
                counter = counter + 1;
                self.expression(input, output)?;
                // Keep going?
                if !input.advance_on(TokenKind::Comma) { break; }
                if input.matches(kind) { break; } // That was a trailing comma
            }
        }
        return Ok(counter);
    }    


    fn expression(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        trace!("begin compiling expression at token={}", input.current().lexeme());
        self.parse_precedence(ParserPrec::Assignment, input, output)?;
        Ok(())
    }

    fn and(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let end_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseQ);
        output.compiler.emit_op(&OpCode::Pop);
        self.parse_precedence(ParserPrec::And, input, output)?;
        output.compiler.patch_jmp(end_jmp);
        Ok(())
    }

    fn array(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let elements = self.expressions_until(TokenKind::RightBracket, input, output)?;
        self.consume(TokenKind::RightBracket, "Expected ']' after array elements", input, output)?;
        output.compiler.emit_op_variant(&OpCodeSet::defarray(), elements);
        Ok(())
    }

    fn base2number(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let lexeme = input.previous().lexeme();
        let without_prefix = lexeme.trim_start_matches("0b");
        let float = i64::from_str_radix(without_prefix, 2).unwrap() as f64;
        self.emit_constant(Value::number(float), output);
        Ok(())
    }

    fn base8number(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let lexeme = input.previous().lexeme();
        let without_prefix = lexeme.trim_start_matches("0o");
        let float = i64::from_str_radix(without_prefix, 8).unwrap() as f64;
        self.emit_constant(Value::number(float), output);
        Ok(())
    }

    fn base10number(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let lexeme = input.previous().lexeme();
        let float: f64 = lexeme.parse().unwrap();
        self.emit_constant(Value::number(float), output);
        Ok(())
    }

    fn base16number(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let lexeme = input.previous().lexeme();
        let without_prefix = lexeme.trim_start_matches("0x");
        let float = i64::from_str_radix(without_prefix, 16).unwrap() as f64;
        self.emit_constant(Value::number(float), output);
        Ok(())
    }

    fn binary(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError>{

        let mut operator = input.previous().kind();
        let rule = ParserRule::<I>::for_token(&operator);
        if operator == TokenKind::Is && input.advance_on(TokenKind::Not) {
            operator = input.previous().kind();
        }

        self.parse_precedence(rule.precedence.next(), input, output)?;
        
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

            // Keyword
            TokenKind::Is	=> output.compiler.emit_op(&OpCode::Same),
            TokenKind::Not  => {
                output.compiler.emit_op(&OpCode::Same);
                output.compiler.emit_op(&OpCode::Negate);
            }
            _ => {
                panic!("Internal Error: Unhandled binary operator {:?}", operator);
            }
        }
        Ok(())
    }

    fn call(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let arg_count = self.argument_list(input, output)?;
        output.compiler.emit_op(&OpCode::Call);
        output.compiler.emit_bytes(arg_count as u32, OpCode::Call.len());
        Ok(())
    }

    fn dot(&mut self, can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.consume(TokenKind::Identifier, "Expected property name after '.'", input, output)?;
        let name_id = self.identifier_constant(input.previous(), output);
    
        if can_assign && input.advance_on(TokenKind::Equal) {
            self.expression(input, output)?;
            output.compiler.emit_op_variant(&OpCodeSet::setproperty(), name_id as u64);
        } else {
            output.compiler.emit_op_variant(&OpCodeSet::getproperty(), name_id as u64);
        }
        Ok(())
    }

    fn grouping(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        self.expression(input, output)?;
        self.consume(TokenKind::RightParen, "Expect ')' after expression", input, output)?;
        Ok(())
    }

    fn literal(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let literal = input.previous().kind();
        match literal {
            TokenKind::False	=> output.compiler.emit_op(&OpCode::False),
            TokenKind::Inf	=> output.compiler.emit_op(&OpCode::Inf),
            TokenKind::Nan	=> output.compiler.emit_op(&OpCode::NaN),
            TokenKind::Null	=> output.compiler.emit_op(&OpCode::Null),
            TokenKind::True	=> output.compiler.emit_op(&OpCode::True),
            _ => {
                panic!("Internal Error: Unhandled literal {:?}", literal);
            }
        }
        Ok(())
    }

    fn or(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let else_jmp = output.compiler.emit_jmp(&OpCode::JmpFalseQ);
        let end_jmp = output.compiler.emit_jmp(&OpCode::Jmp);
        output.compiler.patch_jmp(else_jmp);
        output.compiler.emit_op(&OpCode::Pop);
        self.parse_precedence(ParserPrec::Or, input, output)?;
        output.compiler.patch_jmp(end_jmp);
        Ok(())
    }

    fn string(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let value = Value::string(input.previous().lexeme());
        self.emit_constant(value, output);
        Ok(())
    }

    fn subscr(&mut self, can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        // At this point, there should be a subscriptable value on the top of the stack
        // Build an array value, each element being an index into the first element
        let indices = self.expressions_until(TokenKind::RightBracket, input, output)?;
        self.consume(TokenKind::RightBracket, "Expected ']' after array elements", input, output)?;
        output.compiler.emit_op_variant(&OpCodeSet::defarray(), indices);
        // Now there are two values on the stack; the subscriptable value and an array of index values
        if can_assign && input.advance_on(TokenKind::Equal) {
            if indices == 0 { c_error!(format!("Can not assign to subscript '[]'")) }
            self.expression(input, output)?;
            output.compiler.emit_op(&OpCode::SetSubscript);
        } else {
            output.compiler.emit_op(&OpCode::GetSubscript);
        }
        Ok(())
    }

    fn super_(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        if self.classes.current_name().is_none() { 
            c_error!(format!("Can not use '{}' outside of a class", KEYWORD_SUPER), input.previous())
        }
        if !self.classes.current().unwrap().has_parent() { 
            c_error!(format!("Can not use '{}' in a class with no superclass", KEYWORD_SUPER), input.previous())
        }
        self.consume(TokenKind::Dot, format!("Expected '.' after '{}'", KEYWORD_SUPER).as_str(), input, output)?;
        self.consume(TokenKind::Identifier, "Expected superclass method name", input, output)?;
        let name_token = input.previous().clone();
        let name_constant = self.identifier_constant(&name_token, output);

        self.named_variable(&name_token.synthetic(KEYWORD_THIS, TokenKind::This), false, input, output)?;
        self.named_variable(&name_token.synthetic(KEYWORD_SUPER, TokenKind::Super), false, input, output)?;
        output.compiler.emit_op_variant(&OpCodeSet::get_super(), name_constant as u64);
        Ok(())
    }

    fn ternary(&mut self, _can_assign: bool, _input: &mut I, _output: &mut ParserOutput) -> Result<(), CompileError> {
        panic!("Not yet implemented.");
    }

    fn this_(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        if self.classes.current_name().is_none() { 
            c_error!(format!("Can not use '{}' outside of a class", KEYWORD_THIS), input.previous())
        }
        self.variable(false, input, output)?;
        Ok(())
    }

    fn unary(&mut self, _can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let operator = input.previous().kind();
        self.parse_precedence(ParserPrec::Unary, input, output)?;
        match operator {
            TokenKind::Bang 	=> output.compiler.emit_op(&OpCode::Not),
            TokenKind::Minus 	=> output.compiler.emit_op(&OpCode::Negate),
            _ => {
                panic!("Internal Error: Unhandled unary operator {:?}", operator);
            }
        }
        Ok(())
    }

    fn variable(&mut self, can_assign: bool, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let token = input.previous().clone();
        self.named_variable(&token, can_assign, input, output)?;
        Ok(())
    }

    // while() and for() loops can declare a single variable 
    // if the conditional expression starts with KEYWORD_VAR
    fn var_expression(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        let name_id = self.parse_variable("Expected variable name", input, output)?;
        self.var_initializer(input, output)?;
        // Unlike a var declaration, a var expression must leave a copy of the assigned value on the stack
        output.compiler.emit_op(&OpCode::Dup); 
        self.define_variable(name_id, output);
        Ok(())
    } 


    fn var_initializer(&mut self, input: &mut I, output: &mut ParserOutput) -> Result<(), CompileError> {
        if input.advance_on(TokenKind::Equal) {
            self.expression(input, output)?;
        } else {
            output.compiler.emit_op(&OpCode::Null);
        }
        Ok(())
    }

}
