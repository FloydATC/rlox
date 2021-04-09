
//use std::rc::Rc;
//use std::ptr;

use super::function::Function;
use super::closure::Closure;
//use super::value::Value;


// This is the runtime representation of an Upvalue 
// For the compile time representation, see "upvalue.rs"
// The question is, do we really need this?
// clox keeps upvalues here to make GC easy. We don't GC.
//#[allow(dead_code)]
//#[derive(Debug)]
//pub struct Upvalue {
//    location:	Value,	// reference? Rc? Box?
//    closed:	bool,
//    next:	???,	// Option?
//}

//impl Upvalue {
//    fn new(v: Value) -> Self {
//        Self {
//            location:	v,
//            closed:	false,
//        }
//    }
//}


#[allow(dead_code)]
#[derive(Debug)]
pub enum Obj {
    String(String),
    Function(Function),
    Closure(Closure),
//    Upvalue(Upvalue),
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
//    pub fn upvalue(v: Value) -> Obj {
//        Obj::Upvalue(Upvalue::new(v))
//    }

//    pub fn as_string(&self) -> &str {
//        match self {
//            Obj::String(s) => return &s,
//            _ => panic!("{} is not a String Object"),
//        }
//    }

    pub fn is_string(&self) -> bool {
        match self {
            Obj::String(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Obj::Function(_) 	=> true,
            _			=> false,
        }
    }

    pub fn is_closure(&self) -> bool {
        match self {
            Obj::Closure(_) 	=> true,
            _			=> false,
        }
    }

    pub fn as_string(&self) -> &String {
        match self {
            Obj::String(s) => return &s,
            _ => panic!("{:?} is not a Function Object", self),
        }
    }

    pub fn as_function(&self) -> &Function {
        match self {
            Obj::Function(f) => return &f,
            _ => panic!("{:?} is not a Function Object", self),
        }
    }

    pub fn as_closure(&self) -> &Closure {
        match self {
            Obj::Closure(c) => return &c,
            _ => panic!("{:?} is not a Closure Object", self),
        }
    }
    pub fn as_closure_mut(&mut self) -> &mut Closure {
        match self {
            Obj::Closure(c) => return c,
            _ => panic!("{:?} is not a Closure Object", self),
        }
    }
}


impl PartialEq for Obj {
    fn eq(&self, other: &Obj) -> bool { 
        match (self, other) {
            (Obj::String(a), Obj::String(b))     => a == b,
            // Obj types other than Obj::String must be same object
            (Obj::Function(a), Obj::Function(b)) => std::ptr::eq(a, b),
            (Obj::Closure(a), Obj::Closure(b))   => std::ptr::eq(a, b),
            _ => false, // Obj types mismatch
        }
    }
}


impl std::cmp::PartialOrd for Obj {
    fn partial_cmp(&self, other: &Obj) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Obj::String(a), Obj::String(b)) => Some(a.cmp(b)),
            _ => None, // Value types mismatch
        }
    }
}


impl std::fmt::Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Obj::String(st) => {
                // Escape non-ascii and non-printable ascii chars
                write!(f, "Obj::String({:?})", st)
            }
            Obj::Function(fu) => {
                write!(f, "Obj::Function({})", fu.name())
            }
            Obj::Closure(cl) => {
                write!(f, "Obj::Closure({})", cl.function().name())
            }
//            Obj::Upvalue(v) => {
//                write!(f, "Obj::Upvalue({})", v.location)
//            }
        }
    }
}



