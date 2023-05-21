

use super::Function;
use super::Globals;
use super::Value;


#[derive(Debug)]
pub struct ByteCode {
    main_: Function,
    globals_: Globals<Value>,
}


impl ByteCode {

    pub fn new(main: Function, globals: Globals<Value>) -> Self {
        ByteCode {
            main_: main, 
            globals_: globals,
        }
    }


    pub fn main(&self) -> &Function {
        return &self.main_;
    }


    pub fn globals(&self) -> &Globals<Value> {
        return &self.globals_;
    }

}