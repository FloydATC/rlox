

use super::Local;
use super::Upvalue;

// A LocalSet represents a set of local variables and upvalues
// for a single function declaration.
// The goal is to hide this as an implementation detail of Locals
// so its caller does not have to worry about lifetimes.
#[derive(Debug)]
pub struct LocalSet {
    pub locals:	Vec<Local>,
    pub upvalues:	Vec<Upvalue>,
    pub parent:	Option<Box<LocalSet>>,
}


impl LocalSet {
    pub fn new(parent: Option<Box<LocalSet>>, with_receiver: bool) -> Self {
        let receiver = if with_receiver { "this" } else { "" };
        Self {
            locals:	vec![Local::new(receiver,0)], // Reserve stack slot zero
            upvalues:	vec![],
            parent,
        }
    }

}

// ======== Locals ========
impl LocalSet {    
    // Declare local in this LocalSet
    pub fn declare_local(&mut self, name: &str, depth: usize) {
        self.locals.push(Local::new(name, depth));
    }
    
    // Resolve local by name in this LocalSet
    // Return None if not found
    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        for id in (0..self.locals.len()).rev() {
            // TODO: May want to check is_defined() here?
            
            if self.locals[id].name() == name { return Some(id); }
        }
        return None;
    }
    
    // Return immutable reference to local by id
    // Note: Panic if id is invalid
    pub fn local_ref_by_id(&self, id: usize) -> &Local {
        return &self.locals[id];
    }

    // Return mutable reference to local by id
    // Note: Panic if id is invalid
    pub fn local_mut_by_id(&mut self, id: usize) -> &mut Local {
        return &mut self.locals[id];
    }
    
    // Return number of locals in this LocalSet
    pub fn local_count(&self) -> usize {
        return self.locals.len();
    }
    
    // Return immutable reference to last local in this LocalSet
    // Note: None if there are no locals
    pub fn last_local(&mut self) -> Option<&mut Local> {
        return self.locals.last_mut();
    }
    
    // Discard last local in this LocalSet
    pub fn pop_local(&mut self) {
        self.locals.pop();
    }
}

// ======== Upvalues ========
impl LocalSet {    
    // Return immutable reference to upvalue by id
    // Note: Panic if id is invalid
    pub fn upvalue_ref_by_id(&self, id: usize) -> &Upvalue {
        return &self.upvalues[id];
    }

    // Return number of upvalues in this LocalSet
    pub fn upvalue_count(&self) -> usize {
        return self.upvalues.len();
    }
    
    // Return immutable reference to last upvalue in this LocalSet
    // Note: None if there are no upvalues
    pub fn last_upvalue(&self) -> Option<&Upvalue> {
        return self.upvalues.last();
    }
    
    // Discard last upvalue in this LocalSet
    pub fn pop_upvalue(&mut self) {
        self.upvalues.pop();
    }
    
    // Add an upvalue in this LocalSet that references either
    // a) a local (if is_local == true) or
    // b) an upvalue (if is_local == false)
    // in the parent LocalSet
    pub fn add_upvalue(&mut self, id: usize, is_local: bool) -> usize {
        //println!("LocalSet.add_upvalue() id={} is_local={}", id, is_local);
    
        // Scan existing upvalues
        for (i, upvalue) in self.upvalues.iter().enumerate() {
            if upvalue.id() == id && upvalue.is_local() == is_local {
                return i;
            }
        }
        
        // Not found, create it now
        let i = self.upvalues.len();
        self.upvalues.push(Upvalue::new(id, is_local));
        return i;
    }
    
    // Recursively search parent LocalSets for upvalue
    // or create one if name is found to be a local
    // in one of the parents
    pub fn resolve_upvalue(&mut self, name: &str) -> Option<usize> {
        //println!("LocalSet.resolve_upvalue() name={}", name);
        match &mut self.parent {
            None => {
                // If we have no parent, there can be no upvalues
                return None;
            }
            Some(parent) => {
            
                // Check for local in parent LocalSet
                let local = parent.resolve_local(name);
                if let Some(id) = local {
                    parent.local_mut_by_id(id).capture();
                    return Some(self.add_upvalue(id, true));
                }
            
                // Check for upvalue in parent LocalSet    
                let upvalue = parent.resolve_upvalue(name);
                if let Some(id) = upvalue {
                    return Some(self.add_upvalue(id, false));
                }
            
                return None;
            }
        }
    }
}

