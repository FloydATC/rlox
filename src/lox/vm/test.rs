

use super::RuntimeError;
use crate::lox::Compiler;
use super::VM;


mod arrays;
mod classes;
mod closures;
mod expressions;
mod for_loops;
mod functions;
mod globals;
mod if_statement;
mod literals;
mod locals;
mod math;
mod numbers;
mod return_statement;
mod stack_trace;
mod while_loops;


fn compile_and_execute(code: &str) -> Result<i32, RuntimeError> {
    let builder = Compiler::new();
    let reader = std::io::Cursor::new(code);
    match builder.compile("test", reader) {
        Ok(bytecode) => {
            match VM::new().execute(&bytecode) {
                Ok(rc) => {
                    println!("Execute returned rc={}", rc);
                    return Ok(rc);
                }
                Err(error) => {
                    println!("Execute failed: {}", error.get_message());
                    for line in error.get_stack_trace() { println!("  {}", line); }
                    return Err(error);
                }
            }
        }
        Err(error) => panic!("Compile failed: {}", error),
    }
}


#[test]
fn vm_new() {
    let _vm = VM::new();
}

#[test]
fn vm_emptystring() {
    let code = "";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
}

// 'exit' statement
#[test]
fn vm_exit_null() {
    let code = "exit;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_exit_zero() {
    let code = "exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_exit_one() {
    let code = "exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}





