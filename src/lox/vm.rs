
#[cfg(test)]
mod test;

pub mod upvalue;
use upvalue::Upvalue;

use super::callframe::CallFrame;
use super::stack::Stack;
use super::value::Value;
use super::globals::Globals;
use super::locals::Locals;
use super::class::Class;
use super::instance::Instance;
use super::closure::Closure;
use super::function::{Function, FunctionKind};
use super::scanner::Scanner;
use super::tokenizer::Tokenizer;
use super::parser::{Parser, ParserInput, ParserOutput};
use super::compiler::Compiler;
use super::opcode::OpCode;


pub struct VM {
    callframes: Vec<CallFrame>,
    stack: Stack<Value>,
    globals: Globals<Value>,
    open_upvalues: Vec<Upvalue<Value>>, // Note: Runtime representation
}


impl VM {
    pub fn new() -> VM {
        VM {
            callframes: 	vec![],
            stack: 		Stack::new(), 
            globals:		Globals::new(),
            open_upvalues:	vec![],
        }
    }
}


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
        let mut function = Function::new("__main__", FunctionKind::Script);    
        let mut compiler = Compiler::new(function);

        let mut parser = Parser::new();        
        //let mut parser = Parser::new(tokenizer, compiler);
        //parser.give_constants(constants);
        let mut input = ParserInput {
            tokenizer: &mut tokenizer,
        };
        let mut output = ParserOutput {
            compiler: 	&mut compiler,
            //constants: 	&mut self.constants,
            globals: 	&mut self.globals,
            locals:	&mut Locals::new(), // Discard after compile
        };
        let result = parser.parse(&mut input, &mut output);
        
        //self.constants = Some(parser.take_constants());
        //compiler = parser.take_compiler();
        function = compiler.take_function();
        
        println!("VM.compile() complete:");
        println!(" function={:?}", function);
        //println!(" constants={:?}", self.constants);
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
    pub fn execute(&mut self) -> i32 {
        println!("VM.execute()");
        
        loop {
            let ip = self.callframe().ip();
            let fn_name = self.callframe().closure_ref().function_ref().name().to_string();

            // Trace VM state
            println!("IP={}:0x{:04x} SP=0x{:04x}", fn_name, ip, self.stack.size());
            println!(" stack={:?}", self.stack);

            let opcode = self.callframe_mut().read_op();
            
            let result;

            match opcode {
                OpCode::Exit		=> {
                    let return_value = self.pop();
                    //println!("OpCode::Exit, close_upvalues");
                    // Execute may be called again so be sure to close any open upvalues
                    self.close_upvalues(self.callframe().stack_bottom());
                    // Rather than wasting time unwinding the stacks,
                    // simply discard them because the script is terminating.
                    // If execute gets called again, we need a clean slate.
                    self.stack = Stack::new();
                    self.callframes = vec![];
                    match return_value {
                        Value::Number(n) => return n as i32,
                        _ => return 0,
                    }
                }
                OpCode::Return 		=> {
                    let return_value = self.pop();
                    //println!("OpCode::Return, close_upvalues");
                    self.close_upvalues(self.callframe().stack_bottom());
                    self.callframes.pop();
                    if self.callframes.len() == 0 { return 0; }
                    
                    self.push(return_value);
                    result = Ok(());
                }
                OpCode::Debug		=> result = self.opcode_debug(),
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
                OpCode::GetProperty8 	=> result = self.opcode_getproperty8(),
                OpCode::GetProperty16 	=> result = self.opcode_getproperty16(),
                OpCode::GetProperty32 	=> result = self.opcode_getproperty32(),

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
                OpCode::SetProperty8 	=> result = self.opcode_setproperty8(),
                OpCode::SetProperty16 	=> result = self.opcode_setproperty16(),
                OpCode::SetProperty32 	=> result = self.opcode_setproperty32(),

                OpCode::Capture8 	=> result = self.opcode_capture8(),
                OpCode::Capture16 	=> result = self.opcode_capture16(),
                OpCode::Capture32 	=> result = self.opcode_capture32(),

                OpCode::Class8 		=> result = self.opcode_class8(),
                OpCode::Class16 	=> result = self.opcode_class16(),
                OpCode::Class32 	=> result = self.opcode_class32(),
                OpCode::Method8 	=> result = self.opcode_method8(),
                OpCode::Method16 	=> result = self.opcode_method16(),
                OpCode::Method32 	=> result = self.opcode_method32(),

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
                OpCode::Call 		=> result = self.opcode_call(),

                OpCode::Pop 		=> result = self.opcode_pop(),
                OpCode::PopN 		=> result = self.opcode_popn(),
                OpCode::CloseUpvalue	=> result = self.opcode_closeupvalue(),

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
                        self.callframe().closure_ref().function_ref()
                    );
                    return -1;
                }
            }
        }
    }
    
    pub fn callframe_mut(&mut self) -> &mut CallFrame {
        return self.callframes.last_mut().unwrap();
    }

    pub fn callframe(&self) -> &CallFrame {
        return self.callframes.last().unwrap();
    }

    fn opcode_call(&mut self) -> Result<(), String> {
        let arg_count = self.callframe_mut().read_byte();
        // TODO: Needs a return value
        let callee = self.peek(arg_count as usize).clone();
        self.call_value(callee, arg_count);        
        Ok(())
    }

    fn opcode_debug(&mut self) -> Result<(), String> {
        let value = self.pop();
        println!("DEBUG> {:?}", value);
        Ok(())
    }

    fn opcode_print(&mut self) -> Result<(), String> {
        let value = self.pop();
        println!("PRINT> {}", value);
        Ok(())
    }

    fn opcode_getconst(&mut self, id: usize) -> Result<(), String> {
        //let value = self.constants.value_by_id(id).clone();
        let value = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        self.push(value);
        Ok(())
    }
    
    fn opcode_getconst8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getconst(id);
    }
    
    fn opcode_getconst16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getconst(id);
    }
    
    fn opcode_getconst32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getconst(id);
    }

    fn opcode_getlocal(&mut self, id: usize) -> Result<(), String> {
        let depth = self.slot_depth(id);
        self.push(self.peek(depth).clone());
        Ok(())
    }

    fn opcode_getlocal8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getlocal(id);
    }
    
    fn opcode_getlocal16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getlocal(id);
    }
    
    fn opcode_getlocal32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getlocal(id);
    }

    fn opcode_getupvalue(&mut self, id: usize) -> Result<(), String> {
        println!("getupvalue id={} of closure upvalues", id);
        let stack_addr;
        let inner;
        
        // The following references must go out of scope before we
        // can manipulate the stack
        {        
            let callframe = self.callframe();
            let closure = callframe.closure_ref();
            let upvalue = closure.upvalue_ref_by_id(id);
            stack_addr = upvalue.addr();
            inner = upvalue.get();
        }

        // We have collected the necessary information about the upvalue
        match inner {
            Some(value)	=> {
                // This upvalue has been closed and now exists off the stack
                println!(" closed upvalue={}", value);
                self.push(value);
            }
            None => {
                // This value still lives on the stack
                let value = self.stack.peek_addr(stack_addr).clone();
                println!(" open upvalue={}", value);
                self.push(value);
            }
        }
        Ok(())
    }

    fn opcode_getupvalue8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getupvalue(id);
    }
    
    fn opcode_getupvalue16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getupvalue(id);
    }
    
    fn opcode_getupvalue32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getupvalue(id);
    }

    fn opcode_getglobal(&mut self, id: usize) -> Result<(), String> {
        // Compiler guarantees the variable is defined
        self.push(self.globals.value_by_id(id).unwrap().clone());
        Ok(())
    }

    fn opcode_getglobal8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getglobal16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getglobal32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getproperty(&mut self, id: usize) -> Result<(), String> {
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let field = constant.as_string();

        let instance = self.pop();// Value with field to be read

        if instance.is_instance() {
            let instance = instance.as_instance();
            if !instance.has(field) {
                return Err(format!("{} does not have a field \"{}\"", instance, field));
            }
            let value = instance.get(field).clone();
            self.push(value);
        } else {
            return Err(format!("{} does not have properties", instance).to_string());
        }
        Ok(())
    }

    fn opcode_getproperty8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getproperty(id);
    }
    
    fn opcode_getproperty16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getproperty(id);
    }
    
    fn opcode_getproperty32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getproperty(id);
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
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_defglobal16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_defglobal32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_setlocal(&mut self, id: usize) -> Result<(), String> {
        let depth = self.slot_depth(id);
        self.poke(self.peek(0).clone(), depth);
        Ok(())
    }
    
    fn opcode_setlocal8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setlocal(id);
    }
    
    fn opcode_setlocal16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setlocal(id);
    }
    
    fn opcode_setlocal32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setlocal(id);
    }

    fn opcode_setupvalue(&mut self, id: usize) -> Result<(), String> {
        //println!("setupvalue id={} of closure upvalues", id);
        let value = self.peek(0).clone();
        let stack_addr;
        
        // The following references must go out of scope before we
        // can manipulate the stack, so we do the checking inside here:
        {
            let callframe = self.callframe_mut();
            let mut closure = callframe.closure_mut();
            let upvalue = closure.upvalue_mut_by_id(id);
            // clox avoided this branch by using some pointer magic,
            // that's not an option for us.
            if upvalue.is_closed() {
                // Not sure if we will ever actually write to a
                // closed upvalue, but we can do so if needed.
                //println!("  upvalue already closed, update as {}", value);
                upvalue.close(value);
                return Ok(()); // Note: Early return
            } else {
                //println!("  upvalue still on the stack, update as {}", value);
                stack_addr = upvalue.addr();
                // Can't poke here because self is borrowed
            }
        }
        
        // We only get this far if the upvalue is not closed
        self.stack.poke_addr(value, stack_addr);
        Ok(())
    }

    fn opcode_setupvalue8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setupvalue(id);
    }
    
    fn opcode_setupvalue16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setupvalue(id);
    }
    
    fn opcode_setupvalue32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setupvalue(id);
    }

    fn opcode_setglobal(&mut self, id: usize) -> Result<(), String> {
        let value = self.peek(0).clone();
        self.globals.define_by_id(id, value);
        Ok(())
    }

    fn opcode_setglobal8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setglobal(id);
    }
    
    fn opcode_setglobal16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setglobal(id);
    }
    
    fn opcode_setglobal32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setglobal(id);
    }

    fn opcode_setproperty(&mut self, id: usize) -> Result<(), String> {
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let field = constant.as_string();

        let value = self.pop(); // Value to assign
        let mut instance = self.pop();// Value with field to be written

        if instance.is_instance() {
            let mut instance = instance.as_instance_mut();
            instance.set(field, value.clone());
            self.push(value);
        } else {
            return Err(format!("{} does not have properties", instance).to_string());
        }
        Ok(())
    }

    fn opcode_setproperty8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setproperty(id);
    }
    
    fn opcode_setproperty16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setproperty(id);
    }
    
    fn opcode_setproperty32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setproperty(id);
    }
    
    fn opcode_capture(&mut self, id: usize) -> Result<(), String> {
        // Get the function from constants table
        let value = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let upvalue_count = value.as_function().upvalue_count();
        // Wrap it in a closure
        let mut closure = Closure::new(value);
        
        // This opcode is followed by one variable length entry per upvalue
        for _i in 0..upvalue_count {
            //println!("VM capturing upvalue {} of {}", _i, upvalue_count);
        
            // Decode is_local and id
            // Because we allow more than 255 upvalues and local variables,
            // we have to encode the length if each id. This means the code 
            // to decode each entry is a bit more complicated than in clox.
            let byte = self.callframe_mut().read_byte();
            let is_local = if (byte & 128) == 128 { true } else { false };
            let id_len = byte & 127; // 1=byte, 2=word, 4=dword
            let mut id: usize = 0;
            match id_len {
                1 => id = self.callframe_mut().read_byte() as usize,
                2 => id = self.callframe_mut().read_word() as usize,
                4 => id = self.callframe_mut().read_dword() as usize,
                _ => {}
            }
            // Capture upvalue and insert into closure
            //println!("  id={} is_local={}", id, is_local);
            if is_local {
                closure.add_upvalue(self.capture_upvalue(self.callframe().stack_bottom() + id));
            } else {
                closure.add_upvalue(self.callframe().closure_ref().upvalue_ref_by_id(id).clone());
            }
        }
        
        // Push the closure onto the stack
        self.push(Value::closure(closure));
        Ok(())
    }

    fn opcode_capture8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_capture(id);
    }
    
    fn opcode_capture16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_capture(id);
    }
    
    fn opcode_capture32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_capture(id);
    }

    fn opcode_class(&mut self, id: usize) -> Result<(), String> {
        //println!("opcode_class({}) lookup constant", id);
        let name = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        //println!("create class");
        let class = Class::new(name.as_string());
        //println!("create value and push it");
        self.push(Value::class(class));
        Ok(())
    }

    fn opcode_class8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_class(id);
    }
    
    fn opcode_class16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_class(id);
    }
    
    fn opcode_class32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_class(id);
    }

    fn opcode_method(&mut self, id: usize) -> Result<(), String> {
        println!("opcode_method({}) lookup constant", id);
        let method_name = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).as_string().clone();
        let method_value = self.pop();
        let mut class_value = self.peek(0).clone();
        println!("opcode_method: class={} method={} set={}", class_value, method_name, method_value);
        class_value.as_class_mut().set(&method_name, method_value);
        Ok(())
    }

    fn opcode_method8(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_method(id);
    }
    
    fn opcode_method16(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_method(id);
    }
    
    fn opcode_method32(&mut self) -> Result<(), String> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_method(id);
    }

    fn opcode_not(&mut self) -> Result<(), String> {
        let value = self.pop();
        self.push(Value::boolean(!value.is_truthy()));
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
        let ip = self.callframe_mut().read_dword();
        self.callframe_mut().jmp(ip);
        Ok(())
    }
    
    fn opcode_jmpfalsep(&mut self) -> Result<(), String> {
        let ip = self.callframe_mut().read_dword();
        if !self.pop().is_truthy() { self.callframe_mut().jmp(ip); }
        Ok(())
    }

    fn opcode_jmpfalseq(&mut self) -> Result<(), String> {
        let ip = self.callframe_mut().read_dword();
        if !self.peek(0).is_truthy() { self.callframe_mut().jmp(ip); }
        Ok(())
    }

    fn opcode_pop(&mut self) -> Result<(), String> {
        let value = self.pop();
        println!("POP = {}", value);
        Ok(())
    }
    
    fn opcode_popn(&mut self) -> Result<(), String> {
        let count = self.callframe_mut().read_byte();
        for _ in 0..count {
            let value = self.pop();
            println!("POP = {}", value);
        }
        Ok(())
    }
    
    fn opcode_closeupvalue(&mut self) -> Result<(), String> {
        self.close_upvalues(self.stack.top());
        Err("OpCode::CloseUpvalue not yet implemented.".to_string())
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

    fn slot_depth(&self, slot: usize) -> usize {
        return self.stack.size()
            - self.callframe().stack_bottom()
            - 1
            - slot;
    }

    fn setup_initial_callframe(&mut self, function: Function) -> Result<(), String>{
        let closure = Closure::new(Value::function(function));
        let value = Value::closure(closure);
        self.push(value.clone());
        self.call_value(value, 0); // Main function takes zero arguments
        return Ok(());
    }

    fn call(&mut self, callee: Value, argc: u8) {
        let want_argc = callee.as_closure().function_ref().arity();
        if argc != want_argc {
            // TODO: Proper error handling
            panic!("Expected {} arguments but got {}", want_argc, argc);
        }

        let stack_bottom = self.stack.size() - (argc as usize) - 1;
        let callframe = CallFrame::new(callee, stack_bottom);
        self.callframes.push(callframe);
    }
    
    fn call_value(&mut self, value: Value, argc: u8) {
        if value.is_closure() {
            self.call(value, argc);            
        } else if value.is_class() {
            let instance = Value::instance(Instance::new(value));
            // callee is on the stack, but may have arguments after it
            // so we can't pop/push. 
            // Fortunately, we know exactly how deep it is.
            self.poke(instance, argc as usize);
            // handle constructor arguments, if any
            
        } else {
            panic!("VM.call_value({}, {}) not implemented.", value, argc);
        }
    }
}



#[allow(dead_code)]
impl VM {
    // Capture a value on the stack by noting its (absolute) position 
    // on the stack, but do not copy the value yet. 
    // Internally, the runtime Upvalue object does this by creating a
    // Rc<RefCell<Option<Value>>> containing None
    // Open upvalues are kept in self.open_upvalues, a Vec<Upvalue>
    fn capture_upvalue(&mut self, stack_addr: usize) -> Upvalue<Value> {
        //println!("VM.capture_upvalue() stack_addr={} stack={:?}", stack_addr, self.stack);
    
        // If slot is already captured, return the upvalue
        // Not exactly sure why, but I think this is because there may 
        // be multiple closures between the current one and the one
        // where the actual variable lives. Through the RefCell,
        // they will all refer to the same state (None=open/Some=closed)
        // and value (if closed). I think.
        for upvalue in &self.open_upvalues {
            if upvalue.addr() == stack_addr { return upvalue.clone(); }
        }
        
        //let index = self.open_upvalues.len();
        //println!("VM.capture_upvalue() capturing as index={} of open_upvalues", index);

        let upvalue = Upvalue::new(stack_addr);
        self.open_upvalues.push(upvalue);
        return self.open_upvalues.last().unwrap().clone();
    }

    // When a captured value is about to get removed from the stack
    // (go out of scope) we lift the value off the stack and into the
    // upvalue object.
    // stack_addr will either point to the top of the stack 
    // or to the bottom of the stack in the current call frame
    fn close_upvalues(&mut self, stack_addr: usize) {
        //println!("VM.close_upvalues() stack_addr={} stack={:?}", stack_addr, self.stack);
        loop {
            match self.open_upvalues.last_mut() {
                Some(upvalue) => {
                    //println!("  consider closing {}", upvalue);
                    
                    // Keep going while last_slot >= upvalue.slot()             
                    // in clox, slot is a pointer to the stack, 
                    // while here it's an index relative to the callframe
                    
                    // Stop if we find an upvalue object referencing a stack address
                    // below stack_addr
                    if upvalue.addr() < stack_addr { 
                        println!("  upvalue.addr() < stack_addr, exiting");
                        return; 
                    }
                    
                    // Close this upvalue by copying the value off the stack
                    // and storing it inside the upvalue object
                    let value = self.stack.peek_addr(upvalue.addr()).clone();
                    upvalue.close(value);
                    
                    // We must now remove this upvalue from self.open_upvalues
                    // but we can't do this while still borrowing a reference to it...
                }
                None => {
                    //println!("  no more open upvalues, exiting.");
                    return;
                }
            }
            //println!("  closed {}", self.open_upvalues.last().unwrap());
            
            // We're no longer borrowing a reference to self.open_upvalues
            // so we can safely pop the one we just closed.
            self.open_upvalues.pop();

            //println!("  {} upvalue(s) still open", self.open_upvalues.len());
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        //println!("VM.drop()");
    }
}


