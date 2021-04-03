
use std::rc::Rc;
use std::borrow::Borrow;

use super::callframe::CallFrame;
use super::stack::Stack;
use super::value::Value;
use super::obj::Obj;
//use super::obj::Obj;
use super::constants::Constants;
use super::globals::Globals;
use super::closure::Closure;
use super::function::Function;
use super::scanner::Scanner;
use super::tokenizer::Tokenizer;
use super::parser::{Parser, ParserInput, ParserOutput};
use super::compiler::Compiler;
use super::opcode::OpCode;


//#[allow(dead_code)]
pub struct VM {
  callframes: Vec<CallFrame>,
  stack: Stack<Value>,
  constants: Constants,
  globals: Globals,
  //objects: Vec<Obj>,
}


impl VM {
    pub fn new() -> VM {
        VM {
            callframes: 	vec![],
            stack: 		Stack::new(), 
            constants:		Constants::new(),
            globals:		Globals::new(),
            //objects: 		vec![],
        }
    }
}


//#[allow(unused_mut)]
impl VM {
    pub fn compile(&mut self, code: &str) -> Result<(), String> {
        println!("VM.compile() code={}", code);
        
        // -------------------------------------------------------
        // This is really bad, I know.
        // Much of the following code may belong within 
        // the Compiler but I'm keeping things here until I 
        // figure out exactly how the pieces need to fit together.
        // -------------------------------------------------------
        
        let scanner = Scanner::str(code);
        let mut tokenizer = Tokenizer::new(scanner);
        let mut function = Function::new("__main__", 0);    
        let mut compiler = Compiler::new(function);

        let mut parser = Parser::new();        
        //let mut parser = Parser::new(tokenizer, compiler);
        //parser.give_constants(constants);
        let mut input = ParserInput {
            tokenizer: &mut tokenizer,
        };
        let mut output = ParserOutput {
            compiler: 	&mut compiler,
            constants: 	&mut self.constants,
            globals: 	&mut self.globals,
        };
        let result = parser.parse(&mut input, &mut output);
        
        //self.constants = Some(parser.take_constants());
        //compiler = parser.take_compiler();
        function = compiler.take_function();
        
        println!("VM.compile() complete:");
        println!(" function={:?}", function);
        println!(" constants={:?}", self.constants);
        println!(" globals={:?}", self.globals);
        
        match result {
            Ok(()) => {
                return self.setup_initial_callframe(function);
            }
            Err(msg) => {
                return Err(msg);
            }
        }
    }
}


impl VM {
    pub fn execute(&mut self) {
        println!("VM.execute()");
        
        loop {
            let ip = self.read_callframe().ip();
            println!("IP=0x{:04x} SP=0x{:04x}", ip, self.stack.size());
            let opcode = self.callframe().read_op();
            match opcode {
                None => {
                    panic!("VM.execute() reached end of function without return");
                }
                Some(opcode) => {
            
                    let result;

                    match opcode {
                        OpCode::Return 		=> {
                            let return_value = self.pop();
                            //self.close_upvalues();
                            self.callframes.pop();
                            if self.callframes.len() == 0 { return; }
                            
                            self.push(return_value);
                            result = Ok(());
                        }
                        OpCode::Print		=> result = self.opcode_print(),

                        OpCode::GetConst8 	=> result = self.opcode_getconst8(),
                        OpCode::GetConst16 	=> result = self.opcode_getconst16(),
                        OpCode::GetConst32 	=> result = self.opcode_getconst32(),
                        OpCode::False 		=> result = self.opcode_false(),
                        OpCode::Null 		=> result = self.opcode_null(),
                        OpCode::True	 	=> result = self.opcode_true(),
                        OpCode::GetLocal8 	=> result = self.opcode_getlocal8(),
                        OpCode::GetLocal16 	=> result = self.opcode_getlocal16(),
                        OpCode::GetLocal32 	=> result = self.opcode_getlocal32(),
                        OpCode::GetUpvalue8 	=> result = self.opcode_getupvalue8(),
                        OpCode::GetUpvalue16 	=> result = self.opcode_getupvalue16(),
                        OpCode::GetUpvalue32 	=> result = self.opcode_getupvalue32(),
                        OpCode::GetGlobal8 	=> result = self.opcode_getglobal8(),
                        OpCode::GetGlobal16 	=> result = self.opcode_getglobal16(),
                        OpCode::GetGlobal32 	=> result = self.opcode_getglobal32(),

                        OpCode::DefGlobal8	=> result = self.opcode_defglobal8(),
                        OpCode::DefGlobal16 	=> result = self.opcode_defglobal16(),
                        OpCode::DefGlobal32 	=> result = self.opcode_defglobal32(),
                        OpCode::SetLocal8 	=> result = self.opcode_setlocal8(),
                        OpCode::SetLocal16 	=> result = self.opcode_setlocal16(),
                        OpCode::SetLocal32 	=> result = self.opcode_setlocal32(),
                        OpCode::SetUpvalue8 	=> result = self.opcode_setupvalue8(),
                        OpCode::SetUpvalue16 	=> result = self.opcode_setupvalue16(),
                        OpCode::SetUpvalue32 	=> result = self.opcode_setupvalue32(),
                        OpCode::SetGlobal8 	=> result = self.opcode_setglobal8(),
                        OpCode::SetGlobal16 	=> result = self.opcode_setglobal16(),
                        OpCode::SetGlobal32 	=> result = self.opcode_setglobal32(),

                        OpCode::Not 		=> result = self.opcode_not(),
                        OpCode::Negate 		=> result = self.opcode_negate(),

                        OpCode::Add 		=> result = self.opcode_add(),
                        OpCode::Sub 		=> result = self.opcode_sub(),
                        OpCode::Mul 		=> result = self.opcode_mul(),
                        OpCode::Div 		=> result = self.opcode_div(),
                        OpCode::Mod 		=> result = self.opcode_mod(),
                        OpCode::Equal		=> result = self.opcode_equal(),
                        OpCode::NotEqual	=> result = self.opcode_notequal(),
                        OpCode::Less		=> result = self.opcode_less(),
                        OpCode::Greater		=> result = self.opcode_greater(),
                        OpCode::LessEqual	=> result = self.opcode_lessequal(),
                        OpCode::GreaterEqual	=> result = self.opcode_greaterequal(),

                        OpCode::Jmp 		=> result = self.opcode_jmp(),
                        OpCode::JmpFalseP	=> result = self.opcode_jmpfalsep(),
                        OpCode::JmpFalseQ	=> result = self.opcode_jmpfalseq(),

                        OpCode::Pop 		=> result = self.opcode_pop(),
                        OpCode::PopN 		=> result = self.opcode_popn(),

                        OpCode::BAD 		=> result = self.opcode_bad(),
                    }
                    
                    // On error, dump message and return
                    match result {
                        Ok(()) => {
                        }
                        Err(message) => {
                            eprintln!(
                                "{} at ip={}\n{:?}", 
                                message,
                                ip, 
                                self.read_callframe().read_function()
                            );
                            return;
                        }
                    }
                }
            }
        }
    }
    
    pub fn callframe(&mut self) -> &mut CallFrame {
        return self.callframes.last_mut().unwrap();
    }

    pub fn read_callframe(&self) -> &CallFrame {
        return self.callframes.last().unwrap();
    }

    fn opcode_print(&mut self) -> Result<(), String> {
        let value = self.pop();
        println!("PRINT> {}", value);
        Ok(())
    }

    fn opcode_getconst(&mut self, id: usize) -> Result<(), String> {
        let value = self.constants.value_by_id(id).clone();
        self.push(value);
        Ok(())
    }
    
    fn opcode_getconst8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_getconst(id);
    }
    
    fn opcode_getconst16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_getconst(id);
    }
    
    fn opcode_getconst32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_getconst(id);
    }

    fn opcode_getlocal(&mut self, id: usize) -> Result<(), String> {
        println!("GETL 0x{:02x}", id);
        let depth = self.stack.size()
            - self.callframe().stack_bottom() as usize
            - 1
            - id;
        self.push(self.peek(depth).clone());
        Ok(())
    }

    fn opcode_getlocal8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_getlocal(id);
    }
    
    fn opcode_getlocal16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_getlocal(id);
    }
    
    fn opcode_getlocal32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_getlocal(id);
    }

    fn opcode_getupvalue(&mut self, _id: usize) -> Result<(), String> {
        Err("OpCode GETUPVALUE not implemented".to_string())
    }

    fn opcode_getupvalue8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_getupvalue(id);
    }
    
    fn opcode_getupvalue16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_getupvalue(id);
    }
    
    fn opcode_getupvalue32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_getupvalue(id);
    }

    fn opcode_getglobal(&mut self, id: usize) -> Result<(), String> {
        // Compiler guarantees the variable is defined
        self.push(self.globals.value_by_id(id).unwrap().clone());
        Ok(())
    }

    fn opcode_getglobal8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getglobal16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getglobal32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_false(&mut self) -> Result<(), String> {
        self.push(Value::boolean(false));
        Ok(())
    }
    
    fn opcode_null(&mut self) -> Result<(), String> {
        self.push(Value::null());
        Ok(())
    }
    
    fn opcode_true(&mut self) -> Result<(), String> {
        self.push(Value::boolean(true));
        Ok(())
    }

    fn opcode_defglobal(&mut self, id: usize) -> Result<(), String> {
        let value = self.pop();
        self.globals.define_by_id(id, value);
        Ok(())
    }
    
    fn opcode_defglobal8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_defglobal16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_defglobal32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_setlocal(&mut self, id: usize) -> Result<(), String> {
        let depth = self.stack.size() - 1
            - self.callframe().stack_bottom() as usize
            - id;
        self.poke(self.peek(0).clone(), depth);
        Ok(())
    }
    
    fn opcode_setlocal8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_setlocal(id);
    }
    
    fn opcode_setlocal16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_setlocal(id);
    }
    
    fn opcode_setlocal32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_setlocal(id);
    }

    fn opcode_setupvalue(&mut self, _id: usize) -> Result<(), String> {
        Err("OpCode SETUPVALUE not implemented".to_string())
    }

    fn opcode_setupvalue8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_setupvalue(id);
    }
    
    fn opcode_setupvalue16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_setupvalue(id);
    }
    
    fn opcode_setupvalue32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_setupvalue(id);
    }

    fn opcode_setglobal(&mut self, id: usize) -> Result<(), String> {
        let value = self.peek(0).clone();
        self.globals.define_by_id(id, value);
        Ok(())
    }

    fn opcode_setglobal8(&mut self) -> Result<(), String> {
        let id = self.callframe().read_byte() as usize;
        return self.opcode_setglobal(id);
    }
    
    fn opcode_setglobal16(&mut self) -> Result<(), String> {
        let id = self.callframe().read_word() as usize;
        return self.opcode_setglobal(id);
    }
    
    fn opcode_setglobal32(&mut self) -> Result<(), String> {
        let id = self.callframe().read_dword() as usize;
        return self.opcode_setglobal(id);
    }

    fn opcode_not(&mut self) -> Result<(), String> {
        let value = self.pop();
        self.push(Value::boolean(!value.truthy()));
        Ok(())
    }
    
    fn opcode_negate(&mut self) -> Result<(), String> {
        let value = self.pop();
        match value {
            Value::Bool(b) => self.push(Value::boolean(!b)),
            Value::Number(n) => self.push(Value::number(-n)),
            _ => self.push(Value::Null),
        }
        Ok(())
    }
    
    fn opcode_add(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.add(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_sub(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.subtract(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_mul(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.multiply(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_div(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.divide(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_mod(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.modulo(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_equal(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a == b));
        Ok(())
    }
    
    fn opcode_notequal(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a != b));
        Ok(())
    }
    
    fn opcode_less(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a < b));
        Ok(())
    }
    
    fn opcode_greater(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a > b));
        Ok(())
    }
    
    fn opcode_lessequal(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a <= b));
        Ok(())
    }
    
    fn opcode_greaterequal(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a >= b));
        Ok(())
    }

    fn opcode_jmp(&mut self) -> Result<(), String> {
        let ip = self.callframe().read_dword();
        self.callframe().jmp(ip);
        Ok(())
    }
    
    fn opcode_jmpfalsep(&mut self) -> Result<(), String> {
        let ip = self.callframe().read_dword();
        if !self.pop().truthy() { self.callframe().jmp(ip); }
        Ok(())
    }

    fn opcode_jmpfalseq(&mut self) -> Result<(), String> {
        let ip = self.callframe().read_dword();
        if !self.peek(0).truthy() { self.callframe().jmp(ip); }
        Ok(())
    }

    fn opcode_pop(&mut self) -> Result<(), String> {
        let value = self.pop();
        println!("POP = {}", value);
        Ok(())
    }
    
    fn opcode_popn(&mut self) -> Result<(), String> {
        let count = self.callframe().read_byte();
        for _ in 0..count {
            let value = self.pop();
            println!("POP = {}", value);
        }
        Ok(())
    }
    
    fn opcode_bad(&mut self) -> Result<(), String> {
        Err("Bad OpCode".to_string())
    }
}


#[allow(dead_code)]
impl VM {
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Value {
        let value = self.stack.pop();
        return value;
    }
    fn peek(&self, depth: usize) -> &Value {
        return self.stack.peek(depth);
    }
    fn poke(&mut self, value: Value, depth: usize) {
        self.stack.poke(value, depth);
    }
    fn setup_initial_callframe(&mut self, function: Function) -> Result<(), String>{
        let closure = Closure::new(function);
        let value = Value::closure(closure);
        //self.push(value);
        self.call_value(value, 0); // Main function takes zero arguments
        return Ok(());
    }

    fn call(&mut self, rc_closure: Rc<Obj>, argc: u8) {
        //let slots: u32 = (self.stack.size() as u32) - (argc as u32) - 1;
        let stack_bottom = (self.stack.size() as u32) - (argc as u32);
        let callframe = CallFrame::new(rc_closure, stack_bottom);
        self.callframes.push(callframe);
    }
    
    // Stack: Value to be called
    fn call_value(&mut self, value: Value, argc: u8) {
        match value {
            Value::Obj(ref obj) => {
                let rc_object = value.as_rc_object();
                match obj.borrow() {
                    Obj::Closure(_) => {
                        //let value = self.pop();
                        self.call(rc_object, argc);
                    }
                    _ => {
                        panic!("VM.call_value({}, {}) not implemented.", value, argc);
                    }
                }
            }
            _ => {
                panic!("VM.call_value({}, {}) not implemented.", value, argc);
            }
        }
    }
}


impl Drop for VM {
    fn drop(&mut self) {
        //println!("VM.drop()");
    }
}






