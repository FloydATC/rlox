

use super::local::Local;
use super::upvalue::Upvalue;


// A LocalSet represents a set of local variables and upvalues
// for a single function declaration.
struct LocalSet {
    locals:	Vec<Local>,
    upvalues:	Vec<Upvalue>,
}


impl LocalSet {
    fn new() -> Self {
        Self {
            locals:	vec![Local::new("",0)], // Reserve stack slot zero
            upvalues:	vec![],
        }
    }
}


pub struct Locals {
    localsets: 	Vec<LocalSet>,
}


impl Locals {

    pub fn new() -> Self {
        Self {
            localsets:	vec![LocalSet::new()],
        }
    }

    // Begin a new LocalSet (=function declaration)
    pub fn begin_function(&mut self) {
        self.localsets.push(LocalSet::new());
    }
    
    // End the current LocalSet (=function declaration)
    pub fn end_function(&mut self) {
        self.localsets.pop();
    }
    
    // Return an immutable reference to the current LocalSet
    fn locals_ref(&self) -> &Vec<Local> {
        return &self.localsets.last().unwrap().locals;
    }

    // Return a mutable reference to the current LocalSet
    fn locals_mut(&mut self) -> &mut Vec<Local> {
        return &mut self.localsets.last_mut().unwrap().locals;
    }

    // Declare a local variable in the current LocalSet
    pub fn declare_local(&mut self, name: &str, depth: usize) {
        let locals = self.locals_mut();
        locals.push(Local::new(name, depth));        
    } 

    // Resolve local variable name in the current LocalSet
    // Return None if not found
    // Note: name may occur multiple times with different scope depths
    // so the search begins at the end (=innermost scope)
    pub fn resolve_local(&self, name: &str) -> Option<usize> {
        let locals = self.locals_ref();
        for id in (0..locals.len()).rev() {
            // TODO: May want to check is_defined() here?
            
            if locals[id].name() == name { return Some(id); }
        }
        return None;
    }
    
    // Return a reference to local in current LocalSet, by id
    // Note: Panic on invalid id
    pub fn local_ref_by_id(&self, id: usize) -> &Local {
        return &self.locals_ref()[id];
    }

    // Return a mutable reference to local in current LocalSet, by id
    // Note: Panic on invalid id
    pub fn local_mut_by_id(&mut self, id: usize) -> &mut Local {
        return &mut self.locals_mut()[id];
    }
    
    // Return the number of local variables in current LocalSet
    pub fn local_count(&self) -> usize {
        return self.locals_ref().len();
    }
    
    // Return a reference to the last local variable in the current LocalSet
    // Return None if none exist
    pub fn last_local(&self) -> Option<&Local> {
        return self.locals_ref().last();
    }
    
    // Discard the last local variable in the current LocalSet, if any
    pub fn pop_local(&mut self) {
        self.locals_mut().pop();
    }
}

