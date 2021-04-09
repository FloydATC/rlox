
#[cfg(test)]
mod test;

use std::rc::Rc;
use std::borrow::Borrow;

use super::obj::Obj;
use super::function::Function;
use super::closure::Closure;


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

    pub fn is_null(&self) -> bool {
        match self {
            Value::Null		=> true,
            _			=> false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Bool(_)	=> true,
            _			=> false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_)	=> true,
            _			=> false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> obj.is_string(),
            _ 			=> false
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> obj.is_function(),
            _ 			=> false
        }
    }

    pub fn is_closure(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> obj.is_closure(),
            _ 			=> false
        }
    }

    pub fn as_string(&self) -> &String {
        match self {
            Value::Obj(obj) 	=> obj.as_string(),
            _ 			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_function(&self) -> &Function {
        match self {
            Value::Obj(obj) 	=> obj.as_function(),
            _ 			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_closure(&self) -> &Closure {
        match self {
            Value::Obj(obj) 	=> obj.as_closure(),
            _ 			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null 	=> false,
            Value::Bool(b) 	=> *b,
            Value::Number(n) 	=> *n != 0.0,
            Value::Obj(obj) 	=> obj.is_truthy(),
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
        return Err(format!("Can not add operands {} and {}.", &self, &other));
    }

    pub fn subtract(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a - b));
            }
            _ => {}
        }
        return Err(format!("Can not subtract operands {} and {}.", &self, &other));
    }

    pub fn multiply(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Bool(a), Value::Bool(b)) => {
                return Ok(Value::boolean(*a && *b));
            }
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a * b));
            }
            (Value::Obj(_), Value::Number(b)) => {
                if self.is_string() {
                    let count = *b as usize;
                    let string = self.as_string().repeat(count);
                    return Ok(Value::string(&string));
                }
            }
            _ => {}
        }
        return Err(format!("Can not multiply operands {} and {}.", &self, &other));
    }

    pub fn divide(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a / b));
            }
            _ => {}
        }
        return Err(format!("Can not divide operands {} and {}.", &self, &other));
    }

    pub fn modulo(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a % b));
            }
            _ => {}
        }
        return Err(format!("Can not divide operands {} and {}.", &self, &other));
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
            (Value::Obj(a), Value::Obj(b)) 	 => a.partial_cmp(b),
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

