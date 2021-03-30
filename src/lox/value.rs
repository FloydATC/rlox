
use std::rc::Rc;

use super::obj::Obj;
use super::function::Function;
use super::closure::Closure;


#[allow(dead_code)]
#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    Obj(Rc<Obj>),
}


#[allow(dead_code)]
impl Value {
    pub fn null() -> Value {
        Value::Null
    }    
    pub fn boolean(b: bool) -> Value {
        Value::Bool(b)
    }
    pub fn number(n: f64) -> Value {
        Value::Number(n)
    }
    pub fn string(s: &str) -> Value {
        Value::Obj(Rc::new(Obj::string(&s)))
    }
    pub fn function(f: Function) -> Value {
        Value::Obj(Rc::new(Obj::function(f)))
    }
    pub fn closure(c: Closure) -> Value {
        Value::Obj(Rc::new(Obj::closure(c)))
    }
    
    pub fn as_boolean(&self) -> bool {
        match self {
            Value::Bool(b) => return *b,
            _ => panic!("{:?} is not a Boolean", self),
        }
    }
    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => return *n,
            _ => panic!("{:?} is not a Number", self),
        }
    }
    pub fn as_rc_object(&self) -> Rc<Obj> {
        match self {
            Value::Obj(obj) => return obj.clone(),
            _ => panic!("{:?} is not an Object", self),
        }
    }
}

