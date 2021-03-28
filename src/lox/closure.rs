
use super::function::Function;

#[allow(dead_code)]
pub struct Closure {
    function: Function,
}


#[allow(dead_code)]
impl Closure {
    pub fn new(function: Function) -> Closure {
        Closure { function }
    }
    pub fn function(&mut self) -> &Function {
        &self.function
    }
}
