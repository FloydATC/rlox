

use crate::lox::common::Function;
use crate::lox::vm::{Class, Instance, Method, NativeMethod};
use crate::lox::common::Closure;
use crate::lox::common::{Array, Value, ValueIterator};
use super::NativeCallable;


#[allow(dead_code)]
#[derive(Debug)]
pub enum Obj {
    Array(Array),
    Function(Function),
    Class(Class),
    Closure(Closure),
    Instance(Instance),
    Iterator(ValueIterator),
    Method(Method),
    Native(NativeCallable),
    NativeMethod(NativeMethod),
    String(String),
}


//#[allow(dead_code)]
impl Obj {
    // ======== Constructors ========

    pub fn array(a: Array) -> Obj {
        Obj::Array(a)
    }
    pub fn function(f: Function) -> Obj {
        Obj::Function(f)
    }
    pub fn class(c: Class) -> Obj {
        Obj::Class(c)
    }
    pub fn closure(c: Closure) -> Obj {
        Obj::Closure(c)
    }
    pub fn instance(i: Instance) -> Obj {
        Obj::Instance(i)
    }
    pub fn iterator(i: ValueIterator) -> Obj {
        Obj::Iterator(i)
    }
    pub fn method(m: Method) -> Obj {
        Obj::Method(m)
    }


    pub fn native(nc: NativeCallable) -> Obj {
        Obj::Native(nc)
    }


    pub fn native_method(nc: NativeMethod) -> Obj {
        Obj::NativeMethod(nc)
    }


    pub fn string(s: &str) -> Obj {
        Obj::String(String::from(s))
    }

}

#[allow(dead_code)]
impl Obj {
    // ======== Variant checks ========

    pub fn is_array(&self) -> bool {
        match self {
            Obj::Array(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Obj::Function(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_class(&self) -> bool {
        match self {
            Obj::Class(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_closure(&self) -> bool {
        match self {
            Obj::Closure(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_instance(&self) -> bool {
        match self {
            Obj::Instance(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_iterator(&self) -> bool {
        match self {
            Obj::Iterator(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_method(&self) -> bool {
        match self {
            Obj::Method(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_native(&self) -> bool {
        match self {
            Obj::Native(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_native_method(&self) -> bool {
        match self {
            Obj::NativeMethod(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Obj::String(_) 	=> true,
            _			=> false,
        }
    }

}


// Subscript

impl Obj {

    pub fn can_get(&self) -> bool {
        match self {
            Obj::Array(_) => true,
            Obj::Class(_) => true,
            Obj::Instance(_) => true,
            Obj::String(_) => true,
            _ => false,
        }
    }


    pub fn get(&self, key: &Value) -> Option<Value> {
        match self {
            Obj::Array(a) => {
                if !key.is_number() { return None }
                let index = key.as_number().floor();
                if index < 0.0 || index >= a.len() as f64 { return None }
                return a.get(index as usize).cloned();
            }
            Obj::Class(c) => if key.is_string() { c.get(key.as_string().as_str()).cloned() } else { None },
            Obj::Instance(i) => if key.is_string() { i.get(key.as_string().as_str()).cloned() } else { None },
            Obj::String(s) => {
                if !key.is_number() { return None }
                let index = key.as_number().floor();
                if index < 0.0 || index >= s.chars().count() as f64 { return None }
                match s.chars().nth(index as usize) {
                    Some(cp) => return Some(Value::string(String::from(cp).as_str())),
                    None => return None,
                }
            }
            _ => None,
        }
    }


    pub fn can_set(&self) -> bool {
        match self {
            Obj::Array(_) => true,
            Obj::Class(_) => false, // MUST NOT modify a class after declaration!
            Obj::Instance(_) => true,
            Obj::String(_) => true,
            _ => false,
        }
    }


    pub fn set(&mut self, key: &Value, value: Value) -> Result<(), String> {
        match self {
            Obj::Array(a) => {
                if !key.is_number() { return Err(format!("Invalid subscript '{}' for {}", key, a)) }
                let index = key.as_number().floor();
                if index < 0.0 || index >= a.len() as f64 { return Err(format!("Bad subscript {} for {}", key, a)) }
                return a.set(index as usize, value);
            }         
            Obj::Instance(i) => if key.is_string() { Ok(i.set(key.as_string().as_str(), value)) } else { 
                return Err(format!("Invalid subscript '{}' for {}", key, i)); 
            },
            Obj::String(s) => {
                // Check the index (note that index >= s.chars() will be checked further down so we don't walk the string twice)
                if !key.is_number() { return Err(format!("Invalid subscript index '{}' for {}", key, self)) }
                let index = key.as_number().floor();
                if index < 0.0 { return Err(format!("Bad subscript index {} for {}", index, s)) }
                let index = index as usize;
                // The value must be a number (=unicode codepoint) or a string containing a single (unicode) character
                let cp = if value.is_number() {
                    match char::from_u32(value.as_number().floor() as u32) {
                        None => return Err(format!("Invalid utf-8 codepoint {}", value.as_number().floor() as u32)),
                        Some(cp) => cp,
                    }
                } else if value.is_string() && value.as_string().chars().count() == 1 {
                    value.as_string().chars().nth(0).unwrap()
                } else {
                    return Err(format!("Can not set '{}' as a string character", value));
                };
                // If we made it this far, we're ready to split the string, replace a char and reassemble
                let mut chars = s.chars().collect::<Vec<char>>();
                if index >= chars.len() { return Err(format!("Bad subscript {} for '{}'", index, s)) };
                chars[index] = cp;
                *s = chars.into_iter().collect();
                Ok(())
            }
            _ => return Err(format!("Can't .set() on {}", self)),
        }
    }

}

//#[allow(dead_code)]
impl Obj {
    // ======== Property checks ========

    pub fn is_truthy(&self) -> bool {
        match self {
            Obj::String(s) => s != "",
            _ => true,	// All other objects are truthy (for now)
        }
    }

}


//#[allow(dead_code)]
impl Obj {
    // ======== Getters ========

    pub fn as_array(&self) -> &Array {
        match self {
            Obj::Array(a) => return a,
            _ => panic!("{:?} is not an Array Object", self),
        }
    }

    pub fn as_array_mut(&mut self) -> &mut Array {
        match self {
            Obj::Array(a) => return a,
            _ => panic!("{:?} is not an Array Object", self),
        }
    }

    pub fn as_function(&self) -> &Function {
        match self {
            Obj::Function(f) => return f,
            _ => panic!("{:?} is not a Function Object", self),
        }
    }

    pub fn as_class(&self) -> &Class {
        match self {
            Obj::Class(c) => return c,
            _ => panic!("{:?} is not a Class Object", self),
        }
    }

    pub fn as_class_mut(&mut self) -> &mut Class {
        match self {
            Obj::Class(c) => return c,
            _ => panic!("{:?} is not a Class Object", self),
        }
    }

    pub fn as_closure(&self) -> &Closure {
        match self {
            Obj::Closure(c) => return c,
            _ => panic!("{:?} is not a Closure Object", self),
        }
    }

    pub fn as_closure_mut(&mut self) -> &mut Closure {
        match self {
            Obj::Closure(c) => return c,
            _ => panic!("{:?} is not a Closure Object", self),
        }
    }

    pub fn as_instance(&self) -> &Instance {
        match self {
            Obj::Instance(i) => return i,
            _ => panic!("{:?} is not an Instance Object", self),
        }
    }

    pub fn as_instance_mut(&mut self) -> &mut Instance {
        match self {
            Obj::Instance(i) => return i,
            _ => panic!("{:?} is not an Instance Object", self),
        }
    }

    pub fn as_iterator(&self) -> &ValueIterator {
        match self {
            Obj::Iterator(i) => return i,
            _ => panic!("{:?} is not a ValueIterator Object", self),
        }
    }

    pub fn as_iterator_mut(&mut self) -> &mut ValueIterator {
        match self {
            Obj::Iterator(i) => return i,
            _ => panic!("{:?} is not a ValueIterator Object", self),
        }
    }

    pub fn as_method(&self) -> &Method {
        match self {
            Obj::Method(m) => return m,
            _ => panic!("{:?} is not a Method Object", self),
        }
    }

    pub fn as_native(&self) -> &NativeCallable {
        match self {
            Obj::Native(nc) => return nc,
            _ => panic!("{:?} is not a NativeCallable Object", self),
        }
    }

    pub fn as_native_method(&self) -> &NativeMethod {
        match self {
            Obj::NativeMethod(nm) => return nm,
            _ => panic!("{:?} is not a NativeMethod Object", self),
        }
    }

    pub fn as_string(&self) -> &String {
        match self {
            Obj::String(s) => return s,
            _ => panic!("{:?} is not a String Object", self),
        }
    }

    pub fn as_string_mut(&mut self) -> &mut String {
        match self {
            Obj::String(s) => return s,
            _ => panic!("{:?} is not a String Object", self),
        }
    }

}


impl Obj {

    pub fn len(&self) -> Option<usize> {
        match self {
            Obj::Array(a) => Some(a.len()),
            Obj::String(s) => Some(s.chars().count()),
            _ => None,
        }
    }

    // Arithmetics

    pub fn append_value(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (_, Value::Obj(_)) => {
                return self.append(other.as_obj());
            }
            (Obj::Array(a), _) => {
                // Add simple value
                let mut copy = a.clone();
                copy.push(other.clone());
                return Ok(Value::array(copy));
            }
            _ => {}
        }
        return Err(format!("Can not add operands {} and {}", &self, &other));
    }

    pub fn append(&self, other: std::cell::Ref<'_, Obj>) -> Result<Value, String> {
        match self {
            Obj::Array(a) => {
                if other.is_array() {
                    let mut copy = a.clone();
                    copy.extend_from_slice(other.as_array().as_slice());
                    return Ok(Value::array(copy));
                }
            }
            Obj::String(a) => {
                if other.is_string() {
                    let string = format!("{}{}", a, other.as_string());
                    return Ok(Value::string(&string));
                }
            }
            _ => {}
        }
        return Err(format!("Can not add operands {} and {}", &self, &other));
    }

    pub fn prepend_value(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (_, Value::Obj(_)) => {
                return self.prepend(other.as_obj());
            }
            (Obj::Array(a), _) => {
                // Add simple value
                let mut copy = Array::new();
                copy.push(other.clone());
                copy.extend_from_slice(a.as_slice());
                return Ok(Value::array(copy));
            }
            _ => {}
        }
        return Err(format!("Can not append value {} to {}", &other, &self));
    }

    pub fn prepend(&self, other: std::cell::Ref<'_, Obj>) -> Result<Value, String> {
        match self {
            Obj::Array(a) => {
                if other.is_array() {
                    let mut copy = other.as_array().clone();
                    copy.extend_from_slice(a.as_slice());
                    return Ok(Value::array(copy));
                }
            }
            _ => {}
        }
        return Err(format!("Can not prepend value {} to {}", &other, &self));
    }

/*
    pub fn subtract_value(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Obj::Array(a), _) => {
                // Subtract number value from array = pop
                if other.is_number() {
                    let count = other.as_number().floor();
                    if count < 0.0 { return Err(format!("Can not subtract negative number {} from array", other)) }
                    if count > a.len() as f64 { return Err(format!("Can not subtract {} from array of length {}", other, a.len())) }
                    let copy = Array::from(&a.as_slice()[..a.len()-count as usize]);
                    return Ok(Value::array(copy));
                }
            }
            _ => {}
        }
        return Err(format!("Can not subtract value {} from {}", &other, &self));
    }

    pub fn subtract_from_value(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Obj::Array(a), _) => {
                // Subtract array from number value = shift
                if other.is_number() {
                    let count = other.as_number().floor();
                    if count < 0.0 { return Err(format!("Can not subtract negative number {} from array", other)) }
                    if count > a.len() as f64 { return Err(format!("Can not subtract {} from array of length {}", other, a.len())) }
                    let copy = Array::from(&a.as_slice()[count as usize..]);
                    return Ok(Value::array(copy));
                }
            }
            _ => {}
        }
        return Err(format!("Can not subtract {} from value {}", &self, &other));
    }
*/

    pub fn repeat(&self, count: usize) -> Result<Value, String> {
        match self {
            Obj::String(s) => {
                let string = s.repeat(count);
                return Ok(Value::string(&string));
            }
            _ => {}
        }
        return Err(format!("Can not multiply {}", &self));
    }

}

// ======== Traits ========

impl PartialEq for Obj {
    fn eq(&self, other: &Obj) -> bool { 
        match (self, other) {
            (Obj::Array(a), Obj::Array(b)) => {
                //println!("comparing Obj::Arrays");
                a.eq(b)
            }
            (Obj::String(a), Obj::String(b)) 	 => a.eq(b),
            // All other Obj types must be same object
            (Obj::Function(a), Obj::Function(b)) => std::ptr::eq(a, b),
            (Obj::Class(a), Obj::Class(b)) 	 => std::ptr::eq(a, b),
            (Obj::Closure(a), Obj::Closure(b))   => std::ptr::eq(a, b),
            (Obj::Instance(a), Obj::Instance(b)) => std::ptr::eq(a, b),
            (Obj::Iterator(a), Obj::Iterator(b)) => std::ptr::eq(a, b),
            (Obj::Method(a), Obj::Method(b)) 	 => std::ptr::eq(a, b),
            _ => false, // Obj types mismatch
        }
    }
}


impl std::cmp::PartialOrd for Obj {
    fn partial_cmp(&self, other: &Obj) -> Option<std::cmp::Ordering> {
        match (self, other) {
            _ => None, // Value types mismatch or can't be ordered
        }
    }
}


impl std::fmt::Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Obj::Array(ar) => {
                write!(f, "Obj::Array({})", ar)
            }
            Obj::Function(fu) => {
                write!(f, "Obj::Function({})", fu.name())
            }
            Obj::Class(cl) => {
                write!(f, "Obj::Class({})", cl.name())
            }
            Obj::Closure(cl) => {
                write!(f, "Obj::Closure({})", cl.name())
            }
            Obj::Instance(inst) => {
                write!(f, "Obj::Instance(class={})", inst.class_name())
            }
            Obj::Iterator(iter) => {
                write!(f, "Obj::Iterator({})", iter)
            }
            Obj::Method(m) => {
                write!(f, "Obj::Method({}.{})", m.receiver_class_name(), m.method_name())
            }
            Obj::Native(nc) => {
                write!(f, "Obj::Native({})", nc.name())
            }
            Obj::NativeMethod(nm) => {
                write!(f, "Obj::NativeMethod({}.{})", nm.receiver(), nm.method().as_native().name())
            }
            Obj::String(s)	=> write!(f, "{}", s), // Strings print without any decoration
        }
    }
}


impl From<&Obj> for Obj {
    fn from(other: &Obj) -> Obj {
        match other {
            Obj::Array(a) => Obj::Array(a.clone()),
            Obj::Function(f) => Obj::Function(f.clone()),
            Obj::Class(c) => Obj::Class(c.clone()),
            Obj::Closure(c) => Obj::Closure(c.clone()),
            Obj::Instance(i) => Obj::Instance(i.clone()),
            Obj::Iterator(i) => Obj::Iterator(i.clone()),
            Obj::Method(m) => Obj::Method(m.clone()),
            Obj::Native(nc) => Obj::Native(nc.clone()),
            Obj::NativeMethod(nm) => Obj::NativeMethod(nm.clone()),
            Obj::String(s) => Obj::String(s.clone()),
        }
    }
}


impl Clone for Obj {
    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone()
    }

    fn clone(&self) -> Self {
        Obj::from(self)
    }
}
