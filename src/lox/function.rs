
use super::chunk::Chunk;
use super::value::Value;
use super::constants::Constants;


pub const INITIALIZER: &str = "init";


#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum FunctionKind {
    Function,
    Initializer,
    Method,
    Script,
}


impl FunctionKind {

    pub fn has_receiver(&self) -> bool {
        return match self {
            FunctionKind::Initializer => true,
            FunctionKind::Method => true,
            _ => false,
        }
    }


    pub fn return_self(&self) -> bool {
        return match self {
            FunctionKind::Initializer => true,
            _ => false,
        }
    }


    pub fn is_toplevel(&self) -> bool {
        return match self {
            FunctionKind::Script => true,
            _ => false,
        }
    }

}


//#[allow(dead_code)]
pub struct Function {
    name:  		String,
    kind:		FunctionKind,
    arity: 		u8,
    chunk: 		Chunk,
    constants:  	Constants<Value>,
    upvalue_count:	usize,
}


#[allow(dead_code)]
impl Function {
    pub fn new(name: &str, kind: FunctionKind) -> Function {
        Function {
            name:		name.to_string(),
            kind,
            arity:		0,
            chunk: 		Chunk::new(),
            constants:		Constants::new(),
            upvalue_count:	0,
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
    
    pub fn set_upvalue_count(&mut self, upvalue_count: usize) {
        self.upvalue_count = upvalue_count;
    }
    
    pub fn upvalue_count(&self) -> usize {
        return self.upvalue_count;
    }
}


impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("kind", &self.kind)
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



