

use super::compile_and_execute;


// boolean &&
#[test]
fn vm_boolean_false_and_false() {
    let code = "if (false && false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_false_and_true() {
    let code = "if (false && true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_true_and_false() {
    let code = "if (true && false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_true_and_true() {
    let code = "if (true && true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// boolean ||
#[test]
fn vm_boolean_false_or_false() {
    let code = "if (false || false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_false_or_true() {
    let code = "if (false || true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_true_or_false() {
    let code = "if (true || false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_true_or_true() {
    let code = "if (true || true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Boolean ! (negate)

#[test]
fn vm_boolean_not_true() {
    let code = "if (!true == false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_not_false() {
    let code = "if (!false == true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Null is not equal to true or false, but null is falsey and !null is truey

#[test]
fn vm_boolean_null_not_same_as_false() {
    let code = "if (null != false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_null_not_same_as_true() {
    let code = "if (null != true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_null_not_same_as_not_null() {
    let code = "if (null != !null) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_null_is_falsey() {
    let code = "if (null) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_not_null_is_truey() {
    let code = "if (!null) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_null_and_true() {
    let code = "if (null && true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_null_and_false() {
    let code = "if (null && false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}

#[test]
fn vm_boolean_null_or_true() {
    let code = "if (null || true) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_boolean_null_or_false() {
    let code = "if (null || false) { exit 1; } else { exit 2; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 2);
}


// Equality
#[test]
fn vm_equal_null() {
    let code = "if (null == null) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_null() {
    let code = "if (null != null) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_true() {
    let code = "if (true == true) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_true() {
    let code = "if (true != true) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_false() {
    let code = "if (false == false) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_false() {
    let code = "if (false != false) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_number() {
    let code = "if (123 == 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_number() {
    let code = "if (123 != 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_string() {
    let code = "if ('foo' == 'foo') { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_not_equal_string() {
    let code = "if ('foo' != 'foo') { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_equal_difftypes() {
    let code = "if ('123' == 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_not_equal_difftypes() {
    let code = "if ('123' != 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Single or double quotes behave the same
#[test]
fn vm_quotes() {
    let code = "if ('123\r\n' == \"123\r\n\") { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Less
#[test]
fn vm_less_1() {
    let code = "if (123 < 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_less_2() {
    let code = "if (123 < 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_less_3() {
    let code = "if (234 < 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_less_4() {
    let code = "if (234 < 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

// Less or equal
#[test]
fn vm_less_or_equal_1() {
    let code = "if (123 <= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_less_or_equal_2() {
    let code = "if (123 <= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_less_or_equal_3() {
    let code = "if (234 <= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_less_or_equal_4() {
    let code = "if (234 <= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

// Greater
#[test]
fn vm_greater_1() {
    let code = "if (123 > 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_greater_2() {
    let code = "if (123 > 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_greater_3() {
    let code = "if (234 > 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_greater_4() {
    let code = "if (234 > 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

// Greater or equal
#[test]
fn vm_greater_or_equal_1() {
    let code = "if (123 >= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_greater_or_equal_2() {
    let code = "if (123 >= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 0);
}

#[test]
fn vm_greater_or_equal_3() {
    let code = "if (234 >= 123) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}

#[test]
fn vm_greater_or_equal_4() {
    let code = "if (234 >= 234) { exit 1; }";
    let res = compile_and_execute(code);
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap(), 1);
}
