

pub struct Local {
    name:	String,
    depth:	u32,
    defined: 	bool,
}


#[allow(dead_code)]
impl Local {
    pub fn new(name: &str, depth: u32) -> Local {
        Local {
            name:	name.to_string(),
            depth,
            defined: 	false,	
        }
    }
    
    pub fn define(&mut self) {
        self.defined = true;
    }

    pub fn name(&self) -> &str {
        return self.name.as_str();
    }
    
    pub fn depth(&self) -> u32 {
        return self.depth;
    }
    
    pub fn is_defined(&self) -> bool {
        return self.defined;
    }
}
