

use crate::lox::VM;
use super::*;


#[test]
fn len_string_empty() {
    let code = "var s=''; exit s.len() == 0;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn len_string_1() {
    let code = "var s='a'; exit s.len() == 1;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn len_string_2() {
    let code = "var s='ab'; exit s.len() == 2;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn len_string_3() {
    let code = "var s='abc'; exit s.len() == 3;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn len_array_empty() {
    let code = "var a=[]; exit a.len() == 0;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn len_array_1() {
    let code = "var a=['foo']; exit a.len() == 1;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn len_array_2() {
    let code = "var a=['foo','bar']; exit a.len() == 2;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn len_array_3() {
    let code = "var a=['foo','bar','baz']; exit a.len() == 3;";
    let mut vm = VM::new();
    vm.native_callables().insert_method("len", len, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

