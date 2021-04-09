

use super::function::Function;
use super::value::Value;
use super::vm::upvalue::Upvalue; // Runtime representation


#[allow(dead_code)]
#[derive(Debug)]
pub struct Closure {
    // Functions are first class objects so they're stored in Value
    function_value: 	Value,
    
    // I *think* this must be a vector of references to values
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
    
    pub fn function(&self) -> &Function {
        return self.function_value.as_function();
    }

// This may only be needed for GC in clox. Not sure.    
//    pub fn upvalue_count(&self) -> usize {
//        return self.rc_upvalues.len();
//    }

    pub fn add_upvalue(&mut self, upvalue: Upvalue<Value>) {
        println!("Closure.add_upvalue() adding value={} as index={} of closure \"{}\"", upvalue, self.upvalues.len(), self.function().name());
        self.upvalues.push(upvalue);
    }
    
    pub fn upvalue_ref_by_id(&self, id: usize) -> &Upvalue<Value> {
        return &self.upvalues[id];
    }

    pub fn upvalue_mut_by_id(&mut self, id: usize) -> &mut Upvalue<Value> {
        return &mut self.upvalues[id];
    }
}
