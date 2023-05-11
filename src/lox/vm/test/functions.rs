

use super::compile_and_execute;


// Functions
#[test]
fn vm_fun_empty() {
    let code = "fun f() {} exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_empty_with_var() {
    let code = "fun f() { var a; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_empty_with_var_defined_1() {
    let code = "fun f() { var a=123; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_empty_with_var_defined_2() {
    let code = "var a=1; fun f() { var a=123; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_1() {
    let code = "fun f(a) { } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_2() {
    let code = "fun f(a,b) { } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_3() {
    let code = "fun f(a,b,c) { } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_fun_args_4() {
    let code = "fun f(a,b,c) { exit a+b+c; } f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn vm_fun_args_5() {
    let code = "var a=10; fun f(a,b,c) { exit a+b+c; } f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

#[test]
fn vm_fun_args_6() {
    let code = "fun f(a,b,c) { exit a+b+c; } var a=10; f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}
