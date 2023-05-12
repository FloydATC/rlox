

use super::compile_and_execute;


#[test]
fn empty_equal() {
    let code = "exit [] == [];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn three_equal() {
    let code = "exit [1,2,3] == [1,2,3];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn nan_not_equal() {
    let code = "exit [1,2,nan] == [1,2,nan];"; // NaN makes this false
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn three_is_same() {
    let code = "exit [1,2,3] is [1,2,3];"; // Both are literals but they are not the same
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn assign_to_global() {
    let code = "var v=[1,2,3]; exit 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn global_three_eq_self() {
    let code = "var v=[1,2,3]; exit v == v;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn global_three_is_self() {
    let code = "var v=[1,2,3]; exit v is v;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn global_nan_is_self() {
    let code = "var v=[1,2,nan]; exit v is v;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

