

use super::test;


#[test]
fn void_empty() {
    let code = "[];";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 0);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_empty() {
    let code = "var a=[];";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_one_number() {
    let code = "var a=[1];";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_two_numbers() {
    let code = "var a=[1,2];";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_three_numbers() {
    let code = "var a=[1,2,3];";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_three_strings() {
    let code = "var a=['foo', 'bar', 'baz'];";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_three_selves_in_global() {
    let code = "var a=[a,a,a];"; // Global variable auto-initialized as null
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_three_selves_in_local() {
    let code = "fun f() { var a=[a,a,a]; }"; // Local variable can't initialize itself
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    assert_eq!(error.get_message(), "Can not read local variable in its own initializer");
}

#[test]
fn assignment_references_same_object() {
    let code = "var a=[1,2,3]; var alias=a;"; // Copy all elements
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn empty_subscript_copies_array() {
    let code = "var a=[1,2,3]; var copy=a[];"; // Copy all elements
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn single_subscript_returns_value() {
    let code = "var a=[1,2,3]; var element=a[0];"; // Take single value
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn multi_subscript_returns_array() {
    let code = "var a=[1,2,3]; var b=a[0,1];"; // Copy multiple values into new array
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn invalid_subscript_is_runtime_error() {
    let code = "var a=[1,2,3]; var fail=a[-1];"; // Bad subscript
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 2);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test]
fn assign_to_single_subscript() {
    let code = "var a=[1,2,3]; a[0]=123;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test] 
fn assign_single_to_array_subscript() {
    let code = "var a=[1,2,3,4,5]; a[1,2,3]=123;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}

#[test] 
fn assign_array_to_array_subscript() {
    let code = "var a=[1,2,3,4,5]; a[1,2,3]=['foo','bar','baz'];";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
    let bytecode = res.unwrap();
    assert_eq!(bytecode.globals().count(), 1);
    assert_eq!(bytecode.main().clone().kind().is_toplevel(), true);
}


