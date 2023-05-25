

use super::test;


#[test]
fn var_global_uninitialized_ok() {
    let code = "var a;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_global_initialized_ok() {
    let code = "var a=123;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_global_reinitialized_ok() {
    let code = "var a=123; a=234;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_global_redeclared_bad() {
    let code = "var a=123; var a=234;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn var_global_self_initialize_bad() {
    let code = "var a=a;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}


#[test]
fn var_local_uninitialized_ok() {
    let code = "fun f() { var a; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_local_initialized_ok() {
    let code = "fun f() { var a=123; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_local_reinitialized_ok() {
    let code = "fun f() { var a=123; a=234; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_local_redeclared_bad() {
    let code = "fun f() { var a=123; var a=234; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn var_local_self_initialize_bad() {
    let code = "fun f() { var a=a; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}


#[test]
fn var_upvalue_uninitialized_ok() {
    let code = "fun f() { var a; fun g() { exit a; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_upvalue_pre_initialized_ok() {
    let code = "fun f() { var a=123; fun g() { exit a; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_upvalue_late_initialized_ok() {
    let code = "fun f() { var a; fun g() { a=123; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_upvalue_pre_initialized_then_reinitialize_ok() {
    let code = "fun f() { var a=123; fun g() { a=234; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn var_local_shadow_ok() {
    let code = "fun f() { var a=123; fun g() { var a=234; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}
