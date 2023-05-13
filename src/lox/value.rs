
#[cfg(test)]
mod test;


use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use super::obj::Obj;
use super::function::Function;
use super::vm::{Class, Method, Instance};
use super::closure::Closure;

mod array;

pub use array::Array;

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

    pub fn array(a: Array) -> Value {
        Value::Obj(Rc::new(RefCell::new(Obj::array(a))))
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

    pub fn method(m: Method) -> Value {
        Value::Obj(Rc::new(RefCell::new(Obj::method(m))))
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

    pub fn is_obj(&self) -> bool {
        match self {
            Value::Obj(_) 	=> true,
            _ 			=> false
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> RefCell::borrow(obj).is_array(),
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

    pub fn is_method(&self) -> bool {
        match self {
            Value::Obj(obj) 	=> RefCell::borrow(obj).is_method(),
            _ 			=> false
        }
    }

}


impl Value {
    // ======== Subscripting ========

    pub fn can_get(&self) -> bool {
        match self {
            Value::String(_) => true,
            Value::Obj(obj) => obj.borrow().can_get(),
            _ => false,
        }
    }

    pub fn get(&self, key: &Value) -> Option<Value> {
        match self {
            Value::String(s) => {
                if !key.is_number() { return None }
                let index = key.as_number().floor();
                if index < 0.0 || index >= s.len() as f64 { return None }
                match s.chars().nth(index as usize) {
                    Some(char) => return Some(Value::String(char.into())),
                    None => return None,
                }
            }
            Value::Obj(obj) => obj.borrow().get(key).cloned(),
            _ => None,
        }
    }

}


#[allow(dead_code)]
impl Value {
    // ======== Property checks ========

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => {
                if n.is_nan() { return false }
                *n != 0.0
            }
            Value::String(s) => s != "",
            Value::Obj(obj) => RefCell::borrow(obj).is_truthy(),
        }
    }

    pub fn is(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => {
                // NAN != NAN, but NAN is NAN
                if a.is_nan() && b.is_nan() { return true } 
                // INF is INF, and -INF is -INF
                if a.is_infinite() && b.is_infinite() { return a.is_sign_negative() == b.is_sign_negative() } 
                a.eq(b)
            }
            (Value::String(a), Value::String(b)) => a.eq(b),
            (Value::Obj(a), Value::Obj(b)) => Rc::ptr_eq(a, b), // Same RefCell?
            _ => false,
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
    
    pub fn as_obj(&self) -> Ref<'_, Obj> {
        match self {
            Value::Obj(obj) 	=> return obj.borrow(),
            _ 			=> {
                panic!("{} is not an Obj", self)
            }
        }
    }

    pub fn as_array(&self) -> Ref<'_, Array> {
        match self {
            Value::Obj(obj)	=> {
                Ref::map(obj.borrow(), |o| o.as_array())
            }
            _			=> {
                panic!("{} is not an object", self)
            }
        }
    }
    
    pub fn as_array_mut(&self) -> RefMut<'_, Array> {
        match self {
            Value::Obj(obj)	=> {
                RefMut::map(obj.borrow_mut(), |o| o.as_array_mut())
            }
            _			=> {
                panic!("{} is not an object", self)
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
    
    pub fn as_method(&self) -> Ref<'_, Method> {
        match self {
            Value::Obj(obj)	=> {
                Ref::map(obj.borrow(), |o| o.as_method())
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
            (Value::Obj(obj), _) => {
                return obj.borrow().append_value(&other);
            }
            (_, Value::Obj(obj)) => {
                return obj.borrow().prepend_value(&self);
            }
            _ => {
                return Err(format!("Can not add operands {} and {}", &self, &other));
            }
        }
    }

    pub fn subtract(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a - b));
            }
            (Value::Obj(obj), _) => {
                return obj.borrow().subtract_value(&other);
            }
            (_, Value::Obj(obj)) => {
                return obj.borrow().subtract_from_value(&self);
            }
            _ => {}
        }
        return Err(format!("Can not subtract operands {} and {}", &self, &other));
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
        return Err(format!("Can not multiply operands {} and {}", &self, &other));
    }

    pub fn divide(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a / b));
            }
            _ => {}
        }
        return Err(format!("Can not divide operands {} and {}", &self, &other));
    }

    pub fn modulo(self: &Value, other: &Value) -> Result<Value, String> {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => {
                return Ok(Value::number(a % b));
            }
            _ => {}
        }
        return Err(format!("Can not divide operands {} and {}", &self, &other));
    }

}


// ======== Traits ========

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Bool(b) => Value::boolean(*b),
            Value::Number(n) => Value::number(*n),
            Value::String(s) => Value::string(s),
            Value::Obj(o) => Value::Obj(o.clone()),
        }
    }
}


impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool { 
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            // Note: NAN != NAN, INF != INF, -INF != -INF
            (Value::Number(a), Value::Number(b)) => a.eq(b),
            (Value::String(a), Value::String(b)) => a.eq(b),
            (Value::Obj(ra), Value::Obj(rb)) => ra.borrow().eq(&rb.borrow()),
            _ => false, // Value types mismatch
        }    
    }
}


impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Obj(a), Value::Obj(b)) => a.partial_cmp(b),
            _ => None, // Value types mismatch or can't be ordered
        }
    }
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Null		=> write!(f, "null"),
            Value::Bool(b)	=> write!(f, "{}", b),
            Value::Number(n)	=> {
                if n.is_nan() { return write!(f, "nan") }
                write!(f, "{}", n)
            }
            Value::String(s)	=> write!(f, "{}", s),
            Value::Obj(rc)	=> write!(f, "{}", RefCell::borrow(rc)),
        }
    }
}


impl From<&Value> for Value {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Value::Null,
            Value::Bool(b) => Value::Bool(*b),
            Value::Number(n) => Value::Number(*n),
            Value::String(s) => Value::String(s.clone()),
            // Clone the inner Obj, not the Rc<RefCell<Obj>>
            Value::Obj(obj) => {
                let copy = Obj::from(obj.borrow().clone());
                Value::Obj(Rc::new(RefCell::new(copy)))
            }
        }
    }
}

