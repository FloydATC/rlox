

use crate::lox::VM;
use super::*;


#[test]
fn shift_string_empty() {
    let code = "var s=''; s.shift();";
    let mut vm = VM::new();
    vm.native_callables().insert_method("shift", shift, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_err(), true);
}

#[test]
fn shift_string_1() {
    let code = "var s='abc'; exit s.shift() == 'a';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("shift", shift, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn shift_string_2() {
    let code = "var s='abc'; s.shift(); exit s.shift() == 'b';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("shift", shift, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn shift_string_3() {
    let code = "var s='abc'; s.shift(); s.shift(); exit s.shift() == 'c';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("shift", shift, 0);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}
