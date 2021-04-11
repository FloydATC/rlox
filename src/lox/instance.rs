

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


impl Drop for Instance {
    fn drop(&mut self) {
        println!("Instance.drop() {}", self.class_name());
    }
}

