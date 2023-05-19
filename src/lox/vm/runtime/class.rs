

use std::collections::HashMap;


use crate::lox::Value;


#[derive(Clone)]
pub struct Class {
    name: String,
    superclass: Option<Value>,
    methods: HashMap<String, Value>,
}


impl Class {

    pub fn new(name: &str) -> Self {
        //println!("Class::new() {}", name);
        Self {
            name: name.to_string(),
            superclass: None,
            methods: HashMap::new(),
        }
    }

}


#[allow(dead_code)]
impl Class {
    
    pub fn name(&self) -> &str {
        return &self.name;
    }
    
    pub fn superclass(&self) -> Option<&Value> {
        return self.superclass.as_ref();
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.methods.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        return self.methods.get(name);
    }

    // Used exclusively by .inherit_from()
    fn methods(&self) -> &HashMap<String, Value> {
        return &self.methods;
    }

    // Used exclusively from VM opcode_inherit() which is only emitted by the compiler
    // during declaration of a Class, before any of its own methods are compiled.
    // We therefore know that &mut self has no methods of its own yet at this point.
    pub fn inherit_from(&mut self, other: &Value) {
        if !other.is_class() { panic!("Can not inherit from {}", other) }
        self.methods = other.as_class().methods().clone();
        self.superclass = Some(other.clone());
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
