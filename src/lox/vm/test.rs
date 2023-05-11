

use super::RuntimeError;
use crate::lox::Builder;
use super::VM;


mod classes;
mod closures;
mod expressions;
mod functions;
mod globals;
mod if_statement;
mod literals;
mod locals;
mod math;
mod numbers;
mod return_statement;
mod while_loops;


fn compile_and_execute(code: &str) -> Result<i32, RuntimeError> {
    let builder = Builder::new();
    let reader = std::io::Cursor::new(code);
    match builder.compile(reader) {
        Ok(bytecode) => return VM::new().execute(&bytecode),
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





