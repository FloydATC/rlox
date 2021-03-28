
use super::scanner::Scanner;
use super::parser::Parser;
use super::function::Function;


#[cfg(test)]
mod test;


//#[allow(dead_code)]
pub struct Compiler {
    //enclosing: 	Option<&mut Compiler>,
    parser: 	Option<Parser>,
    function: 	Option<Function>,
}


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
        //self.function = Some(&mut function);
        //Err("Not yet implemented.".to_string())
        //self.function = None;
        self.parser = None;
        
        let f = self.function.take().unwrap();
        return Ok(f);
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        println!("Compiler.drop()");
    }
}


