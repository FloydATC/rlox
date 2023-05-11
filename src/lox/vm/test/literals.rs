

use super::compile_and_execute;


#[test]
fn vm_exit_null_is_zero() {
    let code = "exit null;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_exit_true_is_one() {
    let code = "exit true;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_exit_false_is_zero() {
    let code = "exit false;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_exit_nan_is_magic() {
    let code = "exit nan;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), i32::MAX);
}

#[test]
fn vm_exit_inf_is_magic() {
    let code = "exit inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), i32::MAX);
}

#[test]
fn vm_exit_infn_is_magic() {
    let code = "exit -inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), i32::MIN);
}

#[test]
fn vm_nan_not_equal_to_nan() {
    let code = "exit nan == nan;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_nan_is_nan() {
    let code = "exit nan is nan;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_inf_is_inf() {
    let code = "exit inf is inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_infn_is_infn() {
    let code = "exit -inf is -inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_inf_is_infn() {
    let code = "exit inf is -inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_inf_is_nan() {
    let code = "exit inf is nan;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_infn_is_nan() {
    let code = "exit -inf is nan;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_null_is_null() {
    let code = "exit null is null;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_true_is_true() {
    let code = "exit true is true;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_false_is_false() {
    let code = "exit false is false;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_true_is_notfalse() {
    let code = "exit true is !false;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_false_is_not_true() {
    let code = "exit false is !true;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_true_is_neg_false() {
    let code = "exit true is -false;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_false_is_neg_true() {
    let code = "exit false is -true;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_neg_inf_isnt_not_inf() {
    let code = "exit -inf is !inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_zero_is_not_one() {
    let code = "exit 0 is 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_zero_is_zero() {
    let code = "exit 0 is 0;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_one_is_one() {
    let code = "exit 1 is 1;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}


#[test]
fn vm_true_is_not_false() {
    let code = "exit true is not false;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_true_is_not_true() {
    let code = "exit true is not true;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_false_is_not_false() {
    let code = "exit false is not false;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_false_is_not_false_parens_ok() {
    let code = "exit (false) is not false;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_false_neginf_is_not_inf() {
    let code = "exit (-inf) is not inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_inf_minus_one_is_inf() {
    let code = "exit inf - 1 is inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_inf_minus_one_is_not_inf() {
    let code = "exit inf - 1 is not inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_neginf_minus_one_is_neginf() {
    let code = "exit -inf - 1 is -inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_neginf_minus_one_is_not_neginf() {
    let code = "exit -inf - 1 is not -inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_false_neginf_is_not_neginf() {
    let code = "exit (-inf) is not -inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_neginf_plus_inf_is_nan() {
    let code = "exit -inf + inf is nan;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}


// Practical application
#[test]
fn vm_division_by_zero() {
    let code = "exit (10 / 0) is inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_neg_division_by_zero() {
    let code = "exit (-10 / 0) is -inf;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_division_by_null() {
    let code = "exit (10 / null) is nan;";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}
