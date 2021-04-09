
//use std::rc::Rc;

use super::value::Value;
use super::opcode::OpCode;
//use super::obj::Obj;
use super::function::Function;
use super::closure::Closure;


#[allow(dead_code)]
pub struct CallFrame {
    closure: 		Value,
    ip: 		u32,
    stack_bottom:	u32,
}


#[allow(dead_code)]
impl CallFrame {
    pub fn new(closure: Value, stack_bottom: u32) -> CallFrame {
        println!("CallFrame.new() stack_bottom={}", stack_bottom);
        CallFrame { 
            closure,
            ip: 	0,
            stack_bottom,
        }
    }

    pub fn closure(&self) -> &Closure {
        return self.closure.as_closure();
    }

//    pub fn closure_mut(&mut self) -> &mut Closure {
//        let inner = Rc::get_mut(&mut self.closure.as_rc_object())
//            .expect("Could not get mutable reference to self.closure, because other references exist.");
//        return inner.as_closure_mut();
//    }

    pub fn function_ref(&self) -> &Function {
        //let closure = self.closure.as_rc_object().as_closure();
        return self.closure.as_closure().function();
    }
    
    pub fn read_op(&mut self) -> Option<OpCode> {
        let len = self.function_ref().read_chunk().length();
        if self.ip < len {
            let byte = self.read_byte();
            return Some(OpCode::code(byte));        
        } else {
            return None;
        }
    }
    
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.function_ref().read_chunk().read_byte(self.ip);
        self.ip = self.ip + 1;
        return byte;        
    }

    pub fn read_word(&mut self) -> u16 {
        let word = self.function_ref().read_chunk().read_word(self.ip);
        self.ip = self.ip + 2;
        return word;        
    }

    pub fn read_dword(&mut self) -> u32 {
        let dword = self.function_ref().read_chunk().read_dword(self.ip);
        self.ip = self.ip + 4;
        return dword;        
    }

    pub fn ip(&self) -> u32 {
        return self.ip;
    }
    
    pub fn jmp(&mut self, ip: u32) {
        self.ip = ip;
    }
    
    pub fn stack_bottom(&self) -> u32 {
        return self.stack_bottom;
    }
}
