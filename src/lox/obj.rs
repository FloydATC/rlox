
//use std::rc::Rc;

use super::function::Function;
use super::closure::Closure;


#[allow(dead_code)]
#[derive(Debug)]
pub enum Obj {
    String(String),
    Function(Function),
    Closure(Closure),
}


#[allow(dead_code)]
impl Obj {
    pub fn string(s: &str) -> Obj {
        Obj::String(s.to_string())
    }
    pub fn function(f: Function) -> Obj {
        Obj::Function(f)
    }
    pub fn closure(c: Closure) -> Obj {
        Obj::Closure(c)
    }

//    pub fn as_string(&self) -> &str {
//        match self {
//            Obj::String(s) => return &s,
//            _ => panic!("{} is not a String Object"),
//        }
//    }
//    pub fn as_function(&self) -> Rc<Function> {
//        match self {
//            Obj::Function(f) => return &f,
//            _ => panic!("{} is not a Function Object"),
//        }
//    }
    pub fn as_closure(&self) -> &Closure {
        match self {
            Obj::Closure(c) => return &c,
            _ => panic!("{:?} is not a Closure Object", self),
        }
    }
}
