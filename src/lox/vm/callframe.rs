
use std::cell::{Ref, RefMut};


use crate::lox::common::{Closure, OpCode, Value};


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


    // Shorthand functions for reading the bytecode
    pub fn read_op(&mut self) -> OpCode {
        let byte = self.closure_ref().function_ref().read_chunk().read_bytes(self.ip, 1) as u8;
        self.ip = self.ip + 1;
        return byte.into();
    }
    

    pub fn read_bytes(&mut self, len: usize) -> u32 {
        let result = self.closure_ref().function_ref().read_chunk().read_bytes(self.ip, len);
        self.ip = self.ip + len as u32;
        return result;
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


// Used by the VM when generating a stack trace after a RuntimeError
impl std::fmt::Debug for CallFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:0x{:08x}", self.closure_ref().name(), self.ip())
    }
}
