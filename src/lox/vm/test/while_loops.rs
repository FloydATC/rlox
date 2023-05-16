

use super::compile_and_execute;


// 'while' loops with 'break'/'continue'
#[test]
fn vm_while_naked() {
    let code = "var i=0; while (i<5) i=i+1; exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_scoped() {
    let code = "var i=0; while (i<5) { i=i+1; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_scoped_var_before() {
    let code = "var i=0; while (i<5) { var j=10; i=i+1; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_scoped_var_after() {
    let code = "var i=0; while (i<5) { i=i+1; var j=10; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn vm_while_immediate_break() {
    let code = "var i=0; while (i<5) { i=i+1; break; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_nested_break() {
    let code = "var i=0; while (i<5) { i=i+1; { break; } } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_if_break() {
    let code = "var i=0; while (i<5) { i=i+1; if (i==3) break; } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_while_if_nested_break() {
    let code = "var i=0; while (i<5) { i=i+1; if (i==3) { break; } } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_while_nested_if_nested_break() {
    let code = "var i=0; while (i<5) { i=i+1; { if (i==3) { break; } } } exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 3);
}

#[test]
fn vm_while_if_continue() {
    let code = "var a=0; var i=0; while (i<10) { i=i+1; if (i>4) continue; a=a+2; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

#[test]
fn vm_while_if_nested_continue() {
    let code = "var a=0; var i=0; while (i<10) { i=i+1; if (i>4) { continue; } a=a+2; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

#[test]
fn vm_while_nested_if_nested_continue() {
    let code = "var a=0; var i=0; while (i<10) { i=i+1; { if (i>4) { continue; } } a=a+2; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 8);
}

#[test]
fn vm_while_not_true() {
    let code = "while not (true) { exit 0; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_not_false() {
    let code = "while not (false) { exit 1; } exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_var_declaration() {
    let code = "while (var a=true) { exit a; } exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_var_declaration_is_local() {
    let code = "var a=1; while (var a=false) {} exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_while_var_declaration_countdown() {
    let code = "var a=5; while (var b=a) { a=a-1; } exit a;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
#[should_panic]
fn vm_while_var_declaration_scope_ends() {
    let code = "var a=5; while (var b=a) { a=a-1; } exit b;";
    let _res = compile_and_execute(code);
}

#[test]
fn vm_while_var_declaration_uninitialized_is_false() {
    let code = "while (var a) { exit 0; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
#[should_panic]
fn vm_while_var_cannot_self_initialize() {
    let code = "while (var a=a+1) { exit 0; } exit 1;";
    let _res = compile_and_execute(code);
}

