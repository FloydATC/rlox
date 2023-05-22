

use super::NativeFn;


#[derive(Clone)]
pub struct NativeCallable {
    name: String,
    callable: NativeFn,
    arity: usize,
}


impl NativeCallable {

    pub fn new(name: String, callable: NativeFn, arity: usize) -> Self {
        NativeCallable {
            name, 
            callable, 
            arity, 
        }
    }


    pub fn name(&self) -> &str {
        return &self.name;
    }


    pub fn arity(&self) -> usize {
        return self.arity;
    }


    pub fn callable(&self) -> &NativeFn {
        return &self.callable;
    }

}


impl std::fmt::Debug for NativeCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeCallable").field("name", &self.name).field("arity", &self.arity).finish()
    }
}
