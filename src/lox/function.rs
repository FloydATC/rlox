
use super::chunk::Chunk;

#[allow(dead_code)]
pub struct Function {
    name:  String,
    arity: u8,
    chunk: Chunk,
}


#[allow(dead_code)]
impl Function {
    pub fn new(name: &str, arity: u8) -> Function {
        Function {
            name:	name.to_string(),
            arity,
            chunk: 	Chunk::new()
        }
    }
    pub fn chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
}


impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .field("chunk", &self.chunk)
            .finish()
    }
}


impl Drop for Function {
    fn drop(&mut self) {
        println!("Function.drop()");
    }
}



