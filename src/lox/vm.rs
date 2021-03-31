
use super::callframe::CallFrame;
use super::stack::Stack;
use super::value::Value;
//use super::obj::Obj;
use super::constants::Constants;
use super::closure::Closure;
use super::function::Function;
use super::scanner::Scanner;
use super::tokenizer::Tokenizer;
use super::parser::{Parser, ParserInput, ParserOutput};
use super::compiler::Compiler;
use super::opcode::OpCode;


//#[allow(dead_code)]
pub struct VM {
  callframes: Vec<CallFrame>,
  stack: Stack<Value>,
  constants: Constants,
  //objects: Vec<Obj>,
}


impl VM {
    pub fn new() -> VM {
        VM {
            callframes: 	vec![],
            stack: 		Stack::new(), 
            constants:		Constants::new(),
            //objects: 		vec![],
        }
    }
}


//#[allow(unused_mut)]
impl VM {
    pub fn compile(&mut self, code: &str) -> Result<(), String> {
        println!("VM.compile() code={}", code);
        
        let scanner = Scanner::str(code);
        let mut tokenizer = Tokenizer::new(scanner);
        let mut function = Function::new("__main__", 0);    
        let mut compiler = Compiler::new(function);

        let mut parser = Parser::new();        
        //let mut parser = Parser::new(tokenizer, compiler);
        //parser.give_constants(constants);
        let mut input = ParserInput {
            tokenizer: &mut tokenizer,
        };
        let mut output = ParserOutput {
            compiler: &mut compiler,
            constants: &mut self.constants,
        };
        let result = parser.parse(&mut input, &mut output);
        
        //self.constants = Some(parser.take_constants());
        //compiler = parser.take_compiler();
        function = compiler.take_function();
        
        println!("VM.compile() function={:?}", function);
        
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
        println!("VM.execute()");
        
        loop {
            let ip = self.read_callframe().ip();
            let opcode = self.callframe().read_op();
            match opcode {
                None => {
                    println!("VM.execute() completed successfully");
                    return;
                }
                Some(opcode) => {
            
                    let result;

                    match opcode {
                        OpCode::Return 		=> result = self.opcode_return(),

                        OpCode::Const8 		=> result = self.opcode_const8(),
                        OpCode::Const16 	=> result = self.opcode_const16(),
                        OpCode::Const32 	=> result = self.opcode_const32(),

                        OpCode::Add 		=> result = self.opcode_add(),
                        OpCode::Sub 		=> result = self.opcode_sub(),
                        OpCode::Mul 		=> result = self.opcode_mul(),
                        OpCode::Div 		=> result = self.opcode_div(),
                        OpCode::Mod 		=> result = self.opcode_mod(),

                        OpCode::Pop 		=> result = self.opcode_pop(),
                        OpCode::PopN 		=> result = self.opcode_popn(),

                        OpCode::BAD 		=> result = self.opcode_bad(),
                    }
                    
                    // On error, dump message and return
                    match result {
                        Ok(()) => {
                        }
                        Err(message) => {
                            eprintln!(
                                "{} at ip={}\n{:?}", 
                                message,
                                ip, 
                                self.read_callframe().read_function()
                            );
                            return;
                        }
                    }
                }
            }
        }
    }
    
    pub fn callframe(&mut self) -> &mut CallFrame {
        return self.callframes.last_mut().unwrap();
    }

    pub fn read_callframe(&self) -> &CallFrame {
        return self.callframes.last().unwrap();
    }

    pub fn opcode_return(&mut self) -> Result<(), String> {
        Err("OpCode not implemented".to_string())
    }

    pub fn opcode_const8(&mut self) -> Result<(), String> {
        let constant = self.callframe().read_byte() as usize;
        let value = self.constants.value_by_index(constant);
        self.push(value);
        Ok(())
    }
    
    pub fn opcode_const16(&mut self) -> Result<(), String> {
        Err("OpCode not implemented".to_string())
    }

    pub fn opcode_const32(&mut self) -> Result<(), String> {
        Err("OpCode not implemented".to_string())
    }
    
    pub fn opcode_add(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.add(&b);
        match res {
            Ok(value) => {
                self.push(value);
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }
    
    pub fn opcode_sub(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.subtract(&b);
        match res {
            Ok(value) => {
                self.push(value);
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }
    
    pub fn opcode_mul(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.multiply(&b);
        match res {
            Ok(value) => {
                self.push(value);
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }
    
    pub fn opcode_div(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.divide(&b);
        match res {
            Ok(value) => {
                self.push(value);
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }
    
    pub fn opcode_mod(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let res = a.modulo(&b);
        match res {
            Ok(value) => {
                self.push(value);
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }
    
    pub fn opcode_pop(&mut self) -> Result<(), String> {
        let value = self.pop();
        println!("POP = {:?}", value);
        Ok(())
    }
    
    pub fn opcode_popn(&mut self) -> Result<(), String> {
        Err("OpCode not implemented".to_string())
    }
    
    pub fn opcode_bad(&mut self) -> Result<(), String> {
        Err("Bad OpCode".to_string())
    }
}


//#[allow(dead_code)]
impl VM {
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Value {
        let value = self.stack.pop();
        return value;
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






