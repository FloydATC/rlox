

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
    let code = "exit [1,2,nan] != [1,2,nan];"; // NaN makes them not equal
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn three_is_same() {
    let code = "exit [1,2,3] is not [1,2,3];"; // Both are literals but they are not the same
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
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

#[test]
fn copy_equal() {
    let code = "var v1=[1,2,3]; var v2=v1; exit v1 == v2;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn copy_same() {
    let code = "var v1=[1,2,3]; var v2=v1; exit v1 is v2;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn copy_then_reassign_equal() {
    let code = "var v1=[1,2,3]; var v2=v1; v1=[1,2,3]; exit v1 == v2;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn copy_then_reassign_is_not_same() {
    let code = "var v1=[1,2,3]; var v2=v1; v1=[1,2,3]; exit v1 is not v2;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Subscripting

#[test]
fn subscript_one() {
    let code = "var v1=[1,2,3]; var v2=v1[0]; exit v2 == 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_two() {
    let code = "var v1=[1,2,3]; var v2=v1[1]; exit v2 == 2;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_three() {
    let code = "var v1=[1,2,3]; var v2=v1[2]; exit v2 == 3;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_bad_under() {
    let code = "var v1=[1,2,3]; var v2=v1[-1];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    // TODO: Make error msg not leak internals
    assert_eq!(error.get_message(), "Bad subscript '-1' into value 'Obj::Array([1, 2, 3])'");
}

#[test]
fn subscript_bad_over() {
    let code = "var v1=[1,2,3]; var v2=v1[3];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_err(), true);
    let error = res.unwrap_err();
    // TODO: Make error msg not leak internals
    assert_eq!(error.get_message(), "Bad subscript '3' into value 'Obj::Array([1, 2, 3])'"); 
}

#[test]
fn subscript_array() {
    let code = "var v1=[1,2,3,4,5]; var v2=v1[1,3]; exit v2 == [2,4];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_full_is_equal() {
    let code = "var v1=[1,2,3,4,5]; var v2=v1[0,1,2,3,4]; exit v2 == v1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_full_is_not_same() {
    let code = "var v1=[1,2,3,4,5]; var v2=v1[0,1,2,3,4]; exit v2 is not v1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_empty_is_copy() {
    let code = "var v1=[1,2,3,4,5]; var v2=v1[]; exit v2 == [1,2,3,4,5];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_copy_is_equal() {
    let code = "var v1=[1,2,3,4,5]; var v2=v1[]; exit v2 == v1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subscript_copy_is_not_same() {
    let code = "var v1=[1,2,3,4,5]; var v2=v1[]; exit v2 is not v1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

