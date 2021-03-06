
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
                self.emit_op(&ops.byte);
                self.emit_byte(arg as u8);
            }
            0x100..=0xffff => {
                self.emit_op(&ops.word);
                self.emit_word(arg as u16);
            }
            0x10000..=0xffffffff => {
                self.emit_op(&ops.dword);
                self.emit_dword(arg as u32);
            }
            _ => {
                panic!("Argument greater than 32 bits.");
            }
        }
    }

    pub fn emit_op(&mut self, opcode: &OpCode) {
        self.emit_byte(*opcode as u8);
    }


    pub fn emit_byte(&mut self, byte: u8) {
        self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .chunk()
            .append_byte(byte);
    }


    pub fn emit_word(&mut self, word: u16) {
        self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .chunk()
            .append_word(word);
    }

    pub fn emit_dword(&mut self, dword: u32) {
        self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .chunk()
            .append_dword(dword);
    }
    
    pub fn emit_jmp(&mut self, opcode: &OpCode) -> u32 {
        self.emit_op(opcode);
        let current_ip = self.current_ip();
        self.emit_dword(0xffffffff);
        return current_ip;
    }
    
    pub fn patch_jmp(&mut self, ip: u32) {
        let current_ip = self.current_ip();
        self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .chunk()
            .write_dword(current_ip, ip);
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
