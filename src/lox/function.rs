
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
