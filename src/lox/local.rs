

pub struct Local {
    name:	String,
    depth:	usize,	// Scope depth within function
    defined: 	bool,
    captured:	bool,
}


#[allow(dead_code)]
impl Local {
    pub fn new(name: &str, depth: usize) -> Local {
        Local {
            name:	name.to_string(),
            depth,
            defined: 	false,
            captured:	false,
        }
    }
    
    pub fn define(&mut self) {
        self.defined = true;
    }

    pub fn capture(&mut self) {
        self.captured = true;
    }

    pub fn name(&self) -> &str {
        return self.name.as_str();
    }
    
    pub fn depth(&self) -> usize {
        return self.depth;
    }
    
    pub fn is_defined(&self) -> bool {
        return self.defined;
    }

    pub fn is_captured(&self) -> bool {
        return self.captured;
    }
}
