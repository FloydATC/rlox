

use super::compile_and_execute;

// 'if' statement

#[test]
fn vm_if_true() {
    let code = "if (true) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_if_false() {
    let code = "if (false) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_if_true_else() {
    let code = "if (true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_if_false_else() {
    let code = "if (false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

// 'then' and 'else' blocks are different scopes if braced
#[test]
fn vm_if_scopes_1() {
    let code = "if (true) { var a=1; exit a; } else { var a=2; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
#[should_panic]
fn vm_if_scopes_2() {
    let code = "if (true) { var a=1; exit a; } else { exit a; }";
    let _res = compile_and_execute(code);
}

#[test]
fn vm_if_scopes_3() {
    let code = "if (false) { var a=1; exit a; } else { var a=2; exit a; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
#[should_panic]
fn vm_if_scopes_4() {
    let code = "if (false) { var a=1; exit a; } else { exit a; }";
    let _res = compile_and_execute(code);
}

// 'then' and 'else' blocks are same scope if not braced
#[test]
fn vm_if_noscopes_1() {
    let code = "var a; if (true) a=1; else a=2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_if_noscopes_2() {
    let code = "var a; if (false) a=1; else a=2; exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
#[should_panic]
fn vm_if_noscopes_3() {
    let code = "var a; if (true) var a=1; else a=2; exit a;";
    let _res = compile_and_execute(code);
}

#[test]
#[should_panic]
fn vm_if_noscopes_4() {
    let code = "var a; if (true) a=1; else var a=2; exit a;";
    let _res = compile_and_execute(code);
}

