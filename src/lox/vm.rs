
use super::callframe::CallFrame;
use super::stack::Stack;
use super::value::Value;
//use super::obj::Obj;
use super::closure::Closure;
use super::function::Function;
use super::scanner::Scanner;
use super::tokenizer::Tokenizer;
use super::parser::Parser;
use super::compiler::Compiler;


//#[allow(dead_code)]
pub struct VM {
  callframes: Vec<CallFrame>,
  stack: Stack<Value>,
  //objects: Vec<Obj>,
}


impl VM {
    pub fn new() -> VM {
        VM {
            callframes: 	vec![],
            stack: 		Stack::new(), 
            //objects: 		vec![],
        }
    }
}


//#[allow(unused_mut)]
impl VM {
    pub fn compile(&mut self, code: &str) -> Result<(), String> {
        println!("VM.compile() code={}", code);
        
        
        let scanner = Scanner::str(code);
        let tokenizer = Tokenizer::new(scanner);
        let mut function = Function::new("__main__", 0);    
        let mut compiler = Compiler::new(function);
        
        let mut parser = Parser::new(tokenizer, compiler);
        let result = parser.parse();
        
        compiler = parser.take_compiler();
        function = compiler.take_function();
        
        match result {
            Ok(()) => {
                return self.setup_initial_callframe(function);
            }
            Err(msg) => {
                return Err(msg);
            }
        }
    }
}


impl VM {
    pub fn execute(&mut self) {
        //println!("VM.execute()");
        
    }
}


//#[allow(dead_code)]
impl VM {
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Value {
        return self.stack.pop();
    }
    fn setup_initial_callframe(&mut self, function: Function) -> Result<(), String>{
        let closure = Closure::new(function);
        self.push(Value::closure(closure));
        self.call_value(0); // Main function takes zero arguments
        return Ok(());
    }
    
    // Stack: Value to be called
    fn call_value(&mut self, _args: u8) {
        let value = self.pop();
        let callframe = CallFrame::new(value.as_rc_object());
        self.callframes.push(callframe);
    }
}


impl Drop for VM {
    fn drop(&mut self) {
        //println!("VM.drop()");
    }
}






