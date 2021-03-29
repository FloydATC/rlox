
//use super::scanner::Scanner;
//use super::parser::Parser;
use super::opcode::OpCode;
use super::function::Function;


#[cfg(test)]
mod test;


// ======== Layout ========
//#[allow(dead_code)]
pub struct Compiler {
    //enclosing: 	Option<&mut Compiler>,
    function: 	Option<Function>,
}


// ======== Public interface ========
#[allow(dead_code)]
//#[allow(unused_mut)]
impl Compiler {
    pub fn new(function: Function) -> Compiler {
        println!("Compiler::new()");
        Compiler {
            function: 	Some(function),
        }
    }


    pub fn take_function(&mut self) -> Function {
        let function = self.function.take().unwrap();
        return function;
    }


    pub fn emit(&mut self, opcode: OpCode) {
        self.emit_byte(opcode as u8);
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
}


impl Drop for Compiler {
    fn drop(&mut self) {
        println!("Compiler.drop()");
    }
}


