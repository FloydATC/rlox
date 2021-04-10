
//use std::rc::Rc;
use std::cell::{Ref, RefMut};

use super::value::Value;
use super::opcode::OpCode;
//use super::obj::Obj;
//use super::function::Function;
use super::closure::Closure;


#[allow(dead_code)]
pub struct CallFrame {
    closure_value:	Value,
    ip: 		u32,
    stack_bottom:	u32,
}


//#[allow(dead_code)]
impl CallFrame {
    pub fn new(closure: Value, stack_bottom: u32) -> CallFrame {
        if !closure.is_closure() {
            panic!("{} is not a Closure", closure);
        }
        println!("CallFrame.new() stack_bottom={}", stack_bottom);
        CallFrame { 
            closure_value:	closure,
            ip: 		0,
            stack_bottom,
        }
    }

    pub fn closure_ref(&self) -> Ref<'_, Closure> {
        return self.closure_value.as_closure();
    }

    pub fn closure_mut(&mut self) -> RefMut<'_, Closure> {
        return self.closure_value.as_closure_mut();
    }

//    pub fn closure_ref(&self) -> Ref<'_, Obj> {
//        return self.closure_value.as_ref();
//    }

//    pub fn closure_mut(&mut self) -> &mut Closure {
//        return self.closure_value.as_closure_mut();
//    }

//    pub fn function_ref(&self) -> Ref<'_, Function> {
//        return self.closure_ref().function_ref();
//        return self.closure_value.as_closure().function();
//    }
    
    pub fn read_op(&mut self) -> OpCode {
        //let len = self.function_ref().read_chunk().length();
        //if self.ip < len {
            let byte = self.read_byte();
            return OpCode::code(byte);
        //} else {
        //    return None;
        //}
    }
    
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.closure_ref().function_ref().read_chunk().read_byte(self.ip);
    
    
//        let byte = self.function_ref().read_chunk().read_byte(self.ip);
        self.ip = self.ip + 1;
        return byte;        
    }

    pub fn read_word(&mut self) -> u16 {
//        let word = self.function_ref().read_chunk().read_word(self.ip);
        let word = self.closure_ref().function_ref().read_chunk().read_word(self.ip);
        self.ip = self.ip + 2;
        return word;        
    }

    pub fn read_dword(&mut self) -> u32 {
//        let dword = self.function_ref().read_chunk().read_dword(self.ip);
        let dword = self.closure_ref().function_ref().read_chunk().read_dword(self.ip);
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
