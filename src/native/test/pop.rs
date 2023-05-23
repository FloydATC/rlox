

use crate::lox::VM;
use super::*;


#[test]
fn pop_string_empty() {
    let code = "var s=''; s.pop();";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_err(), true);
}

#[test]
fn pop_string_1() {
    let code = "var s='abc'; exit s.pop() == 'c';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn pop_string_2() {
    let code = "var s='abc'; s.pop(); exit s.pop() == 'b';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn pop_string_3() {
    let code = "var s='abc'; s.pop(); s.pop(); exit s.pop() == 'a';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn pop_array_empty() {
    let code = "var a=[]; a.pop();";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_err(), true);
}

#[test]
fn pop_array_1() {
    let code = "var a=[123,456,789]; exit a.pop() == 789;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn pop_array_2() {
    let code = "var a=[123,456,789]; a.pop(); exit a.pop() == 456;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn pop_array_3() {
    let code = "var a=[123,456,789]; a.pop(); a.pop(); exit a.pop() == 123;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("pop", pop, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}
