

use super::function::Function;
use super::class::Class;
use super::instance::Instance;
use super::method::Method;
use super::closure::Closure;


#[allow(dead_code)]
#[derive(Debug)]
pub enum Obj {
    Function(Function),
    Class(Class),
    Closure(Closure),
    Instance(Instance),
    Method(Method),
}


//#[allow(dead_code)]
impl Obj {
    // ======== Constructors ========

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
    pub fn method(m: Method) -> Obj {
        Obj::Method(m)
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

    pub fn is_method(&self) -> bool {
        match self {
            Obj::Method(_) 	=> true,
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

    pub fn as_class(&self) -> &Class {
        match self {
            Obj::Class(c) => return &c,
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

    pub fn as_instance(&self) -> &Instance {
        match self {
            Obj::Instance(i) => return &i,
            _ => panic!("{:?} is not an Instance Object", self),
        }
    }

    pub fn as_instance_mut(&mut self) -> &mut Instance {
        match self {
            Obj::Instance(i) => return i,
            _ => panic!("{:?} is not an Instance Object", self),
        }
    }

    pub fn as_method(&self) -> &Method {
        match self {
            Obj::Method(i) => return &i,
            _ => panic!("{:?} is not a Method Object", self),
        }
    }

}


// ======== Traits ========

impl PartialEq for Obj {
    fn eq(&self, other: &Obj) -> bool { 
        match (self, other) {
            // Obj types must be same object
            (Obj::Function(a), Obj::Function(b)) => std::ptr::eq(a, b),
            (Obj::Class(a), Obj::Class(b)) 	 => std::ptr::eq(a, b),
            (Obj::Closure(a), Obj::Closure(b))   => std::ptr::eq(a, b),
            (Obj::Instance(a), Obj::Instance(b)) => std::ptr::eq(a, b),
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
            Obj::Method(m) => {
                write!(f, "Obj::Method({}.{})", m.receiver_class_name(), m.method_name())
            }
        }
    }
}


