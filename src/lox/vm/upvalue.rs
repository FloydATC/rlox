

use std::rc::Rc;
use std::cell::RefCell;

// This is the runtime representation of an upvalue.
// Unlike local variables, each upvalue must be accessible 
// for read/write from multiple closures at the same time.

// Upvalues make absolutely no sense to me, which means
// the following code may also make absolutely no sense.
// On top of this I'm trying to learn Rust.
#[derive(Debug, Clone)]
pub struct Upvalue<T> {
    slot:       usize,
    value:      Rc<RefCell<T>>, 
    // closed: // Not sure what exact purpose this serves. GC only?
}

impl<T: Clone + std::fmt::Display> Upvalue<T> {
    
    pub fn new(slot: usize, value: T) -> Self {
        //println!("Upvalue created: slot={} value={}", slot, value);
        Self {
            slot,
            value:      Rc::new(RefCell::new(value)), 
        }
    }
    
    pub fn slot(&self) -> usize {
        return self.slot;
    }
    
    pub fn get(&self) -> T {
        return self.value.borrow().clone();
    }
    
//    pub fn set(&mut self, value: T) {
//        println!("Upvalue.set() value changed from {} to {}", self.value.borrow(), value);
//        *self.value.borrow_mut() = value;
//    }
    
}


impl<T> std::fmt::Display for Upvalue<T> 
    where T: Clone + core::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Upvalue(slot={}, value={})", self.slot, self.get())
    }
}
