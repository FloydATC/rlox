
use std::rc::Rc;

use super::opcode::OpCode;
use super::obj::Obj;
use super::function::Function;
//use super::closure::Closure;


#[allow(dead_code)]
pub struct CallFrame {
    closure: Rc<Obj>,
    ip: usize,
}


#[allow(dead_code)]
impl CallFrame {
    pub fn new(closure: Rc<Obj>) -> CallFrame {
        CallFrame { 
            closure,
            ip: 0, 
        }
    }

    pub fn read_function(&self) -> &Function {
        let closure = self.closure.as_closure(); // Hmm.
        return closure.function();
    }
    
    pub fn read_op(&mut self) -> Option<OpCode> {
        let len = self.read_function().read_chunk().length();
        if self.ip < len {
            let byte = self.read_byte();
            return Some(OpCode::code(byte));        
        } else {
            return None;
        }
    }
    
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.read_function().read_chunk().read_byte(self.ip);
        self.ip = self.ip + 1;
        return byte;        
    }

    pub fn ip(&self) -> usize {
        return self.ip;
    }
}
