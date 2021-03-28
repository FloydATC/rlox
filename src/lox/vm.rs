
use super::callframe::CallFrame;
use super::stack::Stack;
use super::value::Value;
use super::obj::Obj;

#[allow(dead_code)]
pub struct VM {
  callframes: Vec<CallFrame>,
  stack: Stack<Value>,
  objects: Vec<Obj>,
}


impl VM {
    pub fn new() -> VM {
        VM {
            callframes: 	vec![],
            stack: 		Stack::new(), 
            objects: 		vec![],
        }
    }
}


impl VM {
    pub fn compile(&mut self, _code: &str) {
        //println!("VM.compile() code={}", code);
    }
}


impl VM {
    pub fn execute(&mut self) {
        //println!("VM.execute()");
    }
}


impl Drop for VM {
    fn drop(&mut self) {
        //println!("VM.drop()");
    }
}






