
use std::cell::{Ref, RefMut};

use super::value::Value;
use super::opcode::OpCode;
use super::closure::Closure;


pub struct CallFrame {
    closure_value:	Value,
    ip: 		u32,
    stack_bottom:	usize,
}


impl CallFrame {
    pub fn new(closure: Value, stack_bottom: usize) -> CallFrame {
        if !closure.is_closure() {
            panic!("{} is not a Closure", closure);
        }
        //println!("CallFrame.new() stack_bottom={}", stack_bottom);
        CallFrame { 
            closure_value:	closure,
            ip: 		0,
            stack_bottom,
        }
    }


    // Shorthand for dereferencing the current closure as;
    // ...immutable
    pub fn closure_ref(&self) -> Ref<'_, Closure> {
        return self.closure_value.as_closure();
    }
    // ...and mutable
    pub fn closure_mut(&mut self) -> RefMut<'_, Closure> {
        return self.closure_value.as_closure_mut();
    }


    pub fn read_op(&mut self) -> OpCode {
        let byte = self.read_byte();
        return byte.into();
    }
    
    
    // Shorthand functions for reading the bytecode
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.closure_ref().function_ref().read_chunk().read_byte(self.ip);
        self.ip = self.ip + 1;
        return byte;        
    }

    pub fn read_word(&mut self) -> u16 {
        let word = self.closure_ref().function_ref().read_chunk().read_word(self.ip);
        self.ip = self.ip + 2;
        return word;        
    }

    pub fn read_dword(&mut self) -> u32 {
        let dword = self.closure_ref().function_ref().read_chunk().read_dword(self.ip);
        self.ip = self.ip + 4;
        return dword;        
    }


    // State of the callframe itself
    pub fn ip(&self) -> u32 {
        return self.ip;
    }
    
    pub fn jmp(&mut self, ip: u32) {
        self.ip = ip;
    }
    
    pub fn stack_bottom(&self) -> usize {
        return self.stack_bottom;
    }
}
