

pub struct Upvalue {
    id:		usize,
    is_local:	bool,
}


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
