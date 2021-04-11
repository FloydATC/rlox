
#[cfg(test)]
mod test;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use super::obj::Obj;
use super::function::Function;
use super::class::Class;
use super::instance::Instance;
use super::closure::Closure;


#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Obj(Rc<RefCell<Obj>>),
}


#[allow(dead_code)]
impl Value {
    // ======== Constructors ========

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
        Value::String(s.to_string())
    }

    pub fn function(f: Function) -> Value {
        Value::Obj(Rc::new(RefCell::new(Obj::function(f))))
    }

    pub fn class(c: Class) -> Value {
        Value::Obj(Rc::new(RefCell::new(Obj::class(c))))
    }

    pub fn closure(c: Closure) -> Value {
        Value::Obj(Rc::new(RefCell::new(Obj::closure(c))))
    }

    pub fn instance(i: Instance) -> Value {
        Value::Obj(Rc::new(RefCell::new(Obj::instance(i))))
    }

}
    

#[allow(dead_code)]
impl Value {
    // ======== Variant checks ========

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
            Value::String(_) 	=> true,
            _ 			=> false
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> RefCell::borrow(obj).is_function(),
            _ 			=> false
        }
    }

    pub fn is_class(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> RefCell::borrow(obj).is_class(),
            _ 			=> false
        }
    }

    pub fn is_closure(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> RefCell::borrow(obj).is_closure(),
            _ 			=> false
        }
    }

    pub fn is_instance(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> RefCell::borrow(obj).is_instance(),
            _ 			=> false
        }
    }

}


#[allow(dead_code)]
impl Value {
    // ======== Property checks ========

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null 	=> false,
            Value::Bool(b) 	=> *b,
            Value::Number(n) 	=> *n != 0.0,
            Value::String(s) 	=> s != "",
            Value::Obj(obj) 	=> RefCell::borrow(obj).is_truthy(),
        }
    }
}

#[allow(dead_code)]
impl Value {
    // ======== Getters ========

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

    pub fn as_string(&self) -> &String {
        match self {
            Value::String(s) 	=> return &s,
            _ 			=> {
                panic!("{} is not a string", self)
            }
        }
    }
    
    pub fn as_function(&self) -> Ref<'_, Function> {
        match self {
            Value::Obj(obj)	=> {
                Ref::map(obj.borrow(), |o| o.as_function())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_class(&self) -> Ref<'_, Class> {
        match self {
            Value::Obj(obj)	=> {
                Ref::map(obj.borrow(), |o| o.as_class())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_class_mut(&mut self) -> RefMut<'_, Class> {
        match self {
            Value::Obj(obj)	=> {
                RefMut::map(obj.borrow_mut(), |o| o.as_class_mut())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_closure(&self) -> Ref<'_, Closure> {
        match self {
            Value::Obj(obj)	=> {
                Ref::map(obj.borrow(), |o| o.as_closure())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_closure_mut(&mut self) -> RefMut<'_, Closure> {
        match self {
            Value::Obj(obj)	=> {
                RefMut::map(obj.borrow_mut(), |o| o.as_closure_mut())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_instance(&self) -> Ref<'_, Instance> {
        match self {
            Value::Obj(obj)	=> {
                Ref::map(obj.borrow(), |o| o.as_instance())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_instance_mut(&mut self) -> RefMut<'_, Instance> {
        match self {
            Value::Obj(obj)	=> {
                RefMut::map(obj.borrow_mut(), |o| o.as_instance_mut())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
}


impl Value {
    // ======== Arithmetics ========

    pub fn add(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Bool(a), Value::Bool(b)) => {
                return Ok(Value::boolean(*a || *b));
            }
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a + b));
            }
            (Value::String(a), Value::String(b)) => {
                let string = format!("{}{}", a, b);
                return Ok(Value::string(&string));
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
            (Value::String(a), Value::Number(b)) => {
                let count = *b as usize;
                let string = a.repeat(count);
                return Ok(Value::string(&string));
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


// ======== Traits ========

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::Null => return Value::null(),
            Value::Bool(b) => return Value::boolean(*b),
            Value::Number(n) => return Value::number(*n),
            Value::String(s) => return Value::string(s),
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
            (Value::String(a), Value::String(b)) 	 => a == b,
            (Value::Obj(ra), Value::Obj(rb)) 	 	 => ra == rb,
            _ => false, // Value types mismatch
        }    
    }
}


impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Obj(a), Value::Obj(b)) 	 => a.partial_cmp(b),
            _ => None, // Value types mismatch or can't be ordered
        }
    }
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null		=> write!(f, "Value::Null"),
            Value::Bool(b)	=> write!(f, "Value::Bool({})", b),
            Value::Number(n)	=> write!(f, "Value::Number({})", n),
            Value::String(s)	=> write!(f, "Value::String({})", s),
            Value::Obj(rc)	=> write!(f, "Value::{}", RefCell::borrow(rc)),
        }
    }
}

