
use std::rc::Rc;

use super::obj::Obj;
//use super::closure::Closure;


#[allow(dead_code)]
pub struct CallFrame {
    closure: Rc<Obj>,
    ip: u32,
}


#[allow(dead_code)]
impl CallFrame {
    pub fn new(closure: Rc<Obj>) -> CallFrame {
        CallFrame { 
            closure,
            ip: 0, 
        }
    }
}
