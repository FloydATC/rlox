

use crate::lox::RuntimeError;


use super::compile_and_execute;


fn generate_error(code: &str) -> RuntimeError {
    match compile_and_execute(code) {
        Ok(rc) => panic!("Unexpectedly returned rc={}", rc),
        Err(error) => return error,
    }
}


#[test]
fn main() {
    let code = "0.invalid;";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x00000004 at line 1 char 1 of test",
    ]);
}

#[test]
fn anon_scope() {
    let code = "{ 0.invalid; }";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x00000004 at line 1 char 1 of test",
    ]);
}

#[test]
fn nested_anon_scopes() {
    let code = "{ { 0.invalid; } }";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x00000004 at line 1 char 1 of test",
    ]);
}

#[test]
fn function() {
    let code = "fun f1() { 0.invalid; } f1();";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x00000008 at line 1 char 1 of test",
        "f1:0x00000004 at line 1 char 5 of test",
    ]);
}

#[test]
fn nested_functions() {
    let code = "fun f1() { fun f2() { 0.invalid; } f2(); } f1();";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x00000008 at line 1 char 1 of test",
        "f1:0x00000006 at line 1 char 5 of test",
        "f2:0x00000004 at line 1 char 16 of test",
    ]);
}

#[test]
fn returned_function() {
    let code = "fun f1() { fun f2() { 0.invalid; } return f2; } var fn=f1(); fn();";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x0000000e at line 1 char 1 of test",
        "f2:0x00000004 at line 1 char 16 of test",
    ]);
}

#[test]
fn class_initializer() {
    let code = "class c1 { init() { 0.invalid; } } var i1=c1();";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x0000000f at line 1 char 1 of test",
        "init:0x00000004 at line 1 char 12 of test",
    ]);
}

#[test]
fn class_method() {
    let code = "class c1 { m1() { 0.invalid; } } var i1=c1(); i1.m1();";
    let error = generate_error(code);
    let trace = error.get_stack_trace();
    assert_eq!(trace.as_ref(), vec![
        "__main__:0x00000017 at line 1 char 1 of test",
        "m1:0x00000004 at line 1 char 12 of test",
    ]);
}

