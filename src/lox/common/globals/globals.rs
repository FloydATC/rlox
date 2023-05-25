

use std::collections::HashMap;


use crate::lox::compiler::{CompileError, c_error};
use crate::lox::common::IdentifierKind;
use super::Global;

// The rlox compiler accesses variables by name =~ O(logN)
// The rlox VM accesses variables by id == O(1)


#[allow(dead_code)]
#[derive(Clone)]
pub struct Globals<T> {
    index: HashMap<String, Global>,	// Used at script compile time only
    values: Vec<Option<T>>,	
}


//#[allow(dead_code)]
impl<T> Globals<T> {
    pub fn new() -> Self {
        Self {
            index:	HashMap::new(),
            values:	Vec::new(),
        }
    }
    
    // Return Ok(id) if name is unique
    // Otherwise, return Err(String)
    // Note: Used at script compile time only
    pub fn declare(&mut self, name: &str, kind: IdentifierKind) -> Result<usize, CompileError> {
        let opt_global = self.index.get(name);
        match opt_global {
            None => {
                let id = self.values.len();
                self.values.push(None);
                self.index.insert(name.to_string(), Global::new(id, kind));
                return Ok(id);
            }
            Some(_) => {
                c_error!(format!("Global '{}' already declared", name))
            }
        }
    }

    // Assign value to id
    // Panic if id is invalid
    // Used at runtime
    pub fn define_by_id(&mut self, id: usize, value: T) {
        self.values[id] = Some(value);
    }
    
    // Return value associated with id (if any)
    // Panic if id is invalid
    // Used at runtime
    pub fn value_by_id(&self, id: usize) -> Option<&T> {
        match &self.values[id] {
            Some(value) => return Some(&value),
            None => return None,
        }
    }
    
    // Return Some(id) if name exists
    // Otherwise, return None
    // Note: Used at script compile time only
    pub fn id_by_name(&self, name: &str) -> Option<usize> {
        let res = self.index.get(name);
        match res {
            Some(global) => {
                return Some(global.index());
            }
            None => {
                return None;
            }
        }
    }
    
    // Return the declared name associated with an id
    // Panic if id is invalid
    // Note: Used only to generate error messages =~ O(N)
    pub fn name_by_id(&self, id: usize) -> &String {
        for (name, global) in &self.index {
            if global.index() == id { return &name; }
        }
        panic!("Id {} not found in index, length of vector is {}.", id, self.values.len());
    }


    // Note: Used only at compile time =~ O(N)
    pub fn global_ref_by_id(&self, id: usize) -> &Global {
        for (_name, global) in &self.index {
            if global.index() == id { return global; }
        }
        panic!("Id {} not found in index, length of vector is {}.", id, self.values.len());
    }


    // Note: Used only at compile time =~ O(N)
    pub fn global_mutref_by_id(&mut self, id: usize) -> &mut Global {
        for (_name, global) in &mut self.index {
            if global.index() == id { return global; }
        }
        panic!("Id {} not found in index, length of vector is {}.", id, self.values.len());
    }


    // Return the number of variables
    // Only used in testing
    #[cfg(test)]
    pub fn count(&self) -> usize {
        return self.values.len();
    }    

}


impl<T> std::fmt::Debug for Globals<T> 
    where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (id, value) in self.values.iter().enumerate() {
            let name = self.name_by_id(id);
            match value {
                Some(value) => {
                    write!(f, "  0x{:04x} {}={}\n", id, name, &value)?;
                }
                None => {
                    write!(f, "  0x{:04x} {}=undefined\n", id, name)?;
                }
            }
        }
        Ok(())
    }
}



