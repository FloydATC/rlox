

use super::test;


#[test]
fn const_global_uninitialized_ok() {
    let code = "const a;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn const_global_initialized_ok() {
    let code = "const a=123;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn const_global_reinitialized_bad() {
    let code = "const a=123; a=234;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn const_global_redeclared_bad() {
    let code = "const a=123; const a=234;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn const_global_self_initialize_bad() {
    let code = "const a=a;";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}


#[test]
fn const_local_uninitialized_ok() {
    let code = "fun f() { const a; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn const_local_initialized_ok() {
    let code = "fun f() { const a=123; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn const_local_reinitialized_bad() {
    let code = "fun f() { const a=123; a=234; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn const_local_redeclared_bad() {
    let code = "fun f() { const a=123; const a=234; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}

#[test]
fn const_local_self_initialize_bad() {
    let code = "fun f() { const a=a; }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}


#[test]
fn const_upvalue_uninitialized_ok() {
    let code = "fun f() { const a; fun g() { exit a; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn const_upvalue_pre_initialized_ok() {
    let code = "fun f() { const a=123; fun g() { exit a; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_ok(), true);
}

#[test]
fn const_upvalue_late_inner_reassign_bad() {
    let code = "fun f() { const a; fun g() { a=123; } }";
    println!("code={}", code);
    let res = test(code);
    assert_eq!(res.is_err(), true);
}
