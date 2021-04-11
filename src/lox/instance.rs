

use std::collections::HashMap;


use super::value::Value;


pub struct Instance {
    class_value:	Value,
    fields:		HashMap<String,Value>,
}


impl Instance {
    // ======== Constructors ========
    pub fn new(class: Value) -> Self {
        if !class.is_class() {
            panic!("{} is not a class", class);
        }
        Instance {
            class_value:	class,
            fields:		HashMap::new(),
        }
    }
}


impl Instance {

    pub fn class_name(&self) -> String {
        return self.class_value.as_class().name().to_string();
    }

    pub fn set(&mut self, field: &str, value: Value) {
        self.fields.insert(field.to_string(), value);
    }

    pub fn has(&self, field: &str) -> bool {
        return self.fields.contains_key(field);
    }
    
    pub fn get(&self, field: &str) -> &Value {
        return self.fields.get(field).unwrap();
    }
}


// ======== Traits ========

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("class", &self.class_name())
            .field("fields", &self.fields)
            .finish()
    }
}


impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Instance(class={})", self.class_name())
    }
}


impl Drop for Instance {
    fn drop(&mut self) {
        println!("Instance.drop() {}", self.class_name());
    }
}

