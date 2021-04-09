
use std::rc::Rc;
use std::borrow::Borrow;

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
//    pub fn upvalue(v: Value) -> Value {
//        Value::Obj(Rc::new(Obj::upvalue(v)))
//    }
    
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
    
    pub fn truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Obj(ra) => {
                match &*ra.borrow() {
                    Obj::String(a) => a.len() > 0,
                    _ => true,
                }
            }
        }
    }
}


impl Value {
    pub fn add(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Bool(a), Value::Bool(b)) => {
                return Ok(Value::boolean(*a || *b));
            }
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

    pub fn subtract(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a - b));
            }
            _ => {}
        }
        return Err(format!("Can not subtract operands {:?} and {:?}.", &self, &other));
    }

    pub fn multiply(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Bool(a), Value::Bool(b)) => {
                return Ok(Value::boolean(*a && *b));
            }
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a * b));
            }
            (Value::Obj(ra), Value::Number(b)) => {
                // Value::Obj is Rc<Obj>, dereference and compare
                match &*ra.borrow() {
                    Obj::String(a) => {
                        let count = *b as usize;
                        let string = a.repeat(count);
                        return Ok(Value::string(&string));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Err(format!("Can not multiply operands {:?} and {:?}.", &self, &other));
    }

    pub fn divide(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a / b));
            }
            _ => {}
        }
        return Err(format!("Can not divide operands {:?} and {:?}.", &self, &other));
    }

    pub fn modulo(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a % b));
            }
            _ => {}
        }
        return Err(format!("Can not divide operands {:?} and {:?}.", &self, &other));
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
            (Value::Obj(ra), Value::Obj(rb)) 	 	 => ra == rb,
            _ => false, // Value types mismatch
        }    
    }
}


impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::Obj(ra), Value::Obj(rb)) => ra.partial_cmp(rb),
            _ => None, // Value types mismatch
        }
    }
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null		=> write!(f, "Value::Null"),
            Value::Bool(b)	=> write!(f, "Value::Bool({})", b),
            Value::Number(n)	=> write!(f, "Value::Number({})", n),
            Value::Obj(rc)	=> write!(f, "Value::{}", rc),
        }
    }
}

