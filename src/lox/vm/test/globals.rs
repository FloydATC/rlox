

use super::compile_and_execute;


// Global vars
#[test]
fn vm_global_undefined() {
    let code = "var a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_global_defined() {
    let code = "var a=1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_global_0() {
    let code = "var a; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_global_1() {
    let code = "var a=1; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_global_2() {
    let code = "var a=1; a=a*2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_global_3() {
    let code = "var a=1; var b=4; a=a*2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_global_4() {
    let code = "var a=1; var b=4; a=a*2; exit b;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_global_5() {
    let code = "var a=1; var b=4; b=b*2; exit b;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

#[test]
#[should_panic]
fn vm_global_redefine() {
    let code = "var a=1; var a=2;";
    let _res = compile_and_execute(code);
}

