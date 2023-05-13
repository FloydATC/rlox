

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


// Array manipulation with arithmetics

#[test]
fn add_array_is_concatenate() {
    let code = "var a=[1,2,3]; var b=[4,5,6]; var v=a+b; exit v == [1,2,3,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_array_is_concatenate_a_unchanged() {
    let code = "var a=[1,2,3]; var b=[4,5,6]; var v=a+b; exit a == [1,2,3];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_array_is_concatenate_b_unchanged() {
    let code = "var a=[1,2,3]; var b=[4,5,6]; var v=a+b; exit b == [4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_number_to_array_works_as_push() {
    let code = "var a=[1,2,3]; var b=a+4; b=b+5; b=b+6; exit b == [1,2,3,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_numbers_to_array_works() {
    let code = "var a=[1,2,3]; var b=a+4+5+6; exit b == [1,2,3,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_grouped_numbers_to_array_works() {
    let code = "var a=[1,2,3]; var b=a+(4+5+6); exit b == [1,2,3,15];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_numbers_to_array_original_is_unchanged() {
    let code = "var a=[1,2,3]; var b=a+4+5+6; exit a == [1,2,3];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_array_to_single_number_works_as_unshift() {
    let code = "var a=[4,5,6]; var b=1+a; exit b == [1,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_array_to_multiple_numbers_not_recommended() {
    let code = "var a=[4,5,6]; var b=1+2+3+a; exit b == [6,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn add_array_to_multiple_numbers_original_is_unchanged() {
    let code = "var a=[4,5,6]; var b=1+2+3+a; exit a == [4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subtract_number_from_array_works_as_pop() {
    let code = "var a=[1,2,3,4,5,6]; var b=a-2; exit b == [1,2,3,4];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subtract_number_from_array_original_is_unchanged() {
    let code = "var a=[1,2,3,4,5,6]; var b=a-2; exit a == [1,2,3,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subtract_array_from_number_works_as_shift() {
    let code = "var a=[1,2,3,4,5,6]; var b=2-a; exit b == [3,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn subtract_array_from_number_original_is_unchanged() {
    let code = "var a=[1,2,3,4,5,6]; var b=2-a; exit a == [1,2,3,4,5,6];";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}
