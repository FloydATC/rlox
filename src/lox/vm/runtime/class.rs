

use std::collections::HashMap;


use crate::lox::Value;


#[derive(Clone)]
pub struct Class {
    name: String,
    methods: HashMap<String, Value>,
}


#[allow(dead_code)]
impl Class {

    pub fn new(name: &str) -> Self {
        println!("Class::new() {}", name);
        Self {
            name: name.to_string(),
            methods: HashMap::new(),
        }
    }

}


impl Class {
    
    pub fn name(&self) -> &str {
        return &self.name;
    }
    
    pub fn set(&mut self, name: &str, value: Value) {
        self.methods.insert(name.to_string(), value);
    }

// Not sure we really need this when get() can return None
//    pub fn has(&self, name: &str) -> bool {
//        return self.methods.contains_key(name);
//    }
    
    pub fn get(&self, name: &str) -> Option<&Value> {
        return self.methods.get(name);
    }

    pub fn methods(&self) -> &HashMap<String,Value> {
        return &self.methods;
    }

    pub fn methods_mut(&mut self) -> &mut HashMap<String,Value> {
        return &mut self.methods;
    }

}


impl std::fmt::Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Class")
            .field("name", &self.name)
            .field("methods", &self.methods)
            .finish()
    }
}


impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Class(name={})", self.name)
    }
}


impl Drop for Class {
    fn drop(&mut self) {
        println!("Class.drop() {}", self.name);
    }
}
