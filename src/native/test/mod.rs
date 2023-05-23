

mod len;
mod pop;
mod push; 
mod shift;
mod unshift;


use crate::lox::{Compiler, VM, RuntimeError};


pub use super::*;


// Convenience function used for testing native methods and functions
fn compile_and_execute_using(mut vm: VM, code: &str) -> Result<i32, RuntimeError> {
    let compiler = Compiler::new();
    let reader = std::io::Cursor::new(code);
    match compiler.compile("test", reader) {
        Err(compile_error) => panic!("Compile failed unexpectedly: {}", compile_error),
        Ok(bytecode) => {
            match vm.execute(&bytecode) {
                Err(runtime_error) => {
                    eprintln!("{}", runtime_error);
                    return Err(runtime_error);
                },
                Ok(rc) => {
                    println!("rc={}", rc);
                    return Ok(rc);
                },
            }
        }
    }
}
