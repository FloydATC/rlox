
use std::rc::Rc;
use std::borrow::Borrow;
use std::ptr;

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


impl Value {
    pub fn add(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a + b));
            }
            (Value::Obj(ra), Value::Obj(rb)) => {
                // Value::Obj is Rc<Obj>, dereference and compare
                match (&*ra.borrow(), &*rb.borrow()) {
                    (Obj::String(a), Obj::String(b)) 	 => {
                        let string = format!("{}{}", a, b);
                        return Ok(Value::string(&string));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Err(format!("Can not add operands {:?} and {:?}.", &self, &other));
    }
}


impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::Null => return Value::null(),
            Value::Bool(b) => return Value::boolean(*b),
            Value::Number(n) => return Value::number(*n),
            Value::Obj(o) => return Value::Obj(o.clone()),
        }
    }
}


impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool { 
        match (self, other) {
            (Value::Null, Value::Null) 			 => true,
            (Value::Bool(a), Value::Bool(b)) 		 => a == b,
            (Value::Number(a), Value::Number(b)) 	 => a == b,
            (Value::Obj(ra), Value::Obj(rb)) => {
                // Value::Obj is Rc<Obj>, dereference and compare
                match (&*ra.borrow(), &*rb.borrow()) {
                    (Obj::String(a), Obj::String(b)) 	 => a == b,
                    // Obj types other than Obj::String must be same object
                    (Obj::Function(a), Obj::Function(b)) => ptr::eq(a, b),
                    (Obj::Closure(a), Obj::Closure(b)) 	 => ptr::eq(a, b),
                    _ => false, // Obj types mismatch
                }
            }
            _ => false, // Value types mismatch
        }    
    }
}


