

use std::borrow::BorrowMut;


use log::{trace, debug, warn};


use super::CallFrame;
use super::runtime::NativeMethod;
use crate::lox::common::keyword::*;
use crate::lox::common::ByteCode;
use super::Stack;
use crate::lox::common::{Array, Value, ValueIterator, NativeCallables};
use crate::lox::common::Globals;
use crate::lox::common::Closure;
use super::{RuntimeError, r_error};
use crate::lox::common::OpCode;
use super::Upvalue;
use super::{Class, Instance, Method};


pub struct VM {
    callframes: Vec<CallFrame>,
    stack: Stack<Value>,
    globals: Globals<Value>,
    open_upvalues: Vec<Upvalue<Value>>, // Note: Runtime representation
    native_callables: NativeCallables,
}


impl VM {
    pub fn new() -> VM {
        VM {
            callframes: 	vec![],
            stack: 		Stack::new(), 
            globals:		Globals::new(),
            open_upvalues:	vec![],
            native_callables: NativeCallables::new(),
        }
    }

    pub fn native_callables(&mut self) -> &mut NativeCallables {
        return &mut self.native_callables;
    }
    
}


impl VM {
    pub fn execute(&mut self, bytecode: &ByteCode) -> Result<i32, RuntimeError> {
        trace!("initialize");
        self.initialize(&bytecode)?;
        
        loop {
            let ip = self.callframe().ip();
            let fn_name = self.callframe().closure_ref().function_ref().name().to_string();

            let opcode = self.callframe_mut().read_op();

            // Trace VM state
            debug!("IP={}:0x{:04x} SP=0x{:04x} CF=0x{:04x} Next={} stack=", fn_name, ip, self.stack.size(), self.callframe().stack_bottom(), opcode.mnemonic());
            debug!("{:?}", self.stack);
            
            let result = match opcode {
                OpCode::Exit		    => return self.opcode_exit(),
                OpCode::Return 		    => self.opcode_return(),
                OpCode::Debug		    => self.opcode_debug(),
                OpCode::Print		    => self.opcode_print(),
                OpCode::Dup 		    => self.opcode_dup(),
                OpCode::MakeIter        => self.opcode_makeiter(),
                OpCode::ReadIter        => self.opcode_readiter(),
                OpCode::NextIter        => self.opcode_nextiter(),

                OpCode::GetConst8 	    |
                OpCode::GetConst16 	    |
                OpCode::GetConst32 	    => self.opcode_getconst(opcode.len()),
                OpCode::False 		    => self.opcode_literal(Value::Bool(false)),
                OpCode::Null 		    => self.opcode_literal(Value::Null),
                OpCode::True	 	    => self.opcode_literal(Value::Bool(true)),
                OpCode::NaN	 	        => self.opcode_literal(Value::Number(f64::NAN)),
                OpCode::Inf	 	        => self.opcode_literal(Value::Number(f64::INFINITY)),

                OpCode::GetLocal8 	    |
                OpCode::GetLocal16 	    |
                OpCode::GetLocal32 	    => self.opcode_getlocal(opcode.len()),
                OpCode::GetUpvalue8 	|
                OpCode::GetUpvalue16 	|
                OpCode::GetUpvalue32 	=> self.opcode_getupvalue(opcode.len()),
                OpCode::GetGlobal8 	    |
                OpCode::GetGlobal16 	|
                OpCode::GetGlobal32 	=> self.opcode_getglobal(opcode.len()),
                OpCode::GetProperty8 	|
                OpCode::GetProperty16 	|
                OpCode::GetProperty32 	=> self.opcode_getproperty(opcode.len()),
                OpCode::GetSuper8 	    |
                OpCode::GetSuper16 	    |
                OpCode::GetSuper32 	    => self.opcode_getsuper(opcode.len()),

                OpCode::DefGlobal8	    |
                OpCode::DefGlobal16 	|
                OpCode::DefGlobal32 	=> self.opcode_defglobal(opcode.len()),
                OpCode::DefArray8	    |
                OpCode::DefArray16 	    |
                OpCode::DefArray32 	    => self.opcode_defarray(opcode.len()),

                OpCode::SetLocal8 	    |
                OpCode::SetLocal16 	    |
                OpCode::SetLocal32 	    => self.opcode_setlocal(opcode.len()),
                OpCode::SetUpvalue8 	|
                OpCode::SetUpvalue16 	|
                OpCode::SetUpvalue32 	=> self.opcode_setupvalue(opcode.len()),
                OpCode::SetGlobal8 	    |
                OpCode::SetGlobal16 	|
                OpCode::SetGlobal32 	=> self.opcode_setglobal(opcode.len()),
                OpCode::SetProperty8 	|
                OpCode::SetProperty16 	|
                OpCode::SetProperty32 	=> self.opcode_setproperty(opcode.len()),

                OpCode::Capture8 	    |
                OpCode::Capture16 	    |
                OpCode::Capture32 	    => self.opcode_capture(opcode.len()),

                OpCode::Class8 		    |
                OpCode::Class16 	    |
                OpCode::Class32 	    => self.opcode_class(opcode.len()),
                OpCode::Method8 	    |
                OpCode::Method16 	    |
                OpCode::Method32 	    => self.opcode_method(opcode.len()),

                OpCode::Not 		    => self.opcode_not(),
                OpCode::Negate 		    => self.opcode_negate(),

                OpCode::Add 		    => self.opcode_add(),
                OpCode::Sub 		    => self.opcode_sub(),
                OpCode::Mul 		    => self.opcode_mul(),
                OpCode::Div 		    => self.opcode_div(),
                OpCode::Mod 		    => self.opcode_mod(),
                OpCode::Equal		    => self.opcode_equal(),
                OpCode::NotEqual	    => self.opcode_notequal(),
                OpCode::Less		    => self.opcode_less(),
                OpCode::Greater		    => self.opcode_greater(),
                OpCode::LessEqual	    => self.opcode_lessequal(),
                OpCode::GreaterEqual	=> self.opcode_greaterequal(),
                OpCode::Same		    => self.opcode_same(),

                OpCode::Jmp 		    => self.opcode_jmp(opcode.len()),
                OpCode::JmpFalseP	    => self.opcode_jmpfalsep(opcode.len()),
                OpCode::JmpFalseQ	    => self.opcode_jmpfalseq(opcode.len()),
                OpCode::Call 		    => self.opcode_call(opcode.len()),

                OpCode::Pop 		    => self.opcode_pop(),
                OpCode::PopN 		    => self.opcode_popn(),
                OpCode::CloseUpvalue	=> self.opcode_closeupvalue(),
                OpCode::Inherit	        => self.opcode_inherit(),
                OpCode::GetSubscript    => self.opcode_getsubscript(),
                OpCode::SetSubscript    => self.opcode_setsubscript(),

                OpCode::BAD 		    => self.opcode_bad(),
            };
            
            // On error, dump message and return
            if let Err(mut runtime_error) = result {
                debug!(
                    "{} at ip={:#06x}\n{:?}", 
                    runtime_error.get_message(),
                    ip, 
                    self.callframe().closure_ref().function_ref()
                );
                runtime_error.set_stack_trace(self.stack_trace());
                return Err(runtime_error);
            }
        }
    }


    fn stack_trace(&self) -> Vec<String> {
        self.callframes.iter().map(|callframe| format!("{:?}", callframe)).collect()
    }

    
    pub fn callframe_mut(&mut self) -> &mut CallFrame {
        return self.callframes.last_mut().unwrap();
    }

    pub fn callframe(&self) -> &CallFrame {
        return self.callframes.last().unwrap();
    }


    fn opcode_exit(&mut self) -> Result<i32, RuntimeError> {
        match self.pop() {
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


    fn opcode_return(&mut self) -> Result<(), RuntimeError> {
        let return_value = self.pop();
        let bottom = self.callframe().stack_bottom();
        trace!("return statement: stack_bottom={}", bottom);
        self.close_upvalues(self.callframe().stack_bottom());
        self.callframes.pop();
        trace!("  stack height is now {}", self.stack.len());
        while self.stack.len() > bottom { self.pop(); } // Workaround hack to solve problem with receiver being left on the stack
        if self.callframes.len() == 0 { 
            // Note: The compiler should make this impossible but we're checking just in case
            r_error!(format!("Can not 'return' from top-level code, use 'exit' instead."))
        }
        
        self.push(return_value);
        Ok(())
    }


    fn opcode_call(&mut self, len: usize) -> Result<(), RuntimeError> {
        let arg_count = self.callframe_mut().read_bytes(len);
        let callee = self.peek(arg_count as usize).clone();
        return self.call_value(callee, arg_count as u8);        
    }

    fn opcode_debug(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop();
        println!("DEBUG> {:?}", value);
        Ok(())
    }

    fn opcode_dup(&mut self) -> Result<(), RuntimeError> {
        self.push(self.peek(0).clone());
        Ok(())
    }

    fn opcode_makeiter(&mut self) -> Result<(), RuntimeError> {
        match ValueIterator::new(self.pop()) {
            Err(msg) => r_error!(format!("{}", msg)),
            Ok(iter) => self.poke(Value::iterator(iter), 2),
        }
        trace!("opcode_makeiter() placed iterator={} in stack slot at depth=2", self.peek(2));
        Ok(())
    }

    fn opcode_readiter(&mut self) -> Result<(), RuntimeError> {
        trace!("opcode_readiter() stack={:?}", self.stack);
        let mut iter_value = self.peek(2).clone();
        let mut iter = iter_value.borrow_mut().as_iterator_mut();
        let (value, next) = iter.next();
        if next.is_some() { 
            self.poke(Value::Bool(true), 1);
            self.poke(next.unwrap().clone(), 0);
        } else {
            self.poke(Value::Bool(false), 1);
            self.poke(Value::Null, 0);
        }
        trace!("opcode_readiter() next={}", value);    
        if value.is_instance() {
            self.push(value.clone()); // receiver (=instance)
            trace!("load method '{}' of {} onto stack", KEYWORD_NEXT, value);
            self.bind_method(&value.as_instance().class(), KEYWORD_NEXT)?; // pops receiver, pushes bound method
            let method = self.pop();
            self.push(value.clone()); // receiver again so opcode_nextiter() can see it
            self.push(Value::Null); // Empty slot where call_value() will put the receiver
            self.push(next.unwrap().clone()); // Previous value or Value::Null if this is the first iteration
            trace!("call method={} of receiver={} with 1 argument: last={}", method, self.peek(1), self.peek(0));
            self.call_value(method, 1)?; 
        }
        Ok(())
    }

    fn opcode_nextiter(&mut self) -> Result<(), RuntimeError> {
        trace!("opcode_nextiter() stack={:?}", self.stack);
        if self.peek(1).is_instance() {
            // Clean up after method call
            let last = self.pop();
            let _instance = self.pop();
            self.poke(Value::Bool(!last.is(&Value::Null)), 1);
            self.poke(last, 0);
        }
        let mut iter_value = self.peek(2).clone();
        let mut iter = iter_value.borrow_mut().as_iterator_mut();
        trace!("opcode_nextiter() copying value={} back to iterator last()", self.peek(0));
        *iter.last() = self.peek(0).clone();
        Ok(())
    }

    fn opcode_print(&mut self) -> Result<(), RuntimeError> {
        let value = self.pop();
        println!("PRINT> {}", value);
        Ok(())
    }

    fn opcode_getconst(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        // When reading from the constant table, we must copy, not clone; constants are immutable
        let value = Value::from(self.callframe().closure_ref().function_ref().read_constants().value_by_id(id));
        trace!("loaded constant id=0x{:08x} onto stack: {}", id, value);
        self.push(value);
        Ok(())
    }


    fn opcode_getlocal(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        let depth = self.slot_depth(id); // Stack index from bottom
        self.push(self.peek(depth).clone());
        trace!("loaded local variable id=0x{:08x} onto stack: {}", id, self.peek(0));
        Ok(())
    }


    fn opcode_getupvalue(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
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
                trace!("closed upvalue={}", value);
                self.push(value);
            }
            None => {
                // This value still lives on the stack
                let value = self.stack.peek_addr(stack_addr).clone();
                trace!("open upvalue={}", value);
                self.push(value);
            }
        }
        Ok(())
    }


    fn opcode_getglobal(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        // Compiler guarantees the variable is defined
        self.push(self.globals.value_by_id(id).unwrap().clone());
        trace!("loaded global id=0x{:08x} onto stack: {}", id, self.globals.value_by_id(id).unwrap());
        Ok(())
    }


    fn opcode_getproperty(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let name = constant.as_string();

        let receiver = self.peek(0).clone();	// Receiver Value

        // Check the user-defined fields and methods (note that these may shadow any built-in ones)
        if receiver.is_instance() {
            if let Some(value) = receiver.as_instance().get(name.as_str()) {
                self.pop();
                self.push(value.clone());
                trace!("loaded field '{}' of {} onto stack", name, receiver);
                return Ok(())
            }
            if let Some(_) = receiver.as_instance().class().get(&constant) {
                trace!("loaded method '{}' of {} onto stack", name, receiver);
                return self.bind_method(&receiver.as_instance().class(), name.as_str());
            }
        }

        // If the name matches a built-in method, bind it and push it onto the stack
        if let Some(callable) = self.native_callables().get_method(name.as_str()).cloned() {
            return self.bind_native_method(callable); // The receiver is still on the stack
        }

        // Not found, return a RuntimeError
        r_error!(format!("{} does not have a method or field '{}'", receiver, name))
    }


    fn opcode_getsuper(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let method_name = constant.as_string();

        let superclass = self.pop();
        if self.bind_method(&superclass, method_name.as_str()).is_err() {
            r_error!(format!("Could not bind method '{}' to superclass {}", method_name, superclass))
        }
        trace!("bound method '{}' to superclass {}", method_name, superclass);
        Ok(())
    }


    fn opcode_literal(&mut self, value: Value) -> Result<(), RuntimeError> {
        self.push(value);
        Ok(())
    }
    
    fn opcode_defglobal(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        let value = self.pop();
        trace!("popped {} off stack, define as global id=0x{:08x}", value, id);
        self.globals.define_by_id(id, value);
        Ok(())
    }


    fn opcode_defarray(&mut self, len: usize) -> Result<(), RuntimeError> {
        let elements = self.callframe_mut().read_bytes(len) as usize;
        let array = Array::from(&self.stack.as_slice()[self.stack.len()-elements..]);
        self.stack.truncate(self.stack.len() - elements); // Drop elements from stack
        self.push(Value::array(array));
        trace!("popped {} value(s) off stack, defined array", elements);
        Ok(())
    }

    fn opcode_getsubscript(&mut self) -> Result<(), RuntimeError> {
        let keys = self.pop();
        let value = self.pop();
        if !value.can_get() { r_error!(format!("Can't subscript into value '{}'", value)) }
        let mut array = Array::new();
        for key in keys.as_array().as_slice().iter() {
            match value.get(key) {
                Some(element) => array.push(element.clone()),
                None => r_error!(format!("Bad subscript '{}' into value '{}'", key, value)),
            }
        }
        if array.len() == 0 {
            if !value.is_array() { r_error!(format!("Can not copy '{}' as array", value)) }
            trace!("copied array");
            self.push(Value::from(&value));
        } else if array.len() == 1 {
            trace!("copied single value from array");
            self.push(array.pop().unwrap());
        } else {
            trace!("copied multiple values into new array");
            self.push(Value::array(array));
        }
        Ok(())
    }


    fn opcode_setsubscript(&mut self) -> Result<(), RuntimeError> {
        let source = self.pop();
        let keys = self.pop(); // Subscript keys of the destination
        let mut destination = self.pop();
        if source.is_array() && source.as_array().len() != keys.as_array().len() {
            r_error!(format!("Length of source ({}) does not match the destination", source.as_array().len()))
        }
        if !destination.can_set() { r_error!(format!("Can not subscript into value '{}'", destination)) }

        for (i, key) in keys.as_array().as_slice().iter().enumerate() {
            trace!("setsubscript source={} key={} destination={}", source, key, destination);
            let value = if source.is_array() { 
                source.as_array().get(i).unwrap().clone() 
            } else { 
                source.clone() 
            };
            if let Err(msg) = destination.set(key, value) {
                r_error!(format!("{}", msg));
            }
        }
        Ok(())
    }


    fn opcode_setlocal(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        let depth = self.slot_depth(id); // Stack index from bottom
        self.poke(self.peek(0).clone(), depth);
        trace!("value copied to local variable id=0x{:08x}: {}", id, self.peek(0));
        Ok(())
    }


    fn opcode_setupvalue(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        trace!("setupvalue id={} of closure upvalues", id);
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
                warn!("write to already closed upvalue, update as {}", value);
                upvalue.close(value);
                return Ok(()); // Note: Early return
            } else {
                trace!("upvalue still on the stack, update as {}", value);
                stack_addr = upvalue.addr();
                // Can't poke here because self is borrowed
            }
        }
        
        // We only get this far if the upvalue is not closed
        self.stack.poke_addr(value, stack_addr);
        Ok(())
    }


    fn opcode_setglobal(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        let value = self.peek(0).clone();
        trace!("set global id=0x{:08x} as {}", id, value);
        self.globals.define_by_id(id, value);
        Ok(())
    }


    fn opcode_setproperty(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        // Read field name from the constants table
        let constant = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let field = constant.as_string();

        let value = self.pop(); // Value to assign
        let mut instance = self.pop();// Value with field to be written

        if instance.is_instance() {
            let mut instance = instance.as_instance_mut();
            instance.set(field.as_str(), value.clone());
            trace!("set field '{}' of {} to {}", field, instance, value);
            self.push(value);
        } else {
            r_error!(format!("{} does not have properties to set", instance))
        }
        Ok(())
    }


    fn opcode_capture(&mut self, len: usize) -> Result<(), RuntimeError> {
        let constant = self.callframe_mut().read_bytes(len) as usize;
        // Get the function from constants table
        let value = self.callframe().closure_ref().function_ref().read_constants().value_by_id(constant).clone();
        let upvalue_count = value.as_function().upvalue_count();
        // Wrap it in a closure
        let mut closure = Closure::new(value);
        
        // This opcode is followed by one variable length entry per upvalue
        for _i in 0..upvalue_count {
            trace!("capturing upvalue {} of {}", _i, upvalue_count);
        
            // Decode is_local and id
            // Because we allow more than 255 upvalues and local variables,
            // we have to encode the length if each id. This means the code 
            // to decode each entry is a bit more complicated than in clox.
            let byte = self.callframe_mut().read_bytes(1) as u8;
            let is_local = if (byte & 128) == 128 { true } else { false };
            let id_len = byte & 127; // 1=byte, 2=word, 4=dword
            let id: usize = self.callframe_mut().read_bytes(id_len as usize) as usize;
            // Capture upvalue and insert into closure
            trace!("id={} is_local={}", id, is_local);
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


    fn opcode_class(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        let name = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).clone();
        let class = Class::new(name.as_string().as_str());
        trace!("class {} defined using constant id=0x{:08x}: {}", class, id, name);
        self.push(Value::class(class));
        Ok(())
    }


    fn opcode_method(&mut self, len: usize) -> Result<(), RuntimeError> {
        let id = self.callframe_mut().read_bytes(len) as usize;
        let method_name = self.callframe().closure_ref().function_ref().read_constants().value_by_id(id).as_string().clone();
        let method_value = self.pop();
        let mut class_value = self.peek(0).clone();
        trace!("popped {} off stack, added as method '{}' of {}", method_value, method_name, class_value);
        class_value.as_class_mut().set(&method_name, method_value);
        Ok(())
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
            Err(msg) => { r_error!(msg) }
        }
        Ok(())
    }
    
    fn opcode_sub(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.subtract(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(msg) => { r_error!(msg) }
        }
        Ok(())
    }
    
    fn opcode_mul(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.multiply(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(msg) => { r_error!(msg) }
        }
        Ok(())
    }
    
    fn opcode_div(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.divide(&b);
        match res {
            Ok(value) => { self.push(value); } // Division by zero = f64::INFINITY
            Err(msg) => { r_error!(msg) }
        }
        Ok(())
    }
    
    fn opcode_mod(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        let res = a.modulo(&b);
        match res {
            Ok(value) => { self.push(value); }
            Err(msg) => { r_error!(msg) }
        }
        Ok(())
    }
    
    fn opcode_equal(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        trace!("comparing {} and {} for equality", a, b);
        self.push(Value::boolean(a.eq(&b)));
        Ok(())
    }
    
    fn opcode_same(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        trace!("comparing {} and {} for sameness", a, b);
        self.push(Value::boolean(a.is(&b)));
        Ok(())
    }

    fn opcode_notequal(&mut self) -> Result<(), RuntimeError> {
        let b = self.pop();
        let a = self.pop();
        trace!("comparing {} and {} for negative equality", a, b);
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

    fn opcode_jmp(&mut self, len: usize) -> Result<(), RuntimeError> {
        let ip = self.callframe_mut().read_bytes(len);
        self.callframe_mut().jmp(ip);
        Ok(())
    }
    
    fn opcode_jmpfalsep(&mut self, len: usize) -> Result<(), RuntimeError> {
        let ip = self.callframe_mut().read_bytes(len);
        if !self.pop().is_truthy() { self.callframe_mut().jmp(ip); }
        Ok(())
    }

    fn opcode_jmpfalseq(&mut self, len: usize) -> Result<(), RuntimeError> {
        let ip = self.callframe_mut().read_bytes(len);
        if !self.peek(0).is_truthy() { self.callframe_mut().jmp(ip); }
        Ok(())
    }

    fn opcode_pop(&mut self) -> Result<(), RuntimeError> {
        let _value = self.pop();
        Ok(())
    }
    
    fn opcode_popn(&mut self) -> Result<(), RuntimeError> {
        let count = self.callframe_mut().read_bytes(1);
        for _ in 0..count {
            let _value = self.pop();
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
        class.as_class_mut().inherit_from(&superclass);
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
        trace!("current stack size is {}", self.stack.size());
        trace!("callframe stack bottom is at {}", self.callframe().stack_bottom());
        let depth = self.stack.size()
            - self.callframe().stack_bottom()
            - 1
            - slot;
        trace!("requested slot is {}; {} - {} - 1 - {} = {}", slot, self.stack.size(), self.callframe().stack_bottom(), slot, depth);
        return depth;
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
        trace!("calling {} with {} argument(s)", value, argc);
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
        } else if value.is_native_method() {
            let bound = value.as_native_method();
            if bound.method().as_native().arity() != argc as usize{
                r_error!(format!("Expected {} argument(s) but got {}", bound.method().as_native().arity(), argc))
            }
            self.stack.poke(bound.receiver().clone(), argc as usize);
            let depth = self.stack.len() - argc as usize - 1;
            let result = bound.method().as_native().callable()(&mut self.stack.as_mut_slice()[depth..])?;
            self.stack.truncate(depth); // Discard the receiver and the arguments, if any
            self.push(result);
        } else {
            r_error!(format!("VM.call_value({}, {}) not implemented.", value, argc))
        }
        Ok(())
    }

        
    fn bind_method(&mut self, class: &Value, method_name: &str) -> Result<(), RuntimeError> {
        let receiver = self.stack.pop();
        trace!("class={} method={} receiver={}", class, method_name, receiver);
        if !class.is_class() {
            r_error!(format!("Can not bind '{}' to {} as {} because it is not a class", method_name, receiver, class))
        }
        if !receiver.is_instance() {
            r_error!(format!("Can not bind '{}' to {} because it is not an instance", method_name, receiver))
        }
        // clox looks up the class by name, 
        // but the receiver already has a reference to its class.
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

    fn bind_native_method(&mut self, callable: Value) -> Result<(), RuntimeError> {
        let receiver = self.pop();
        let bound_native_method = NativeMethod::new(receiver, callable);
        self.push(Value::native_method(bound_native_method));
        Ok(())
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
        trace!("stack_addr={} stack={:?}", stack_addr, self.stack);
    
        // If slot is already captured, return the upvalue
        // Not exactly sure why, but I think this is because there may 
        // be multiple closures between the current one and the one
        // where the actual variable lives. Through the RefCell,
        // they will all refer to the same state (None=open/Some=closed)
        // and value (if closed). I think.
        for upvalue in &self.open_upvalues {
            if upvalue.addr() == stack_addr { return upvalue.clone(); }
        }
        
        trace!("capturing as index={} of open_upvalues", self.open_upvalues.len());
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
        trace!("stack_addr={} stack={:?}", stack_addr, self.stack);
        loop {
            match self.open_upvalues.last_mut() {
                Some(upvalue) => {
                    trace!("consider closing {}", upvalue);
                    
                    // Keep going while last_slot >= upvalue.slot()             
                    // in clox, slot is a pointer to the stack, 
                    // while here it's an index relative to the callframe
                    
                    // Stop if we find an upvalue object referencing a stack address
                    // below stack_addr
                    if upvalue.addr() < stack_addr { 
                        trace!("upvalue.addr() < stack_addr, exiting");
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
                    trace!("no more open upvalues, exiting.");
                    return;
                }
            }
            trace!("closed {}", self.open_upvalues.last().unwrap());
            
            // We're no longer borrowing a reference to self.open_upvalues
            // so we can safely pop the one we just closed.
            self.open_upvalues.pop();
            trace!("{} upvalue(s) still open", self.open_upvalues.len());
        }
    }
}
