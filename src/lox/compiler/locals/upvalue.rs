
// This is the compile time representation of an upvalue;
// it contains information about what outer upvalue or local
// this upvalue refers to.
// The runtime representation of an upvalue is nested in "vm.rs"
#[allow(dead_code)]
#[derive(Debug)]
pub struct Upvalue {
    id:		usize,
    is_local:	bool,
}


#[allow(dead_code)]
impl Upvalue {
    pub fn new(id: usize, is_local: bool) -> Self {
        Self {
            id,
            is_local,
        }
    }
    
    pub fn id(&self) -> usize {
        return self.id;
    }
    
    pub fn is_local(&self) -> bool {
        return self.is_local;
    }
}

