

use super::compile_and_execute;


// Local vars shadowing global ones

#[test]
fn vm_local_shadow_global_1() {
    let code = "var a=1; var b=2; { var a=3; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_global_2() {
    let code = "var a=1; var b=2; { var b=4; exit b; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_local_shadow_global_3() {
    let code = "var a=1; var b=2; { var a=3; var b=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_global_4() {
    let code = "var a=1; var b=2; { var a=3; var b=4; exit b; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

// Re-use of local var names in different scopes
#[test]
fn vm_reuse_local_1() {
    let code = "{ var a=1; } { var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_reuse_local_2() {
    let code = "{ var a=1; } { var b=1; var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_reuse_local_3() {
    let code = "{ var a=1; var b=1; } { var b=2; var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_reuse_local_4() {
    let code = "{ var a=1; var b=1; } { var a=4; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

// Disallow redefine in same scope
#[test]
#[should_panic]
fn vm_redefine_local() {
    let code = "{ var a=1; var a=2; }";
    let _res = compile_and_execute(code);
}

// Disallow self definition
#[test]
#[should_panic]
fn vm_local_self_define() {
    let code = "{ var a=a; }";
    let _res = compile_and_execute(code);
}


// Local var inaccessible in global scope
#[test]
#[should_panic]
fn vm_no_local_in_global_1() {
    let code = "{ var a=1; a=a*2; } exit a;";
    let _res = compile_and_execute(code);
}

// Local var inaccessible in different scope
#[test]
#[should_panic]
fn vm_no_local_in_other_local_1() {
    let code = "{ var a=1; a=a*2; } { exit a; }";
    let _res = compile_and_execute(code);
}


// Local vars shadowing local ones
#[test]
fn vm_local_shadow_local_1() {
    let code = "{ var a=1; var b=2; { var a=3; exit a; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_local_2() {
    let code = "{ var a=1; var b=2; { var b=4; exit b; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

#[test]
fn vm_local_shadow_local_3() {
    let code = "{ var a=1; var b=2; { var a=3; var b=4; exit a; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_local_shadow_local_4() {
    let code = "{ var a=1; var b=2; { var a=3; var b=4; exit b; } }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 4);
}

