

use super::compile_and_execute;


#[test]
fn for_loop_no_conditional_is_true() {
    let code = "for (;;) { exit 1; } exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_true_conditional() {
    let code = "for (; true;) { exit 1; } exit 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_false_conditional() {
    let code = "for (; false;) { exit 0; } exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_local_var() {
    let code = "for (var i=1;;) { exit i; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_local_var_shadows_global() {
    let code = "var i=0; for (var i=1;;) { exit i; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_using_global_var() {
    let code = "var i=0; for (i=1;;) { exit i; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn for_loop_count_up() {
    let code = "var i; for (i=0; i<5; i=i+1) {} exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 5);
}

#[test]
fn for_loop_count_down() {
    let code = "var i; for (i=5; i>0; i=i-1) {} exit i;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

