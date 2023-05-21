

use super::{local::Local, local_set::LocalSet, upvalue::Upvalue};


#[derive(Debug)]
pub struct Locals {
    current: 	Option<Box<LocalSet>>,
}


impl Locals {
    pub fn new(with_receiver: bool) -> Self {
        Self {
            current: Some(Box::new(LocalSet::new(None, with_receiver))),
        }
    }

    // Begin a new LocalSet (=function declaration)
    pub fn begin_function(&mut self, with_receiver: bool) {
        
        // Is this even possible? As long as I never do anything
        // stupid with self.localsets the references would remain valid,
        // but I'm afraid there's no way to make such a promise.
        
        let parent = self.current.take();
        self.current = Some(Box::new(LocalSet::new(parent, with_receiver)));
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
//    pub fn local_mut_by_id(&mut self, id: usize) -> &mut Local {
//        return self.current.as_mut().unwrap().local_mut_by_id(id);
//    }
    
    // Return the number of locals in current LocalSet
    pub fn local_count(&self) -> usize {
        return self.current.as_ref().unwrap().local_count();
    }
    
    // Return a reference to the last local in the current LocalSet
    // Return None if none exist
    pub fn last_local(&mut self) -> Option<&mut Local> {
        return self.current.as_mut().unwrap().last_local();
    }
    
    // Discard the last local in the current LocalSet, if any
    pub fn pop_local(&mut self) {
        return self.current.as_mut().unwrap().pop_local();
    }
}

// ======== Upvalues ========
#[allow(dead_code)]
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

