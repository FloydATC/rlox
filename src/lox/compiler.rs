
use super::scanner::Scanner;
use super::parser::Parser;
use super::opcode::OpCode;
use super::function::Function;


#[cfg(test)]
mod test;


// ======== Layout ========
//#[allow(dead_code)]
pub struct Compiler {
    //enclosing: 	Option<&mut Compiler>,
    parser: 	Option<Parser>,
    function: 	Option<Function>,
}


// ======== Public interface ========
//#[allow(dead_code)]
#[allow(unused_mut)]
impl Compiler {
    pub fn new() -> Compiler {
        println!("Compiler::new()");
        Compiler {
            //enclosing:	None,
            parser:	None,
            function: 	None,
            //function:	Function::new(),
        }
    }
    pub fn compile(&mut self, code: &str, function: Function) -> Result<Function, String> {
        println!("Compiler.compile() code={}", code);
        self.function = Some(function);
        
        let mut scanner = Scanner::new(&code);
        self.parser = Some(Parser::new(scanner));
        
        // Debug
        self.emit(OpCode::RETURN);
        
        self.parser = None;
        
        let function = self.function.take().unwrap();
        println!("{:?}", function);

        //return Err("Not yet implemented.".to_string());
        return Ok(function);
    }
}


#[allow(dead_code)]
impl Compiler {
    fn emit(&mut self, opcode: OpCode) {
        self.emit_byte(opcode as u8);
    }
    fn emit_byte(&mut self, byte: u8) {
        self.function
            .as_mut()
            .expect("Internal error: self.function is None")
            .chunk()
            .append_byte(byte);
    }
    fn emit_word(&mut self, word: u16) {
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


