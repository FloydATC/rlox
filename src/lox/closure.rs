

use std::rc::Rc;

use super::function::Function;
use super::value::Value;
use super::obj::Obj;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Closure {
    //function: Function,
    rc_function: Rc<Obj>,
}


#[allow(dead_code)]
impl Closure {
    pub fn new(function: Value) -> Closure {
        let rc_object = function.as_rc_object();
        match function.as_rc_object().as_ref() {
            Obj::Function(_) => {
                return Closure { 
                    rc_function: rc_object, 
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
}
