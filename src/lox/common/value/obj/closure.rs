
use std::cell::Ref;


use crate::lox::vm::Upvalue; // Runtime representation


use super::{Function, Value};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Closure {
    // Functions are first class objects so they're stored in Value
    function_value: 	Value,
    
    // An Upvalue object contains a slot no. and Rc<RefCell<T>>
    upvalues: 	Vec<Upvalue<Value>>,
}


#[allow(dead_code)]
impl Closure {
    pub fn new(function: Value) -> Self {
        if !function.is_function() {
            panic!("{} is not a Function", function);
        }
        Self { 
            function_value:	function, 
            upvalues: 		vec![],
        }
    }
    
    pub fn name(&self) -> String {
        return self.function_value.as_function().name().to_string();
    }
    

    pub fn function_ref(&self) -> Ref<'_, Function> {
        return self.function_value.as_function();
    }


    pub fn add_upvalue(&mut self, upvalue: Upvalue<Value>) {
        println!("Closure.add_upvalue() adding value={} as index={} of closure \"{}\"", upvalue, self.upvalues.len(), self.function_ref().name());
        self.upvalues.push(upvalue);
    }
    
    pub fn upvalue_ref_by_id(&self, id: usize) -> &Upvalue<Value> {
        return &self.upvalues[id];
    }

    pub fn upvalue_mut_by_id(&mut self, id: usize) -> &mut Upvalue<Value> {
        return &mut self.upvalues[id];
    }

}

