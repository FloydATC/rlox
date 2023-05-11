

use super::compile_and_execute;


// Closures
#[test]
fn vm_closure_getupvalue_1() {
    let code = "fun mk() { var a = 123; fun c() { return a; } return c; } var c=mk(); exit c();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 123);
}

#[test]
fn vm_closure_getupvalue_2() {
    let code = "fun mk(v) { fun c() { return v; } return c; } var a = mk(1); var b = mk(2); exit a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_closure_getupvalue_3() {
    let code = "fun mk(v) { fun c() { return v; } return c; } var a = mk(1); var b = mk(2); exit b();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_closure_setupvalue_1() {
    let code = "fun a() { var x = 123; fun b() { x = 234; } b(); exit x; } a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 234);
}

#[test]
fn vm_closure_setupvalue_2() {
    let code = "fun a() { var x = 123; fun b() { x = 234; x=x*2; } b(); exit x; } a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 468);
}

#[test]
fn vm_closure_setupvalue_3() {
    let code = "fun c(y) { exit y; } fun a() { var x = 123; fun b() { x = 234; x=x*2; c(x); } b(); } a();";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 468);
}

