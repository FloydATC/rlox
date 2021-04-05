
use super::chunk::Chunk;


#[allow(dead_code)]
pub enum FunctionKind {
    Function,
    Script,
}


//#[allow(dead_code)]
pub struct Function {
    name:  	String,
    kind:	FunctionKind,
    arity: 	u8,
    chunk: 	Chunk,
}


#[allow(dead_code)]
impl Function {
    pub fn new(name: &str, kind: FunctionKind, arity: u8) -> Function {
        Function {
            name:	name.to_string(),
            kind,
            arity,
            chunk: 	Chunk::new()
        }
    }
    pub fn chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }
    pub fn read_chunk(&self) -> &Chunk {
        &self.chunk
    }
    
    pub fn name(&self) -> &str {
        return &self.name;
    }
    
    pub fn kind(&self) -> &FunctionKind {
        return &self.kind;
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
        println!("Function.drop() {}", self.name);
    }
}



