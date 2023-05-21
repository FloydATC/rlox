

pub struct Scope {
    depth: 	u32,
    locals:	u32,
}


#[allow(dead_code)]
impl Scope {
    pub fn new(depth: u32) -> Scope {
        Scope {
            depth,
            locals: 0,
        }
    }
    
    pub fn local_count(&self) -> u32 {
        return self.locals;
    }
    
    pub fn depth(&self) -> u32 {
        return self.depth;
    }
    
}

