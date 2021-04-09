

use std::rc::Rc;

use super::function::Function;
use super::value::Value;
use super::obj::Obj;
use super::vm::upvalue::Upvalue; // Runtime representation

#[allow(dead_code)]
#[derive(Debug)]
pub struct Closure {
    // This is a reference to a function that lives in a Value
    // stored in a constants table somewhere. 
    rc_function: Rc<Obj>,
    
    // I *think* this should be a vector of references to values that
    // live in vm.open_upvalues
    upvalues: Vec<Upvalue<Value>>,
}


#[allow(dead_code)]
impl Closure {
    pub fn new(function: Value) -> Self {
        let rc_object = function.as_rc_object();
        match function.as_rc_object().as_ref() {
            Obj::Function(_) => {
                return Self { 
                    rc_function: rc_object, 
                    upvalues: vec![],
                };
            }
            _ => {
                panic!("{} is not a Function", function);
            }
        }
    }
    
    pub fn function(&self) -> &Function {
        return self.rc_function.as_function();
    }

// This may only be needed for GC. Not sure.    
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
