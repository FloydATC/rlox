

use super::local::Local;
use super::upvalue::Upvalue;


// A LocalSet represents a set of local variables and upvalues
// for a single function declaration.
// The goal is to hide this as an implementation detail of Locals
// so its caller does not have to worry about lifetimes.
struct LocalSet {
    locals:	Vec<Local>,
    upvalues:	Vec<Upvalue>,
    parent:	Option<Box<LocalSet>>,
}


impl LocalSet {
    fn new(parent: Option<Box<LocalSet>>) -> Self {
        Self {
            locals:	vec![Local::new("",0)], // Reserve stack slot zero
            upvalues:	vec![],
            parent,
        }
    }

}

// ======== Locals ========
impl LocalSet {    
    // Declare local in this LocalSet
    fn declare_local(&mut self, name: &str, depth: usize) {
        self.locals.push(Local::new(name, depth));
    }
    
    // Resolve local by name in this LocalSet
    // Return None if not found
    fn resolve_local(&self, name: &str) -> Option<usize> {
        for id in (0..self.locals.len()).rev() {
            // TODO: May want to check is_defined() here?
            
            if self.locals[id].name() == name { return Some(id); }
        }
        return None;
    }
    
    // Return immutable reference to local by id
    // Note: Panic if id is invalid
    fn local_ref_by_id(&self, id: usize) -> &Local {
        return &self.locals[id];
    }

    // Return mutable reference to local by id
    // Note: Panic if id is invalid
    fn local_mut_by_id(&mut self, id: usize) -> &mut Local {
        return &mut self.locals[id];
    }
    
    // Return number of locals in this LocalSet
    fn local_count(&self) -> usize {
        return self.locals.len();
    }
    
    // Return immutable reference to last local in this LocalSet
    // Note: None if there are no locals
    fn last_local(&self) -> Option<&Local> {
        return self.locals.last();
    }
    
    // Discard last local in this LocalSet
    fn pop_local(&mut self) {
        self.locals.pop();
    }
}

// ======== Upvalues ========
impl LocalSet {    
    // Return immutable reference to upvalue by id
    // Note: Panic if id is invalid
    fn upvalue_ref_by_id(&self, id: usize) -> &Upvalue {
        return &self.upvalues[id];
    }

    // Return number of upvalues in this LocalSet
    fn upvalue_count(&self) -> usize {
        return self.upvalues.len();
    }
    
    // Return immutable reference to last upvalue in this LocalSet
    // Note: None if there are no upvalues
    fn last_upvalue(&self) -> Option<&Upvalue> {
        return self.upvalues.last();
    }
    
    // Discard last upvalue in this LocalSet
    fn pop_upvalue(&mut self) {
        self.upvalues.pop();
    }
    
    // Add an upvalue in this LocalSet that references either
    // a) a local (if is_local == true) or
    // b) an upvalue (if is_local == false)
    // in the parent LocalSet
    fn add_upvalue(&mut self, id: usize, is_local: bool) -> usize {
    
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
    fn resolve_upvalue(&mut self, name: &str) -> Option<usize> {
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


pub struct Locals {
    current: 	Option<Box<LocalSet>>,
}


impl Locals {
    pub fn new() -> Self {
        Self {
            current: Some(Box::new(LocalSet::new(None))),
        }
    }

    // Begin a new LocalSet (=function declaration)
    pub fn begin_function(&mut self) {
        
        // Is this even possible? As long as I never do anything
        // stupid with self.localsets the references would remain valid,
        // but I'm afraid there's no way to make such a promise.
        
        let parent = self.current.take();
        self.current = Some(Box::new(LocalSet::new(parent)));
    }
    
    // End the current LocalSet (=function declaration)
    pub fn end_function(&mut self) {
        let parent = self.current.as_mut().unwrap().parent.take();
        self.current = parent;
    }
}


// ======== Locals ========
impl Locals {

    // Declare a local variable in the current LocalSet
    pub fn declare_local(&mut self, name: &str, depth: usize) {
        self.current.as_mut().unwrap().declare_local(name, depth);
    } 

    // Resolve local variable name in the current LocalSet
    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        return self.current.as_ref().unwrap().resolve_local(name);
    }
    
    // Return a reference to local in current LocalSet, by id
    // Note: Panic on invalid id
    pub fn local_ref_by_id(&mut self, id: usize) -> &Local {
        return self.current.as_mut().unwrap().local_ref_by_id(id);
    }

    // Return a mutable reference to local in current LocalSet, by id
    // Note: Panic on invalid id
    pub fn local_mut_by_id(&mut self, id: usize) -> &mut Local {
        return self.current.as_mut().unwrap().local_mut_by_id(id);
    }
    
    // Return the number of locals in current LocalSet
    pub fn local_count(&self) -> usize {
        return self.current.as_ref().unwrap().local_count();
    }
    
    // Return a reference to the last local in the current LocalSet
    // Return None if none exist
    pub fn last_local(&mut self) -> Option<&Local> {
        return self.current.as_mut().unwrap().last_local();
    }
    
    // Discard the last local in the current LocalSet, if any
    pub fn pop_local(&mut self) {
        return self.current.as_mut().unwrap().pop_local();
    }
}

// ======== Upvalues ========
impl Locals {

    pub fn resolve_upvalue(&mut self, name: &str) -> Option<usize> {
        return self.current.as_mut().unwrap().resolve_upvalue(name);    
    }

    // Return a reference to upvalue in current LocalSet, by id
    // Note: Panic on invalid id
    pub fn upvalue_ref_by_id(&mut self, id: usize) -> &Upvalue {
        return self.current.as_mut().unwrap().upvalue_ref_by_id(id);
    }

    // Return the number of upvalues in current LocalSet
    pub fn upvalue_count(&self) -> usize {
        return self.current.as_ref().unwrap().upvalue_count();
    }
    
    // Return a reference to the last upvalue in the current LocalSet
    // Return None if none exist
    pub fn last_upvalue(&mut self) -> Option<&Upvalue> {
        return self.current.as_mut().unwrap().last_upvalue();
    }
    
    // Discard the last upvalue in the current LocalSet, if any
    pub fn pop_upvalue(&mut self) {
        return self.current.as_mut().unwrap().pop_upvalue();
    }
}

