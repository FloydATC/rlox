

use std::collections::HashMap;


use super::Value;
use super::{NativeFn, NativeCallable};

// Built-in functions and methods


#[allow(dead_code)]
pub struct NativeCallables {
    methods: HashMap<String, Value>,
    functions: HashMap<String, Value>,
}


#[allow(dead_code)]
impl NativeCallables {

    pub fn new() -> Self {
        NativeCallables {
            methods: HashMap::new(),
            functions: HashMap::new(),
        }
    }


    pub fn insert_method(&mut self, name: &str, method: NativeFn, arity: usize) {
        let callable = NativeCallable::new(name.to_string(), method, arity);
        let _ = self.methods.insert(name.to_string(), Value::native(callable));
    }


    pub fn insert_function(&mut self, name: &str, function: NativeFn, arity: usize) {
        let callable = NativeCallable::new(name.to_string(), function, arity);
        let _ = self.methods.insert(name.to_string(), Value::native(callable));
    }


    pub fn get_method(&self, name: &str) -> Option<&Value> {
        return self.methods.get(name);
    }

    pub fn get_function(&self, name: &str) -> Option<&Value> {
        return self.functions.get(name);
    }

}
