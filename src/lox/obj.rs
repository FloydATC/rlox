

use super::function::Function;
use super::closure::Closure;


#[allow(dead_code)]
#[derive(Debug)]
pub enum Obj {
    Function(Function),
    Closure(Closure),
}


//#[allow(dead_code)]
impl Obj {
    // ======== Constructors ========

    pub fn function(f: Function) -> Obj {
        Obj::Function(f)
    }
    pub fn closure(c: Closure) -> Obj {
        Obj::Closure(c)
    }
}

#[allow(dead_code)]
impl Obj {
    // ======== Variant checks ========

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
}

//#[allow(dead_code)]
impl Obj {
    // ======== Property checks ========

    pub fn is_truthy(&self) -> bool {
        match self {
            _			=> true,	// All objects are truthy (for now)
        }
    }
}


//#[allow(dead_code)]
impl Obj {
    // ======== Getters ========

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


// ======== Traits ========

impl PartialEq for Obj {
    fn eq(&self, other: &Obj) -> bool { 
        match (self, other) {
            // Obj types must be same object
            (Obj::Function(a), Obj::Function(b)) => std::ptr::eq(a, b),
            (Obj::Closure(a), Obj::Closure(b))   => std::ptr::eq(a, b),
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
            Obj::Function(fu) => {
                write!(f, "Obj::Function({})", fu.name())
            }
            Obj::Closure(cl) => {
                write!(f, "Obj::Closure({})", cl.name())
            }
        }
    }
}


