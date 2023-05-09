

use std::rc::Rc;
use std::cell::RefCell;

// This is the runtime representation of an upvalue.
// Unlike local variables, each upvalue must be accessible 
// for read/write from multiple closures at the same time.

// This object can be cloned, all copies will point to the same
// instance of None (if open) or Some<T> (if closed)
#[derive(Debug, Clone)]
pub struct Upvalue<T> {
    addr:       usize,	// Absolute stack position
    value:      Rc<RefCell<Option<T>>>, 
}

impl<T: Clone + std::fmt::Display> Upvalue<T> {
    
    pub fn new(addr: usize) -> Self {
        //println!("Upvalue created: addr={}", addr);
        Self {
            addr,
            value:      Rc::new(RefCell::new(None)), 
        }
    }
    
    pub fn addr(&self) -> usize {
        return self.addr;
    }
    
    pub fn get(&self) -> Option<T> {
        return self.value.borrow().clone();
    }
    
    pub fn close(&mut self, value: T) {
        println!("Upvalue.close() addr={} close with value={}", self.addr, value);
        *self.value.borrow_mut() = Some(value);
    }
    
    pub fn is_closed(&self) -> bool {
        match &self.get() {
            Some(_)	=> true,
            None	=> false,
        }
    }
    
}


impl<T> std::fmt::Display for Upvalue<T> 
    where T: Clone + core::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let inner = &self.get();
        match &inner {
            Some(value) => {
                write!(f, "Upvalue(addr={}, value=Some({}), closed=Yes)", self.addr, value)
            }
            None => {
                write!(f, "Upvalue(addr={}, value=None, closed=No)", self.addr)
            }
        }
    }
}
