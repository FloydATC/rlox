

use crate::lox::VM;
use super::*;


#[test]
fn unshift_string_1() {
    let code = "var s=''; s.unshift('foo'); exit s == 'foo';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("unshift", unshift, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn unshift_string_2() {
    let code = "var s=''; s.unshift('bar'); s.unshift('foo'); exit s == 'foobar';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("unshift", unshift, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn unshift_string_3() {
    let code = "var s=''; s.unshift('baz'); s.unshift('bar'); s.unshift('foo'); exit s == 'foobarbaz';";
    let mut vm = VM::new();
    vm.native_callables().insert_method("unshift", unshift, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn unshift_array_1() {
    let code = "var a=[]; a.unshift('foo'); exit a == ['foo'];";
    let mut vm = VM::new();
    vm.native_callables().insert_method("unshift", unshift, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn unshift_array_2() {
    let code = "var a=[]; a.unshift('bar'); a.unshift('foo'); exit a == ['foo','bar'];";
    let mut vm = VM::new();
    vm.native_callables().insert_method("unshift", unshift, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn unshift_array_3() {
    let code = "var a=[]; a.unshift('baz'); a.unshift('bar'); a.unshift('foo'); exit a == ['foo','bar','baz'];";
    let mut vm = VM::new();
    vm.native_callables().insert_method("unshift", unshift, 1);
    let result = compile_and_execute_using(vm, code);
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), 1);
}

