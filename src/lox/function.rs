
use super::chunk::Chunk;

#[allow(dead_code)]
pub struct Function {
    arity: u8,
    chunk: Chunk,
}


#[allow(dead_code)]
impl Function {
    pub fn new() -> Function {
        Function { 
            arity: 0,
            chunk: Chunk::new()
        }
    }
    pub fn chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
}


impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
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



