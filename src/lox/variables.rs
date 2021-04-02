
#[cfg(test)]
mod test;

use std::collections::HashMap;


use super::value::Value;


// The rlox compiler accesses variables by name =~ O(logN)
// The rlox VM accesses variables by id == O(1)


#[allow(dead_code)]
pub struct Variables {
    index: HashMap<String,usize>,	// Used at script compile time only
    values: Vec<Option<Value>>,	
}


#[allow(dead_code)]
impl Variables {
    pub fn new() -> Variables {
        Variables {
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
                return Err(format!("Variable '{}' already declared", name));
            }
        }
    }

    // Assign value to id
    // Panic if id is invalid
    pub fn set_by_id(&mut self, id: usize, value: Value) {
        self.values[id] = Some(value);
    }
    
    // Return value associated with id (if any)
    // Panic if id is invalid
    pub fn get_by_id(&self, id: usize) -> Option<Value> {
        return self.values[id].clone();
    }
    
    // Return Some(id) if name exists
    // Otherwise, return None
    // Note: Used at script compile time only
    pub fn id_by_name(&self, name: &str) -> Option<usize> {
        let res = self.index.get(name);
        match res {
            Some(&id) => {
                return Some(id);
            }
            None => {
                return None;
            }
        }
    }
    
    // Return the declared name associated with an id
    // Panic if id is invalid
    // Note: Used only to generate error messages =~ O(N)
    pub fn name_by_id(&self, id: usize) -> String {
        for (name, &i) in &self.index {
            if i == id { return name.clone(); }
        }
        panic!("Id {} not found in index, length of vector is {}.", id, self.values.len());
    }
    
}


impl std::fmt::Debug for Variables {
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



