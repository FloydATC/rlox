
//use super::parser::Parser;
use super::function::Function;


#[cfg(test)]
mod test;


#[allow(dead_code)]
pub struct Compiler {
    //enclosing: 	Option<&mut Compiler>,
    //parser: 	Option<Parser>,
    //function: 	Function,
}


#[allow(dead_code)]
impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            //enclosing:	None,
            //parser:	None,
            //function:	Function::new(),
        }
    }
    pub fn compile(&mut self, code: &str, _function: &mut Function) -> Result<(), String> {
        println!("Compiler.compile() code={}", code);
        //Err("Not yet implemented.".to_string())
        return Ok(());
    }
    fn drop(&mut self) {
        println!("Compiler.drop()");
    }
}


