

use crate::lox::VM;
use super::*;


#[test]
fn push_string_1() {
    let code = "var s=''; s.push('foo'); exit s == 'foo';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("push", push, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn push_string_2() {
    let code = "var s=''; s.push('foo'); s.push('bar'); exit s == 'foobar';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("push", push, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn push_string_3() {
    let code = "var s=''; s.push('foo'); s.push('bar'); s.push('baz'); exit s == 'foobarbaz';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("push", push, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn push_array_1() {
    let code = "var a=[]; a.push('foo'); exit a == ['foo'];";
    let mut vm = VM::new();
    vm.native_callables().insert_method("push", push, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn push_array_2() {
    let code = "var a=[]; a.push('foo'); a.push('bar'); exit a == ['foo','bar'];";
    let mut vm = VM::new();
    vm.native_callables().insert_method("push", push, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn push_array_3() {
    let code = "var a=[]; a.push('foo'); a.push('bar'); a.push('baz'); exit a == ['foo','bar','baz'];";
    let mut vm = VM::new();
    vm.native_callables().insert_method("push", push, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

