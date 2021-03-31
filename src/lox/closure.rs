
use super::function::Function;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Closure {
    function: Function,
}


#[allow(dead_code)]
impl Closure {
    pub fn new(function: Function) -> Closure {
        Closure { function }
    }
    pub fn function(&self) -> &Function {
        &self.function
    }
}
