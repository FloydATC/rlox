

use super::compile_and_execute;


// Return values
#[test]
fn vm_fun_return_implicit() {
    let code = "fun f(a,b,c) { var t=a+b+c; } exit f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_fun_return_null() {
    let code = "fun f(a,b,c) { var t=a+b+c; return; } exit f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_fun_return_value() {
    let code = "fun f(a,b,c) { var t=a+b+c; return t; } exit f(1,2,4);";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 7);
}

