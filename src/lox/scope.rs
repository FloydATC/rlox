

use super::local::Local;


pub struct Scope {
    depth: 	u32,
    locals:	Vec<Local>,
}


#[allow(dead_code)]
impl Scope {
    pub fn new(depth: u32) -> Scope {
        Scope {
            depth,
            locals	: vec![],
        }
    }
    
    pub fn local_count(&self) -> u32 {
        return self.locals.len() as u32;
    }
    
    pub fn locals(&self) -> &Vec<Local> {
        return &self.locals;
    }
    
    pub fn resolve(&self, name: &str) -> Option<u32> {
        for (i, local) in self.locals.iter().enumerate() {
            if local.name() == name { return Some(i as u32); }
        }
        return None; // TODO
    }
    
    pub fn declare(&mut self, name: &str) {
        self.locals.push(Local::new(name, self.depth));
    }
    
}

