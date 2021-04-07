
use super::chunk::Chunk;
use super::value::Value;
use super::constants::Constants;


#[allow(dead_code)]
#[derive(PartialEq)]
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
    constants:  Constants<Value>,
}


#[allow(dead_code)]
impl Function {
    pub fn new(name: &str, kind: FunctionKind) -> Function {
        Function {
            name:	name.to_string(),
            kind,
            arity:	0,
            chunk: 	Chunk::new(),
            constants:	Constants::new(),
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
    
    pub fn set_arity(&mut self, arity: u8) {
        self.arity = arity;
    }
    
    pub fn arity(&self) -> u8 {
        return self.arity;
    }
    
    pub fn constants(&mut self) -> &mut Constants<Value> {
        return &mut self.constants;
    }

    pub fn read_constants(&self) -> &Constants<Value> {
        return &self.constants;
    }
}


impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .field("chunk", &self.chunk)
            .field("constants", &self.constants)
            .finish()
    }
}


impl Drop for Function {
    fn drop(&mut self) {
        println!("Function.drop() {}", self.name);
    }
}



