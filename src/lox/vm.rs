
#[cfg(test)]
mod test;

mod runtime;
pub use runtime::{Class, Instance, Method, Upvalue};


use super::keyword::*;
use super::ByteCode;
use super::callframe::CallFrame;
use super::stack::Stack;
use super::value::Value;
use super::globals::Globals;
use super::closure::Closure;
use crate::lox::{RuntimeError, r_error};
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
    pub fn execute(&mut self, bytecode: &ByteCode) -> Result<i32, RuntimeError> {
        println!("VM.execute()");
        self.initialize(&bytecode)?;
        
        loop {
            let ip = self.callframe().ip();
            let fn_name = self.callframe().closure_ref().function_ref().name().to_string();

            let opcode = self.callframe_mut().read_op();

            // Trace VM state
            println!("IP={}:0x{:04x} SP=0x{:04x} CF=0x{:04x} Next={} ", fn_name, ip, self.stack.size(), self.callframe().stack_bottom(), opcode.mnemonic());
            println!(" stack={:?}", self.stack);
            
            let result = match opcode {
                OpCode::Exit		=> {
                    let return_value = self.pop();
                    //println!("OpCode::Exit, close_upvalues");
                    // Execute may be called again so be sure to close any open upvalues
                    //self.close_upvalues(self.callframe().stack_bottom());
                    // Rather than wasting time unwinding the stacks,
                    // simply discard them because the script is terminating.
                    self.stack = Stack::new();
                    self.callframes = vec![];
                    match return_value {
                        Value::Number(n) => {
                            if n.is_nan() { return Ok(i32::MAX) }
                            if n.is_infinite() && n.is_sign_positive() { return Ok(i32::MAX) }
                            if n.is_infinite() && n.is_sign_negative() { return Ok(i32::MIN) }
                            return Ok(n as i32)
                        }
                        Value::Bool(true) => return Ok(1),
                        _ => return Ok(0),
                    }
                }
                OpCode::Return 		=> {
                    let return_value = self.pop();
                    //println!("OpCode::Return, close_upvalues");
                    self.close_upvalues(self.callframe().stack_bottom());
                    self.callframes.pop();
                    if self.callframes.len() == 0 { 
                        r_error!(format!("Can not 'return' from top-level code, use 'exit' instead."))
                    }
                    
                    self.push(return_value);
                    Ok(())
                }
                OpCode::Debug		=> self.opcode_debug(),
                OpCode::Print		=> self.opcode_print(),

                OpCode::GetConst8 	=> self.opcode_getconst8(),
                OpCode::GetConst16 	=> self.opcode_getconst16(),
                OpCode::GetConst32 	=> self.opcode_getconst32(),
                OpCode::False 		=> self.opcode_literal(Value::Bool(false)),
                OpCode::Null 		=> self.opcode_literal(Value::Null),
                OpCode::True	 	=> self.opcode_literal(Value::Bool(true)),
                OpCode::NaN	 	    => self.opcode_literal(Value::Number(f64::NAN)),
                OpCode::Inf	 	    => self.opcode_literal(Value::Number(f64::INFINITY)),

                OpCode::GetLocal8 	=> self.opcode_getlocal8(),
                OpCode::GetLocal16 	=> self.opcode_getlocal16(),
                OpCode::GetLocal32 	=> self.opcode_getlocal32(),
                OpCode::GetUpvalue8 	=> self.opcode_getupvalue8(),
                OpCode::GetUpvalue16 	=> self.opcode_getupvalue16(),
                OpCode::GetUpvalue32 	=> self.opcode_getupvalue32(),
                OpCode::GetGlobal8 	=> self.opcode_getglobal8(),
                OpCode::GetGlobal16 	=> self.opcode_getglobal16(),
                OpCode::GetGlobal32 	=> self.opcode_getglobal32(),
                OpCode::GetProperty8 	=> self.opcode_getproperty8(),
                OpCode::GetProperty16 	=> self.opcode_getproperty16(),
                OpCode::GetProperty32 	=> self.opcode_getproperty32(),
                OpCode::GetSuper8 	=> self.opcode_getsuper8(),
                OpCode::GetSuper16 	=> self.opcode_getsuper16(),
                OpCode::GetSuper32 	=> self.opcode_getsuper32(),

                OpCode::DefGlobal8	=> self.opcode_defglobal8(),
                OpCode::DefGlobal16 	=> self.opcode_defglobal16(),
                OpCode::DefGlobal32 	=> self.opcode_defglobal32(),

                OpCode::SetLocal8 	=> self.opcode_setlocal8(),
                OpCode::SetLocal16 	=> self.opcode_setlocal16(),
                OpCode::SetLocal32 	=> self.opcode_setlocal32(),
                OpCode::SetUpvalue8 	=> self.opcode_setupvalue8(),
                OpCode::SetUpvalue16 	=> self.opcode_setupvalue16(),
                OpCode::SetUpvalue32 	=> self.opcode_setupvalue32(),
                OpCode::SetGlobal8 	=> self.opcode_setglobal8(),
                OpCode::SetGlobal16 	=> self.opcode_setglobal16(),
                OpCode::SetGlobal32 	=> self.opcode_setglobal32(),
                OpCode::SetProperty8 	=> self.opcode_setproperty8(),
                OpCode::SetProperty16 	=> self.opcode_setproperty16(),
                OpCode::SetProperty32 	=> self.opcode_setproperty32(),

                OpCode::Capture8 	=> self.opcode_capture8(),
                OpCode::Capture16 	=> self.opcode_capture16(),
                OpCode::Capture32 	=> self.opcode_capture32(),

                OpCode::Class8 		=> self.opcode_class8(),
                OpCode::Class16 	=> self.opcode_class16(),
                OpCode::Class32 	=> self.opcode_class32(),
                OpCode::Method8 	=> self.opcode_method8(),
                OpCode::Method16 	=> self.opcode_method16(),
                OpCode::Method32 	=> self.opcode_method32(),

                OpCode::Not 		=> self.opcode_not(),
                OpCode::Negate 		=> self.opcode_negate(),

                OpCode::Add 		=> self.opcode_add(),
                OpCode::Sub 		=> self.opcode_sub(),
                OpCode::Mul 		=> self.opcode_mul(),
                OpCode::Div 		=> self.opcode_div(),
                OpCode::Mod 		=> self.opcode_mod(),
                OpCode::Equal		=> self.opcode_equal(),
                OpCode::NotEqual	=> self.opcode_notequal(),
                OpCode::Less		=> self.opcode_less(),
                OpCode::Greater		=> self.opcode_greater(),
                OpCode::LessEqual	=> self.opcode_lessequal(),
                OpCode::GreaterEqual	=> self.opcode_greaterequal(),
                OpCode::Same		=> self.opcode_same(),

                OpCode::Jmp 		=> self.opcode_jmp(),
                OpCode::JmpFalseP	=> self.opcode_jmpfalsep(),
                OpCode::JmpFalseQ	=> self.opcode_jmpfalseq(),
                OpCode::Call 		=> self.opcode_call(),

                OpCode::Pop 		=> self.opcode_pop(),
                OpCode::PopN 		=> self.opcode_popn(),
                OpCode::CloseUpvalue	=> self.opcode_closeupvalue(),
                OpCode::Inherit	=> self.opcode_inherit(),

                OpCode::BAD 		=> self.opcode_bad(),
            };
            
            // On error, dump message and return
            if let Err(mut runtime_error) = result {
                eprintln!(
                    "{} at ip={:#06x}\n{:?}", 
                    runtime_error.get_message(),
                    ip, 
                    self.callframe().closure_ref().function_ref()
                );
                runtime_error.set_stack_trace(vec!["-- stacktrace goes here --".to_string()]);
                return Err(runtime_error);
            }
        }
    }
    
    pub fn callframe_mut(&mut self) -> &mut CallFrame {
        return self.callframes.last_mut().unwrap();
    }

    pub fn callframe(&self) -> &CallFrame {
        return self.callframes.last().unwrap();
    }

    fn opcode_call(&mut self) -> Result<(), RuntimeError> {
        let arg_count = self.callframe_mut().read_byte();
        let callee = self.peek(arg_count as usize).clone();
        return self.call_value(callee, arg_count);        
    }

    fn opcode_debug(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop();
        println!("DEBUG> {:?}", value);
        Ok(())
    }

    fn opcode_print(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop();
        println!("PRINT> {}", value);
        Ok(())
    }

    fn opcode_getconst(&mut self, id: usize) -> Result<(), RuntimeError> {
        //let value = self.constants.value_by_id(id).clone();
        let value = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        //println!("opcode_getconst() loaded constant id=0x{:08x} onto stack: {}", id, value);
        self.push(value);
        Ok(())
    }
    
    fn opcode_getconst8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getconst(id);
    }
    
    fn opcode_getconst16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getconst(id);
    }
    
    fn opcode_getconst32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getconst(id);
    }

    fn opcode_getlocal(&mut self, id: usize) -> Result<(), RuntimeError> {
        let depth = self.slot_depth(id); // Stack index from bottom
        self.push(self.peek(depth).clone());
        //println!("opcode_getlocal() fetched local variable id 0x{:08x} onto stack: {}", id, self.peek(0));
        Ok(())
    }

    fn opcode_getlocal8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getlocal(id);
    }
    
    fn opcode_getlocal16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getlocal(id);
    }
    
    fn opcode_getlocal32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getlocal(id);
    }

    fn opcode_getupvalue(&mut self, id: usize) -> Result<(), RuntimeError> {
        //print!("opcode_getupvalue() id 0x{:08x}:", id);
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
                //println!(" closed upvalue={}", value);
                self.push(value);
            }
            None => {
                // This value still lives on the stack
                let value = self.stack.peek_addr(stack_addr).clone();
                //println!(" open upvalue={}", value);
                self.push(value);
            }
        }
        Ok(())
    }

    fn opcode_getupvalue8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getupvalue(id);
    }
    
    fn opcode_getupvalue16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getupvalue(id);
    }
    
    fn opcode_getupvalue32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getupvalue(id);
    }

    fn opcode_getglobal(&mut self, id: usize) -> Result<(), RuntimeError> {
        // Compiler guarantees the variable is defined
        self.push(self.globals.value_by_id(id).unwrap().clone());
        //println!("opcode_getglobal() loaded global 0x{:08x} onto stack: {}", id, self.globals.value_by_id(id).unwrap());
        Ok(())
    }

    fn opcode_getglobal8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getglobal16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getglobal32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getglobal(id);
    }
    
    fn opcode_getproperty(&mut self, id: usize) -> Result<(), RuntimeError> {
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let name = constant.as_string();

        let instance = self.peek(0).clone();	// Receiver Value

        if instance.is_instance() {
            if let Some(value) = instance.as_instance().get(&name) {
                self.pop();
                self.push(value.clone());
//            if self.get_instance_field(&instance, &name) {
                // Name is a field of this instance
                //println!("opcode_getproperty() Field '{}' of {} pushed onto stack", name, instance);
                return Ok(())
            } else {
                //println!("opcode_getproperty() Method '{}' of {} pushed onto stack", name, instance);
                return self.bind_method(&instance.as_instance().class(), &name);
            }
        } else {
            r_error!(format!("{} does not have properties to get", instance))
        }
    }

    fn opcode_getproperty8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getproperty(id);
    }
    
    fn opcode_getproperty16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getproperty(id);
    }
    
    fn opcode_getproperty32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getproperty(id);
    }
    
    fn opcode_getsuper(&mut self, id: usize) -> Result<(), RuntimeError> {
        //println!("opcode_getsuper() invoked");
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let method_name = constant.as_string();

        let superclass = self.pop();
        //println!("opcode_getsuper() binding method '{}' to superclass {}", method_name, superclass);

        if self.bind_method(&superclass, method_name).is_err() {
            r_error!(format!("Could not bind method '{}' to superclass {}", method_name, superclass))
        }
        //println!("opcode_getsuper() finished");
        Ok(())
    }

    fn opcode_getsuper8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_getsuper(id);
    }
    
    fn opcode_getsuper16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_getsuper(id);
    }
    
    fn opcode_getsuper32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_getsuper(id);
    }
    
    fn opcode_literal(&mut self, value: Value) -> Result<(), RuntimeError> {
        self.push(value);
        Ok(())
    }
    
    fn opcode_defglobal(&mut self, id: usize) -> Result<(), RuntimeError> {
        let value = self.pop();
        //println!("opcode_defglobal() popped {} off stack, define as global 0x{:08x}", value, id);
        self.globals.define_by_id(id, value);
        Ok(())
    }
    
    fn opcode_defglobal8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_defglobal16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_defglobal32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_defglobal(id);        
    }
    
    fn opcode_setlocal(&mut self, id: usize) -> Result<(), RuntimeError> {
        let depth = self.slot_depth(id); // Stack index from bottom
        self.poke(self.peek(0).clone(), depth);
        //println!("opcode_setlocal() stored top of stack in local variable id 0x{:08x}: {}", id, self.peek(0));
        Ok(())
    }
    
    fn opcode_setlocal8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setlocal(id);
    }
    
    fn opcode_setlocal16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setlocal(id);
    }
    
    fn opcode_setlocal32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setlocal(id);
    }

    fn opcode_setupvalue(&mut self, id: usize) -> Result<(), RuntimeError> {
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

    fn opcode_setupvalue8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setupvalue(id);
    }
    
    fn opcode_setupvalue16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setupvalue(id);
    }
    
    fn opcode_setupvalue32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setupvalue(id);
    }

    fn opcode_setglobal(&mut self, id: usize) -> Result<(), RuntimeError> {
        let value = self.peek(0).clone();
        //println!("opcode_setglobal() set id 0x{:08x} as {}", id, value);
        self.globals.define_by_id(id, value);
        Ok(())
    }

    fn opcode_setglobal8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setglobal(id);
    }
    
    fn opcode_setglobal16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setglobal(id);
    }
    
    fn opcode_setglobal32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setglobal(id);
    }

    fn opcode_setproperty(&mut self, id: usize) -> Result<(), RuntimeError> {
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let field = constant.as_string();

        let value = self.pop(); // Value to assign
        let mut instance = self.pop();// Value with field to be written

        if instance.is_instance() {
            let mut instance = instance.as_instance_mut();
            instance.set(field, value.clone());
            //println!("opcode_setproperty() set field '{}' of {} to {}", field, instance, value);
            self.push(value);
        } else {
            r_error!(format!("{} does not have properties to set", instance))
        }
        Ok(())
    }

    fn opcode_setproperty8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_setproperty(id);
    }
    
    fn opcode_setproperty16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_setproperty(id);
    }
    
    fn opcode_setproperty32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_setproperty(id);
    }
    
    fn opcode_capture(&mut self, id: usize) -> Result<(), RuntimeError> {
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

    fn opcode_capture8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_capture(id);
    }
    
    fn opcode_capture16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_capture(id);
    }
    
    fn opcode_capture32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_capture(id);
    }

    fn opcode_class(&mut self, id: usize) -> Result<(), RuntimeError> {
        //println!("opcode_class({}) lookup constant", id);
        let name = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        //println!("create class");
        let class = Class::new(name.as_string());
        //println!("create value and push it");
        //println!("opcode_class() Spawned class {} using constant 0x{:08x}: {}", class, id, name);
        self.push(Value::class(class));
        Ok(())
    }

    fn opcode_class8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_class(id);
    }
    
    fn opcode_class16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_class(id);
    }
    
    fn opcode_class32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_class(id);
    }

    fn opcode_method(&mut self, id: usize) -> Result<(), RuntimeError> {
        let method_name = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).as_string().clone();
        let method_value = self.pop();
        let mut class_value = self.peek(0).clone();
        //println!("opcode_method() popped {} off stack, added as method '{}' of {}", method_value, method_name, class_value);
        class_value.as_class_mut().set(&method_name, method_value);
        Ok(())
    }

    fn opcode_method8(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_byte() as usize;
        return self.opcode_method(id);
    }
    
    fn opcode_method16(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_word() as usize;
        return self.opcode_method(id);
    }
    
    fn opcode_method32(&mut self) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_dword() as usize;
        return self.opcode_method(id);
    }

    fn opcode_not(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop();
        match value {
            _ => self.push(Value::boolean(!value.is_truthy())),
        }
        Ok(())
    }
    
    fn opcode_negate(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop();
        match value {
            //Value::Inf => self.push(Value::InfN),
            //Value::InfN => self.push(Value::Inf),
            Value::Bool(b) => self.push(Value::boolean(!b)),
            Value::Number(n) => self.push(Value::number(-n)),
            _ => self.push(Value::Null),
        }
        Ok(())
    }
    
    fn opcode_add(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.add(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_sub(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.subtract(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_mul(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.multiply(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_div(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.divide(&b);
        match res {
            Ok(value) => { self.push(value); } // Division by zero = f64::INFINITY
            Err(_) => { self.push(Value::Number(f64::NAN)); } // Bad operands
        }
        Ok(())
    }
    
    fn opcode_mod(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.modulo(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(_) => { self.push(Value::number(f64::NAN)); }
        }
        Ok(())
    }
    
    fn opcode_equal(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a == b));
        Ok(())
    }
    
    fn opcode_same(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a.is(&b)));
        Ok(())
    }

    fn opcode_notequal(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a != b));
        Ok(())
    }
    
    fn opcode_less(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a < b));
        Ok(())
    }
    
    fn opcode_greater(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a > b));
        Ok(())
    }
    
    fn opcode_lessequal(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a <= b));
        Ok(())
    }
    
    fn opcode_greaterequal(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::boolean(a >= b));
        Ok(())
    }

    fn opcode_jmp(&mut self) -> Result<(), RuntimeError> {
        let ip = self.callframe_mut().read_dword();
        self.callframe_mut().jmp(ip);
        Ok(())
    }
    
    fn opcode_jmpfalsep(&mut self) -> Result<(), RuntimeError> {
        let ip = self.callframe_mut().read_dword();
        if !self.pop().is_truthy() { self.callframe_mut().jmp(ip); }
        Ok(())
    }

    fn opcode_jmpfalseq(&mut self) -> Result<(), RuntimeError> {
        let ip = self.callframe_mut().read_dword();
        if !self.peek(0).is_truthy() { self.callframe_mut().jmp(ip); }
        Ok(())
    }

    fn opcode_pop(&mut self) -> Result<(), RuntimeError> {
        let _value = self.pop();
        //println!("POP = {}", value);
        Ok(())
    }
    
    fn opcode_popn(&mut self) -> Result<(), RuntimeError> {
        let count = self.callframe_mut().read_byte();
        for _ in 0..count {
            let _value = self.pop();
            //println!("POP = {}", value);
        }
        Ok(())
    }
    
    fn opcode_closeupvalue(&mut self) -> Result<(), RuntimeError> {
        self.close_upvalues(self.stack.top());
        Ok(())
    }
    
    fn opcode_inherit(&mut self) -> Result<(), RuntimeError> {
        let mut class = self.pop();
        let superclass = self.pop();
        if !superclass.is_class() {
            r_error!(format!("Can not inherit from {} because it is not a class", superclass))
        }
        // Copy parent methods
        // Compiler emits INHRT before any MTHD so we know the method table is empty
        *class.as_class_mut().methods_mut() = superclass.as_class().methods().clone();
        self.push(class);
        Ok(())
    }
    
    fn opcode_bad(&mut self) -> Result<(), RuntimeError> {
        r_error!(format!("Bad OpCode! INTERNAL ERROR in VM and/or bytecode compiler!"))
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


    fn initialize(&mut self, bytecode: &ByteCode) -> Result<(), RuntimeError> {
        self.callframes.clear();
        self.stack.clear();
        self.open_upvalues.clear();
        self.globals = bytecode.globals().clone();
        let closure = Closure::new(Value::function(bytecode.main().clone()));
        let value = Value::closure(closure);
        self.push(value.clone());
        return self.call_value(value, 0); // Main function takes zero arguments
    }

    fn call(&mut self, callee: Value, argc: u8) -> Result<(), RuntimeError> {
        let want_argc = callee.as_closure().function_ref().arity();
        if argc != want_argc {
            r_error!(format!("Expected {} argument(s) but got {}", want_argc, argc)) 
        }

        let stack_bottom = self.stack.size() - (argc as usize) - 1;
        let callframe = CallFrame::new(callee, stack_bottom);
        self.callframes.push(callframe);
        Ok(())
    }
    
    fn call_value(&mut self, value: Value, argc: u8) -> Result<(), RuntimeError> {
        if value.is_closure() {
            self.call(value, argc)?;
        } else if value.is_method() {
            let bound = value.as_method();
            self.stack.poke(bound.receiver().clone(), argc as usize);
            self.call(bound.method().clone(), argc)?;
        } else if value.is_class() {
            let initializer = match value.as_class().get(KEYWORD_INIT) {
                None => None,
                Some(function) => Some(function.clone()),
            };
            let instance = Value::instance(Instance::new(value));
            // callee is on the stack, but may have arguments after it
            // so we can't pop/push. 
            // Fortunately, we know exactly how deep it is.
            self.poke(instance, argc as usize);
            // handle constructor arguments, if any
            if let Some(function) = initializer {
                self.call(function, argc)?;
            } else if argc != 0 {
                r_error!(format!("Expected 0 arguments but got {}", argc))
            }
        } else {
            r_error!(format!("VM.call_value({}, {}) not implemented.", value, argc))
        }
        Ok(())
    }

        
    fn bind_method(&mut self, class: &Value, method_name: &str) -> Result<(), RuntimeError> {
        let receiver = self.stack.peek(0).clone();
        //println!("bind_method() invoked, class={} method={} receiver={}", class, method_name, receiver);
        if !class.is_class() {
            r_error!(format!("Can not bind '{}' to {} as {} because it is not a class", method_name, receiver, class))
        }
        if !receiver.is_instance() {
            r_error!(format!("Can not bind '{}' to {} because it is not an instance", method_name, receiver))
        }
        // clox looks up the class by name, 
        // but the receiver already has a reference to its class.
        //let instance = receiver_value.as_instance();
        //println!("bind_method() instance={}", instance);
        //let class = instance.class().as_class();
        
        //let result = class.as_class().get(method_name);
        match class.as_class().get(method_name) {
            Some(method_value) => {
                let bound_method = Method::new(receiver, method_value.clone());        
                self.push(Value::method(bound_method));
                return Ok(());
            }
            None => {
                r_error!(format!("Class {} does not have a method named '{}'", class, method_name))
            }
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
                        //println!("  upvalue.addr() < stack_addr, exiting");
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


