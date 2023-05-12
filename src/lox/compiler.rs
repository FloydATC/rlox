
// Unlike clox, rlox splits parsing and compilation into separate
// modules; the compiler deals only with the actual encoding of 
// logical opcodes into bytecode representation for a single function

use super::opcode::{OpCode, OpCodeSet};
use super::value::Value;
use super::function::Function;


#[cfg(test)]
mod test;


// ======== Layout ========
pub struct Compiler {
    function: 	Option<Function>,
}


// ======== Public interface ========
//#[allow(dead_code)]
impl Compiler {
    pub fn new(function: Function) -> Compiler {
        //println!("Compiler::new()");
        Compiler {
            function: 	Some(function),
        }
    }


    pub fn take_function(&mut self) -> Function {
        let function = self.function.take().unwrap();
        return function;
    }

    pub fn current_ip(&self) -> u32 {
        return self.function
            .as_ref()
            .expect("Internal error: self.function is None")
            .read_chunk()
            .length();
    }

    // Pick OpCode variant based on argument size
    pub fn emit_op_variant(&mut self, ops: &OpCodeSet, arg: u64) {
        match arg {
            0..=0xff => {
                //println!("emitting opcode={} len={} arg={}", ops.byte.mnemonic(), ops.byte.len(), arg);
                assert_eq!(ops.byte.len(), 1);
                self.emit_op(&ops.byte);
                self.emit_bytes(arg as u32, ops.byte.len());
            }
            0x100..=0xffff => {
                //println!("emitting opcode={} len={} arg={}", ops.word.mnemonic(), ops.word.len(), arg);
                assert_eq!(ops.byte.len(), 2);
                self.emit_op(&ops.word);
                self.emit_bytes(arg as u32, ops.byte.len());
            }
            0x10000..=0xffffffff => {
                //println!("emitting opcode={} len={} arg={}", ops.dword.mnemonic(), ops.dword.len(), arg);
                assert_eq!(ops.byte.len(), 4);
                self.emit_op(&ops.dword);
                self.emit_bytes(arg as u32, ops.byte.len());
            }
            _ => {
                panic!("Argument greater than 32 bits.");
            }
        }
    }

    pub fn emit_op(&mut self, opcode: &OpCode) {
        self.emit_bytes(opcode.as_byte() as u32, 1);
    }


    pub fn emit_bytes(&mut self, byte: u32, len: usize) {
        self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .chunk()
            .append_bytes(byte, len);
    }

    
    pub fn emit_jmp(&mut self, opcode: &OpCode) -> u32 {
        self.emit_op(opcode);
        let current_ip = self.current_ip();
        self.emit_bytes(0xffffffff, opcode.len());
        return current_ip;
    }
    
    pub fn patch_jmp(&mut self, ip: u32) {
        let current_ip = self.current_ip();
        self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .chunk()
            .write_bytes(current_ip, ip, OpCode::Jmp.len());
    }
    
    pub fn function(&mut self) -> &mut Function {
        return self.function
            .as_mut()
            .expect("Internal error: self.function is None");
    }
    
    pub fn make_constant(&mut self, value: Value) -> usize {
        return self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .constants()
            .make(value);
    }
}


impl Drop for Compiler {
    fn drop(&mut self) {
        //println!("Compiler.drop()");
    }
}
