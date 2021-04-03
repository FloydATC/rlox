
#[cfg(test)]
mod test;

use std::collections::HashMap;


use super::value::Value;


// The rlox compiler accesses variables by name =~ O(logN)
// The rlox VM accesses variables by id == O(1)


#[allow(dead_code)]
pub struct Globals {
    index: HashMap<String,usize>,	// Used at script compile time only
    values: Vec<Option<Value>>,	
}


#[allow(dead_code)]
impl Globals {
    pub fn new() -> Globals {
        Globals {
            index:	HashMap::new(),
            values:	Vec::new(),
        }
    }
    
    // Return Ok(id) if name is unique
    // Otherwise, return Err(String)
    // Note: Used at script compile time only
    pub fn declare(&mut self, name: &str) -> Result<usize, String> {
        let id = self.index.get(name);
        match id {
            None => {
                let id = self.values.len();
                self.values.push(None);
                self.index.insert(name.to_string(), id);
                return Ok(id);
            }
            Some(_) => {
                return Err(format!("Global '{}' already declared", name));
            }
        }
    }

    // Assign value to id
    // Panic if id is invalid
    pub fn define_by_id(&mut self, id: usize, value: &Value) {
        self.values[id] = Some(value.clone());
    }
    
    // Return value associated with id (if any)
    // Panic if id is invalid
    pub fn value_by_id(&self, id: usize) -> Option<&Value> {
        match &self.values[id] {
            Some(value) => return Some(&value),
            None => return None,
        }
    }
    
    // Return Some(id) if name exists
    // Otherwise, return None
    // Note: Used at script compile time only
    pub fn id_by_name(&self, name: &str) -> Option<u32> {
        let res = self.index.get(name);
        match res {
            Some(&id) => {
                return Some(id as u32);
            }
            None => {
                return None;
            }
        }
    }
    
    // Return the declared name associated with an id
    // Panic if id is invalid
    // Note: Used only to generate error messages =~ O(N)
    pub fn name_by_id(&self, id: u32) -> String {
        for (name, &i) in &self.index {
            if i == id as usize { return name.clone(); }
        }
        panic!("Id {} not found in index, length of vector is {}.", id, self.values.len());
    }

    // Return the number of variables
    pub fn count(&self) -> u32 {
        return self.values.len() as u32;
    }    
}


impl std::fmt::Debug for Globals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (id, value) in self.values.iter().enumerate() {
            let name = self.name_by_id(id as u32);
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



